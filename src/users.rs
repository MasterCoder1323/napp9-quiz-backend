use crate::db::{DbConn, User};
use crate::sessions::{create_session, get_session_data};
use crate::hash::{hash_password, verify_password};  // your new hash module
use rusqlite::{params, Result};
use std::error::Error;
use rusqlite::OptionalExtension;

pub fn add_user(conn: &DbConn, username: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let hash = hash_password(password);

    conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        params![username, hash],
    )?;

    // Retrieve user row back
    let mut stmt =
        conn.prepare("SELECT id, username, password_hash FROM users WHERE username = ?")?;
    let user = stmt.query_row(params![username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
        })
    })?;

    let json = user.sterilise();

    let token = create_session(conn, json)?;

    Ok(token)
}

pub fn login_user(
    conn: &DbConn,
    username: &str,
    password: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    if let Some(user) = get_user_from_username(conn, username)? {
        if verify_password(password, &user.password_hash) {
            let json = user.sterilise();
            let token = create_session(conn, json)?;
            Ok(Some(token))
        } else {
            Ok(None) // Incorrect password
        }
    } else {
        Ok(None) // User not found
    }
}

pub fn get_user_from_token(conn: &DbConn, token: &str) -> Result<Option<User>, Box<dyn Error>> {
    if let Some(user_string) = get_session_data(conn, token)? {
        let user = User::from_sterilised(&user_string);
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn get_user_from_username(
    conn: &DbConn,
    username: &str,
) -> Result<Option<User>, Box<dyn Error>> {
    let mut stmt =
        conn.prepare("SELECT id, username, password_hash FROM users WHERE username = ?")?;
    let user = stmt
        .query_row(params![username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password_hash: row.get(2)?,
            })
        })
        .optional()?;

    Ok(user)
}

pub fn get_user_list(conn: &DbConn) -> Result<Vec<String>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT username FROM users")?;
    let usernames_iter = stmt.query_map([], |row| row.get(0))?;

    let mut usernames = Vec::new();
    for username_res in usernames_iter {
        usernames.push(username_res?);
    }
    Ok(usernames)
}
