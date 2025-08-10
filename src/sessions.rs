use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result};
use std::error::Error;

fn generate_token() -> String {
    use fastrand::Rng;
    let rng = fastrand::Rng::new();
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    (0..64)
        .map(|_| {
            let idx = rng.usize(0..charset.len());
            charset[idx] as char
        })
        .collect()
}


pub fn create_session(conn: &Connection, data: String) -> Result<String, Box<dyn Error>> {
    let token = generate_token();
    conn.execute(
        "INSERT INTO sessions (token, data) VALUES (?, ?)",
        params![&token, data],
    )?;
    Ok(token)
}

pub fn get_session_data(conn: &Connection, token: &str) -> Result<Option<String>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT data FROM sessions WHERE token = ?")?;
    let data = stmt
        .query_row(params![token], |row| row.get(0))
        .optional()?;
    Ok(data)
}

pub fn delete_session(conn: &Connection, token: &str) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM sessions WHERE token = ?", params![token])?;
    Ok(())
}
