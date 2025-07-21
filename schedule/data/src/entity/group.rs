use crate::database::Database;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

impl Group {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM Groups WHERE id = ?", id)
            .fetch_optional(&db.0)
            .await
    }

    pub async fn fetch_by_event(db: &Database, id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "SELECT id, name FROM Groups
            JOIN EventGroups ON group_id = id
            WHERE event_id = ?",
            id
        )
        .fetch_all(&db.0)
        .await
    }
}
