use std::{collections::HashSet, path::Path, time::Duration};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

struct Data(SqlitePool);

impl Data {
    /// Initialize or open an [`SqlitePool`] at the given `file_path`.
    pub async fn new(file_path: impl AsRef<Path>) -> sqlx::Result<Self> {
        let connection = SqliteConnectOptions::new()
            .filename(file_path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .max_lifetime(None)
            .idle_timeout(Duration::from_secs(60))
            .connect_with(connection)
            .await?;

        sqlx::query(include_str!("../schema.sql"))
            .execute(&pool)
            .await?;

        Ok(Self(pool))
    }

    pub async fn fetch_group(&self, id: i64) -> sqlx::Result<Option<Group>> {
        sqlx::query_as!(Group, "SELECT * FROM Groups WHERE id = ?", id)
            .fetch_optional(&self.0)
            .await
    }
    pub async fn fetch_teacher(&self, id: i64) -> sqlx::Result<Option<Teacher>> {
        sqlx::query_as!(Teacher, "SELECT * FROM Teachers WHERE id = ?", id)
            .fetch_optional(&self.0)
            .await
    }
    pub async fn fetch_subject(&self, id: i64) -> sqlx::Result<Option<Subject>> {
        sqlx::query_as!(Subject, "SELECT * FROM Subjects WHERE id = ?", id)
            .fetch_optional(&self.0)
            .await
    }
    pub async fn fetch_auditorium(&self, id: i64) -> sqlx::Result<Option<Auditorium>> {
        sqlx::query_as!(Auditorium, "SELECT * FROM Auditoriums WHERE id = ?", id)
            .fetch_optional(&self.0)
            .await
    }

    pub async fn fetch_event(&self, id: i64) -> sqlx::Result<Option<Event>> {
        let groups = sqlx::query_as!(
            Group,
            "SELECT id, name FROM Groups
            JOIN EventGroups ON group_id = id
            WHERE event_id = ?",
            id
        )
        .fetch_all(&self.0)
        .await?;

        let teachers = sqlx::query_as!(
            Teacher,
            "SELECT id, name FROM Teachers
            JOIN EventTeachers ON teacher_id = id
            WHERE event_id = ?",
            id
        )
        .fetch_all(&self.0)
        .await?;

        Ok(sqlx::query!(
            "SELECT 
              kind, 
              count, 
              subject_id, 
              auditorium_id, 
              starts_at, 
              s.name AS subject_name, 
              s.abbr AS subject_abbr, 
              a.name AS auditorium_name 
            FROM Events e 
              JOIN Subjects s ON s.id = e.subject_id 
              JOIN Auditoriums a ON a.id = e.auditorium_id 
            WHERE e.id = ?",
            id
        )
        .fetch_optional(&self.0)
        .await?
        .map(|e| Event {
            id,
            kind: e.kind.into(),
            count: e.count as u8, // TODO: sketchy
            subject: Subject {
                id: e.subject_id,
                abbr: e.subject_abbr,
                name: e.subject_name,
            },
            auditorium: Auditorium {
                id: e.auditorium_id,
                name: e.auditorium_name,
            },
            starts_at: e.starts_at,
            groups,
            teachers,
        }))
    }

    pub async fn fetch_events(
        &self,
        include: HashSet<Filter>,
        exclude: HashSet<Filter>,
    ) -> sqlx::Result<Vec<i64>> {
        let includes: Vec<String> = include
            .iter()
            .filter(|x| !exclude.contains(x))
            .filter_map(Filter::build_query)
            .collect();

        let excludes: Vec<String> = exclude
            .iter()
            .filter(|x| !include.contains(x))
            .filter_map(Filter::build_query)
            .collect();

        if includes.is_empty() {
            return Ok(vec![]);
        }

        let mut query = join(includes, " UNION ");
        if !excludes.is_empty() {
            query.push_str(" EXCEPT ");
            query.push_str(&join(excludes, " EXCEPT "));
        }

        #[cfg(test)] // TODO: consider normal logging
        dbg!(&query);

        sqlx::query_scalar(&query).fetch_all(&self.0).await
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Filter {
    groups: Vec<i64>,
    teachers: Vec<i64>,
    subjects: Vec<i64>,
    auditoriums: Vec<i64>,
    kinds: Vec<EventKind>,
}

fn join<I, T>(iter: I, sep: &str) -> String
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    use std::fmt::Write;

    let mut iter = iter.into_iter();
    let first = iter.next().map(|x| x.to_string()).unwrap_or_default();

    iter.fold(first, |mut acc, s| {
        write!(acc, "{sep}{s}").expect("failed to join strings with write! macro");
        acc
    })
}

impl Filter {
    /// Should construct something similar to this:
    /// ```sql
    /// SELECT e.id FROM Events e
    ///     JOIN EventGroups eg ON eg.event_id = e.id
    ///     JOIN EventTeachers et ON et.event_id = e.id
    /// WHERE
    ///         e.auditorium_id = 1
    ///     AND e.subject_id IN (1, 2)
    ///     AND e.kind IN (1, 2, 3, 4)
    ///     AND eg.group_id IN (1, 2)
    ///     AND et.teacher_id IN (1, 2)
    /// GROUP BY e.id HAVING
    ///         COUNT(DISTINCT eg.group_id) = 2
    ///     AND COUNT(DISTINCT et.teacher_id) = 2
    /// ```
    fn build_query(&self) -> Option<String> {
        let Self {
            groups,
            teachers,
            subjects,
            auditoriums,
            kinds,
        } = self;

        // TODO: should that be enforced by the type system?
        if groups.is_empty()
            && teachers.is_empty()
            && subjects.is_empty()
            && auditoriums.is_empty()
            && kinds.is_empty()
        {
            return None;
        }

        let mut query = String::from(" SELECT e.id FROM Events e ");
        let mut filters = Vec::new();
        let mut counts = Vec::new();

        if let [id] = subjects[..] {
            filters.push(format!("e.subject_id = {id}"));
        } else if subjects.len() >= 2 {
            filters.push(format!("e.subject_id IN ({})", join(subjects, ", ")));
        }

        if let [id] = auditoriums[..] {
            filters.push(format!("e.auditorium_id = {id}"));
        } else if auditoriums.len() >= 2 {
            filters.push(format!("e.auditorium_id IN ({})", join(auditoriums, ", ")));
        }

        if let [id] = kinds[..] {
            filters.push(format!("e.kind = {id}"));
        } else if kinds.len() >= 2 {
            filters.push(format!("e.kind IN ({})", join(kinds, ", ")));
        }

        if !groups.is_empty() {
            query.push_str(" JOIN EventGroups eg ON eg.event_id = e.id ");
            if let [id] = groups[..] {
                filters.push(format!("eg.group_id = {id}"));
            } else {
                filters.push(format!("eg.group_id IN ({})", join(groups, ", ")));
                counts.push(format!("COUNT(DISTINCT eg.group_id) = {}", groups.len()));
            }
        }
        if !teachers.is_empty() {
            query.push_str(" JOIN EventTeachers et ON et.event_id = e.id ");
            if let [id] = teachers[..] {
                filters.push(format!("et.teacher_id = {id}"));
            } else {
                filters.push(format!("et.teacher_id IN ({})", join(teachers, ", ")));
                counts.push(format!(
                    "COUNT(DISTINCT et.teacher_id) = {}",
                    teachers.len()
                ));
            }
        }

        // must not be empty, checks are at the top of the function
        query.push_str(" WHERE ");
        query.push_str(&join(filters, " AND "));

        if counts.is_empty() {
            return Some(query);
        }

        query.push_str(" GROUP BY e.id HAVING ");
        query.push_str(&join(&counts, " AND "));

        Some(query)
    }
}

#[derive(Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct Teacher {
    pub id: i64,
    pub name: String,
    // pub short_name: String,
}

#[derive(Debug)]
pub struct Subject {
    pub id: i64,
    pub abbr: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Auditorium {
    pub id: i64,
    pub name: String,
    // pub floor: i64,
    // pub has_power: i64,
    // pub building: String,
}

#[derive(Debug)]
pub struct Event {
    pub id: i64,
    pub kind: EventKind,
    pub count: u8,
    pub subject: Subject,
    pub auditorium: Auditorium,
    pub starts_at: i64,
    // pub ends_at: i64,
    pub groups: Vec<Group>,
    pub teachers: Vec<Teacher>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    Lecture,
    PracticalWork,
    LaboratoryWork,
    Consultation,
    /// Залік
    FinalTest,
    Exam,
    CourseWork,
    Unknown,
}

impl std::fmt::Display for EventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8) // TODO: sketchy
    }
}

impl From<i64> for EventKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Lecture,
            1 => Self::PracticalWork,
            2 => Self::LaboratoryWork,
            3 => Self::Consultation,
            4 => Self::FinalTest, // Залік
            5 => Self::Exam,
            6 => Self::CourseWork,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)] // NOTE: comment this line before running `cargo sqlx prepare`
mod tests {
    use super::*;

    use sqlx::{ConnectOptions, sqlite::SqliteConnectOptions};

    #[sqlx::test]
    async fn new() -> sqlx::Result<()> {
        Data::new("/tmp/test_db.sqlite3").await?;
        Ok(())
    }

    #[sqlx::test]
    async fn fetch_events() -> sqlx::Result<()> {
        let db = Data::new("/tmp/test_db.sqlite3").await?;
        let include = HashSet::from([
            Filter {
                groups: vec![1, 2, 3],
                teachers: vec![1, 2, 3],
                subjects: vec![1, 2, 3],
                auditoriums: vec![1, 2, 3],
                kinds: vec![
                    EventKind::Lecture,
                    EventKind::PracticalWork,
                    EventKind::LaboratoryWork,
                ],
            },
            Filter {
                groups: vec![],
                teachers: vec![1, 2, 3],
                subjects: vec![1],
                auditoriums: vec![1],
                kinds: vec![],
            },
            Filter {
                groups: vec![1],
                teachers: vec![],
                subjects: vec![],
                auditoriums: vec![1],
                kinds: vec![],
            },
            Filter {
                groups: vec![42],
                teachers: vec![],
                subjects: vec![],
                auditoriums: vec![],
                kinds: vec![],
            },
        ]);

        let exclude = HashSet::from([
            Filter {
                groups: vec![1, 2],
                teachers: vec![],
                subjects: vec![],
                auditoriums: vec![],
                kinds: vec![],
            },
            Filter {
                groups: vec![42],
                teachers: vec![],
                subjects: vec![],
                auditoriums: vec![],
                kinds: vec![],
            },
        ]);

        let events = dbg!(db.fetch_events(include, exclude).await)?;

        Ok(())
    }

    #[sqlx::test]
    async fn schema() -> sqlx::Result<()> {
        Ok(())
        // let mut conn = SqliteConnectOptions::new()
        //     .in_memory(true)
        //     .connect()
        //     .await?;

        // sqlx::query(include_str!("../schema.sql"))
        //     .execute(&mut conn)
        //     .await?;

        // sqlx::query!("INSERT INTO Auditoriums VALUES (1, 'TEST')") //, 2, 3, 'test')")
        //     .execute(&mut conn)
        //     .await?;
        // sqlx::query!("INSERT INTO Subjects VALUES (1, 't', 'test')")
        //     .execute(&mut conn)
        //     .await?;
        // sqlx::query!("INSERT INTO Events VALUES (1, 4, 1, 1, 4, '2025-5-13 11:15:00')")
        //     .execute(&mut conn)
        //     .await?;

        // sqlx::query_as!(Group, "SELECT * FROM Groups LIMIT 1")
        //     .fetch_optional(&mut conn)
        //     .await?;
        // sqlx::query_as!(Teacher, "SELECT * FROM Teachers LIMIT 1")
        //     .fetch_optional(&mut conn)
        //     .await?;

        // sqlx::query_as!(Subject, "SELECT * FROM Subjects LIMIT 1")
        //     .fetch_one(&mut conn)
        //     .await?;
        // sqlx::query_as!(Auditorium, "SELECT * FROM Auditoriums LIMIT 1")
        //     .fetch_one(&mut conn)
        //     .await?;
        // sqlx::query_as!(Event, "SELECT * FROM Events LIMIT 1")
        //     .fetch_one(&mut conn)
        //     .await?;
        // Ok(())
    }
}

// use derive_more::{Deref, Into};

// #[derive(Debug, Deref, Into)]
// pub struct Bool(bool);

// impl From<i64> for Bool {
//     fn from(value: i64) -> Self {
//         Self(value != 0)
//     }
// }

// use chrono::{DateTime, Utc};

// #[derive(Debug, Deref, Into)]
// pub struct UtcTime(DateTime<Utc>);

// impl From<i64> for UtcTime {
//     fn from(seconds: i64) -> Self {
//         Self(DateTime::from_timestamp(seconds, 0).expect("invalid timestamp"))
//     }
// }
