use crate::database::Database;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Auditorium {
    pub id: i64,
    pub name: String,
    // pub floor: i64,
    // pub has_power: i64,
    // pub building: String,
}

impl Auditorium {
    pub async fn fetch(db: &Database, id: i64) -> sqlx::Result<Option<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM Auditoriums WHERE id = ?", id)
            .fetch_optional(&db.0)
            .await
    }
}
