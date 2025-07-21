use std::{path::Path, time::Duration};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

#[derive(Clone, Debug)]
pub struct Database(pub(crate) SqlitePool);

impl Database {
    /// Initialize or open an [`SqlitePool`] at the given `file_path`.
    pub async fn new(file_path: impl AsRef<Path>) -> sqlx::Result<Self> {
        let connection = SqliteConnectOptions::new()
            .filename(file_path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);

        Self::from_connection(connection).await
    }

    #[cfg(test)]
    pub async fn in_memory() -> sqlx::Result<Self> {
        let connection = SqliteConnectOptions::new()
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);

        Self::from_connection(connection).await
    }

    async fn from_connection(connection: SqliteConnectOptions) -> sqlx::Result<Self> {
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

    /// Call and await for graceful shutdown of the DB.
    pub async fn close(self) {
        self.0.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn connect_to_file() -> sqlx::Result<()> {
        let file = "/tmp/database_connect_to_file.sqlite3";
        let _ = std::fs::remove_file(file);

        let db = Database::new(file).await?;
        db.close().await;

        Path::new(file)
            .try_exists()
            .unwrap_or_else(|_| panic!("{file} should have been created"));

        let _ = std::fs::remove_file(file);

        Ok(())
    }

    #[sqlx::test]
    async fn initialize_in_memory() -> sqlx::Result<()> {
        Database::in_memory().await?.close().await;
        Ok(())
    }
}
