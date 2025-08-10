use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

// Instead of a pool, use a direct Connection for sync rusqlite
pub type DbConn = Connection;

pub fn init_db() -> Result<DbConn> {
    let conn = Connection::open("main.db")?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            data TEXT
        );
        CREATE TABLE IF NOT EXISTS connected_users (
            username TEXT PRIMARY KEY,
            points INTEGER CHECK (points >= 0)
        );
        ",
    )?;

    Ok(conn)
}
