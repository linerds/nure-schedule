use crate::database::Database;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Subject {
    pub id: i64,
    pub abbr: String,
    pub name: String,
}

impl Subject {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM Subjects WHERE id = ?", id)
            .fetch_optional(&db.0)
            .await
    }
}
