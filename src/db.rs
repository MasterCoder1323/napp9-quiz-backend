use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn sterilise(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn from_sterilised(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
    }
}

pub type DbPool = SqlitePool;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect("sqlite:main.db")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            data TEXT
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
