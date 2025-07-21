use crate::database::Database;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Teacher {
    pub id: i64,
    pub name: String,
    // pub short_name: String,
}

impl Teacher {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM Teachers WHERE id = ?", id)
            .fetch_optional(&db.0)
            .await
    }

    pub async fn fetch_by_event(db: &Database, id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT id, name FROM Teachers
            JOIN EventTeachers ON teacher_id = id
            WHERE event_id = ?",
            id
        )
        .fetch_all(&db.0)
        .await
    }
}
