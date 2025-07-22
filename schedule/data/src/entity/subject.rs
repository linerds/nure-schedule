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

    pub(crate) async fn insert(Self { id, abbr, name }: &Self, db: &Database) -> sqlx::Result<()> {
        sqlx::query_as!(
            Self,
            "INSERT OR REPLACE INTO Subjects(id, abbr, name) VALUES (?, ?, ?)",
            id,
            abbr,
            name
        )
        .execute(&db.0)
        .await?;
        Ok(())
    }
}
