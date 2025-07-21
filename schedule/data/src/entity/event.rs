use std::collections::BTreeSet;

use crate::{Auditorium, Database, Filter, Subject, join};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Event {
    pub id: i64,
    pub kind: EventKind,
    pub count: u8,
    pub subject: Subject,
    pub auditorium: Auditorium,
    pub starts_at: i64,
    // pub ends_at: i64,
    // pub groups: Vec<Group>,
    // pub teachers: Vec<Teacher>,
}

impl Event {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        // let groups = Group::fetch_by_event(db, id).await?;
        // let teachers = Teacher::fetch_by_event(db, id).await?;

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
        .fetch_optional(&db.0)
        .await?
        .map(|e| Self {
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
            // groups,
            // teachers,
        }))
    }

    pub async fn fetch_filtered(
        db: &Database,
        include: BTreeSet<Filter>,
        exclude: BTreeSet<Filter>,
    ) -> sqlx::Result<Vec<i64>> {
        let includes: Vec<String> = include
            .iter()
            .filter(|x| !exclude.contains(x))
            .map(Filter::build_query)
            .collect();

        let excludes: Vec<String> = exclude
            .iter()
            .filter(|x| !include.contains(x))
            .map(Filter::build_query)
            .collect();

        if includes.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = join(includes, " UNION ");
        if !excludes.is_empty() {
            query.push_str(" EXCEPT ");
            query.push_str(&join(excludes, " EXCEPT "));
        }

        #[cfg(test)] // TODO: consider normal logging
        dbg!(&query);

        sqlx::query_scalar(&query).fetch_all(&db.0).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::FilterBuilder;

    #[sqlx::test]
    async fn fetch_filtered() -> sqlx::Result<()> {
        let db = Database::in_memory().await?;

        let include = BTreeSet::from([
            FilterBuilder::new()
                .groups([1, 2, 3])
                .teachers([1, 2, 3])
                .subjects([1, 2, 3])
                .auditoriums([1, 2, 3])
                .kinds([
                    EventKind::Lecture,
                    EventKind::PracticalWork,
                    EventKind::LaboratoryWork,
                ])
                .build()
                .unwrap(),
            FilterBuilder::new()
                .teachers([1, 2, 3])
                .subjects([1])
                .auditoriums([1])
                .build()
                .unwrap(),
            FilterBuilder::new()
                .groups([1])
                .auditoriums([1])
                .build()
                .unwrap(),
            FilterBuilder::new().groups([42]).build().unwrap(),
        ]);

        let exclude = BTreeSet::from([
            FilterBuilder::new().groups([42]).build().unwrap(),
            FilterBuilder::new().teachers([1, 2]).build().unwrap(),
        ]);

        // FIXME: why on Earth the first query ALWAYS fails on Database::in_memory()
        let _ = dbg!(Event::fetch(&db, 1).await);

        dbg!(Event::fetch_filtered(&db, include, exclude).await)?;

        db.close().await;

        Ok(())
    }
}
