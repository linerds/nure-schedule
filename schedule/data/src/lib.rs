mod entity;
mod error;
mod filter;

pub use entity::*;
pub use filter::*;

// // TODO: move to utils crate
// fn join<I, T>(iter: I, sep: &str) -> String
// where
//     I: IntoIterator<Item = T>,
//     T: std::fmt::Display,
// {
//     use std::fmt::Write;

//     let mut iter = iter.into_iter();
//     let first = iter.next().map(|x| x.to_string()).unwrap_or_default();

//     iter.fold(first, |mut acc, s| {
//         write!(acc, "{sep}{s}").expect("failed to join strings with write! macro");
//         acc
//     })
// }

use std::{path::Path, time::Duration};

use sqlx::{
    ConnectOptions, Connection, SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

#[derive(Clone, Debug)]
pub struct Database(pub(crate) SqlitePool);

impl Database {
    /// Initialize or open the database at the given `file_path`.
    pub async fn new(file_path: impl AsRef<Path>) -> sqlx::Result<Self> {
        let opt = SqliteConnectOptions::new()
            .filename(file_path)
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);
        // .optimize_on_close(true, analysis_limit) # TODO?

        // Separate single connection to avoid possible race conditions
        let mut conn = opt.connect().await?;
        sqlx::query(include_str!("../schema.sql"))
            .execute(&mut conn)
            .await?;
        conn.close().await?;

        let pool = SqlitePoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .max_lifetime(None)
            .idle_timeout(Duration::from_secs(60))
            .connect_with(opt)
            .await?;

        Ok(Self(pool))
    }

    #[cfg(test)]
    pub async fn in_memory() -> sqlx::Result<Self> {
        let opt = SqliteConnectOptions::new()
            .in_memory(true)
            .foreign_keys(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .min_connections(1) // when a connection closes the in-memory DB gets dropped
            .max_connections(1) // each connection has a separate DB, so use at most one
            .max_lifetime(None)
            .idle_timeout(None)
            .connect_with(opt)
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
