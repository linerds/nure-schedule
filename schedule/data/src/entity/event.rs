use std::collections::BTreeSet;

use crate::{Database, Filter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EventKind {
    Lecture = 0,
    PracticalWork = 1,
    LaboratoryWork = 2,
    Consultation = 3,
    /// Залік
    FinalTest = 4,
    Exam = 5,
    CourseWork = 6,
    Unknown = 255,
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
    pub subject_id: i64,
    pub auditorium_id: i64,
    pub starts_at: i64,
    // pub ends_at: i64,
}

impl Event {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        Ok(sqlx::query!("SELECT * FROM Events WHERE id = ?", id)
            .fetch_optional(&db.0)
            .await?
            .map(|e| Self {
                id,
                kind: e.kind.into(),
                count: e.count.try_into().unwrap_or(u8::MAX),
                subject_id: e.subject_id,
                auditorium_id: e.auditorium_id,
                starts_at: e.starts_at,
            }))
    }

    pub(crate) async fn insert(&self, db: &Database) -> sqlx::Result<()> {
        let kind = self.kind as u8;
        sqlx::query_as!(
            Self,
            "INSERT OR REPLACE INTO Events(id, kind, count, subject_id, auditorium_id, starts_at) VALUES (?, ?, ?, ?, ?,?)",
           self.id,
           kind,
           self.count,
           self.subject_id,
           self.auditorium_id,
           self.starts_at,
        )
        .execute(&db.0)
        .await?;
        Ok(())
    }

    // TODO that should either return Vec<Event>, or be moved out to Database impl or elsewhere
    pub async fn fetch_filtered(
        db: &Database,
        mut include: BTreeSet<Filter>,
        mut exclude: BTreeSet<Filter>,
    ) -> sqlx::Result<Vec<i64>> {
        let mut duplicates = Vec::new();

        for filter in &exclude {
            if let Some(f) = include.take(filter) {
                duplicates.push(f);
            }
        }
        if include.is_empty() {
            return Ok(Vec::new());
        }
        for filter in duplicates {
            exclude.remove(&filter);
        }

        let mut query = String::new();

        let mut written = false;
        for filter in include {
            filter.write_query(&mut query, &mut written, "\n\nUNION\n\n");
        }
        for filter in exclude {
            filter.write_query(&mut query, &mut written, "\n\nEXCEPT\n\n");
        }

        #[cfg(test)] // TODO: consider normal logging
        println!("{query}");

        sqlx::query_scalar(&query).fetch_all(&db.0).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn fetch_filtered() -> sqlx::Result<()> {
        let db = Database::in_memory().await?;

        let include = BTreeSet::from([
            Filter::new(),
            Filter::new()
                .groups([1, 2, 3])
                .teachers([1, 2, 3])
                .subjects([1, 2, 3])
                .auditoriums([1, 2, 3])
                .kinds([
                    EventKind::Lecture,
                    EventKind::PracticalWork,
                    EventKind::LaboratoryWork,
                ]),
            Filter::new()
                .teachers([1, 2, 3])
                .subjects([1])
                .auditoriums([1]),
            Filter::new().groups([1]).auditoriums([1]),
            Filter::new().groups([42]),
        ]);

        let exclude = BTreeSet::from([
            Filter::new().groups([42]),
            Filter::new().teachers([1, 2]),
            Filter::default(),
        ]);

        Event::fetch_filtered(&db, include, exclude).await?;

        db.close().await;

        Ok(())
    }
}
