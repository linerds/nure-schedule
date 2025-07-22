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

    pub(crate) async fn insert(Self { id, name }: &Self, db: &Database) -> sqlx::Result<()> {
        sqlx::query_as!(
            Self,
            "INSERT OR REPLACE INTO Auditoriums VALUES (?, ?)",
            id,
            name
        )
        .execute(&db.0)
        .await?;
        Ok(())
    }
}
