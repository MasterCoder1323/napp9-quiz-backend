use crate::users::{add_user, get_user_from_token, get_user_list, login_user};
use crate::state::APP_STATE;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub token: String,
}

pub fn root() -> String {
    "New Apps 9 Learn Server API".into()
}

pub fn user_list() -> String {
    // Lock the mutex to get access to the DB connection
    let conn = APP_STATE.get().expect("AppState not initialized").db_conn.lock().unwrap();
    match get_user_list(&conn) {
        Ok(users) => serde_json::to_string(&users).unwrap_or_else(|_| "[]".into()),
        Err(e) => format!("error: {}", e),
    }
}

pub fn signup(body: &str) -> String {
    let input: SignupRequest = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(_) => return "invalid json".into(),
    };

    let conn = APP_STATE.get().expect("AppState not initialized").db_conn.lock().unwrap();
    match add_user(&conn, &input.username, &input.password) {
        Ok(token) => token,
        Err(e) => format!("error: {}", e),
    }
}

pub fn login(body: &str) -> String {
    let input: SignupRequest = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(_) => return "invalid json".into(),
    };

    let conn = APP_STATE.get().expect("AppState not initialized").db_conn.lock().unwrap();
    match login_user(&conn, &input.username, &input.password) {
        Ok(Some(token)) => token,
        Ok(None) => "invalid credentials".into(),
        Err(e) => format!("error: {}", e),
    }
}

pub fn get_user(body: &str) -> String {
    let input: UserRequest = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(_) => return "invalid json".into(),
    };

    let conn = APP_STATE.get().expect("AppState not initialized").db_conn.lock().unwrap();
    match get_user_from_token(&conn, &input.token) {
        Ok(Some(user)) => serde_json::to_string(&user).unwrap_or_else(|_| "serialization error".into()),
        Ok(None) => "not found".into(),
        Err(e) => format!("error: {}", e),
    }
}
