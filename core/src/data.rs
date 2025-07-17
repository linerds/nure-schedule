#[derive(Debug)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct Teacher {
    pub id: i64,
    pub short_name: String,
    pub full_name: String,
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
    pub floor: i64,
    pub has_power: i64,
    pub building: String,
}

#[derive(Debug)]
pub struct Event {
    pub id: i64,
    pub kind: i64,
    pub subject_id: i64,
    pub auditorium_id: i64,
    pub count: i64,
    pub starts_at: String,
    pub ends_at: String,
}

#[cfg(test)] // NOTE: comment this line before running `cargo sqlx prepare`
mod tests {
    use super::*;

    use sqlx::{ConnectOptions, sqlite::SqliteConnectOptions};

    #[sqlx::test]
    async fn schema() -> sqlx::Result<()> {
        let mut conn = SqliteConnectOptions::new()
            .in_memory(true)
            .connect()
            .await?;

        sqlx::query(include_str!("../assets/schema.sql"))
            .execute(&mut conn)
            .await?;

        sqlx::query!("INSERT INTO Auditoriums VALUES (1, 'TEST', 2, 3, 'test')")
            .execute(&mut conn)
            .await?;
        sqlx::query_as!(Group, "SELECT * FROM Groups LIMIT 1")
            .fetch_optional(&mut conn)
            .await?;
        sqlx::query_as!(Teacher, "SELECT * FROM Teachers LIMIT 1")
            .fetch_optional(&mut conn)
            .await?;
        sqlx::query_as!(Subject, "SELECT * FROM Subjects LIMIT 1")
            .fetch_optional(&mut conn)
            .await?;
        dbg!(
            sqlx::query_as!(Auditorium, "SELECT * FROM Auditoriums LIMIT 1")
                .fetch_optional(&mut conn)
                .await?
        );
        sqlx::query_as!(Event, "SELECT * FROM Events LIMIT 1")
            .fetch_optional(&mut conn)
            .await?;
        Ok(())
    }
}
