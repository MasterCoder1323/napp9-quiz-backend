use crate::db::{DbPool, User};
use crate::sessions::{create_session, get_session_data};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::query;

pub async fn add_user(
    pool: &DbPool,
    username: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let hash: String = hash(password, DEFAULT_COST)?;

    query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(hash)
        .execute(pool)
        .await?;

    // Get the new user to sterilise it
    let user: User = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    let json: String = user.sterilise();

    let token: String = create_session(pool, json).await?;

    Ok(token)
}

pub async fn login_user(
    pool: &DbPool,
    username: &str,
    password: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Fetch user from DB
    let user: Option<User> = get_user_from_username(pool, username).await?;

    if let Some(user) = user {
        // Compare password to hash
        if verify(password, &user.password_hash)? {
            let json: String = user.sterilise();
            let token: String = create_session(pool, json).await?;
            Ok(Some(token))
        } else {
            Ok(None) // Incorrect password
        }
    } else {
        Ok(None) // User not found
    }
}

pub async fn get_user_from_token(pool: &DbPool, token: &str) -> Result<Option<User>, sqlx::Error> {
    let data: Option<String> = get_session_data(pool, token)
        .await
        .expect("Something Went Wrong");
    if let Some(user_string) = data {
        let user: User = User::from_sterilised(&user_string);
        return Ok(Some(user));
    }
    Ok(None)
}

pub async fn get_user_from_username(
    pool: &DbPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_list(pool: &DbPool) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar("SELECT username FROM users")
        .fetch_all(pool)
        .await
}


