use crate::db::DbPool;
use rand::{distributions::Alphanumeric, Rng};
use sqlx::{query, query_scalar};
use std::error::Error;

fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

pub async fn create_session(pool: &DbPool, data: String) -> Result<String, Box<dyn Error>> {
    let token = generate_token();
    query("INSERT INTO sessions (token, data) VALUES (?, ?)")
        .bind(&token)
        .bind(data)
        .execute(pool)
        .await?;
    Ok(token)
}

pub async fn get_session_data(
    pool: &DbPool,
    token: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let data: Option<String> = query_scalar("SELECT data FROM sessions WHERE token = ?")
        .bind(token)
        .fetch_optional(pool)
        .await?;

    Ok(data)
}

pub async fn delete_session(pool: &DbPool, token: &str) -> Result<(), Box<dyn Error>> {
    query("DELETE FROM sessions WHERE token = ?")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}
