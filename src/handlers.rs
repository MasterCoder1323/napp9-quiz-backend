use libc::c_char;
use serde::Deserialize;
use crate::{
    ffi_util::{read_json_from_buffer, write_str_to_buffer},
    users::*,
    state::APP_STATE,
};

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct UserRequest {
    token: String,
}

pub async fn root(buffer: *mut c_char) {
    write_str_to_buffer(buffer, "New Apps 9 Lean Server API <br> <a href='/user-list'>User List</a>");
}

pub async fn user_list(buffer: *mut c_char) {
    let pool = &APP_STATE.get().expect("AppState not initialized").db_pool;
    let response = match get_user_list(pool).await {
        Ok(users) => serde_json::to_string(&users).unwrap_or_else(|_| "[]".into()),
        Err(e) => format!("error: {}", e),
    };
    write_str_to_buffer(buffer, &response);
}

pub async fn signup(buffer: *mut c_char) {
    let input: SignupRequest = match read_json_from_buffer(buffer) {
        Ok(val) => val,
        Err(e) => {
            write_str_to_buffer(buffer, e);
            return;
        }
    };

    let pool = &APP_STATE.get().expect("AppState not initialized").db_pool;
    match add_user(pool, &input.username, &input.password).await {
        Ok(token) => write_str_to_buffer(buffer, &token),
        Err(e) => write_str_to_buffer(buffer, &format!("error: {}", e)),
    }
}

pub async fn login(buffer: *mut c_char) {
    let input: SignupRequest = match read_json_from_buffer(buffer) {
        Ok(val) => val,
        Err(e) => {
            write_str_to_buffer(buffer, e);
            return;
        }
    };

    let pool = &APP_STATE.get().expect("AppState not initialized").db_pool;
    match login_user(pool, &input.username, &input.password).await {
        Ok(Some(token)) => write_str_to_buffer(buffer, &token),
        Ok(None) => write_str_to_buffer(buffer, "invalid credentials"),
        Err(e) => write_str_to_buffer(buffer, &format!("error: {}", e)),
    }
}

pub async fn get_user(buffer: *mut c_char) {
    let input: UserRequest = match read_json_from_buffer(buffer) {
        Ok(val) => val,
        Err(e) => {
            write_str_to_buffer(buffer, e);
            return;
        }
    };

    let pool = &APP_STATE.get().expect("AppState not initialized").db_pool;
    match get_user_from_token(pool, &input.token).await {
        Ok(Some(user)) => write_str_to_buffer(buffer, &user.sterilise()),
        Ok(None) => write_str_to_buffer(buffer, "not found"),
        Err(e) => write_str_to_buffer(buffer, &format!("error: {}", e)),
    }
}

pub async fn init(buffer: *mut c_char) {
    match crate::state::init_app_state().await {
        Ok(_) => write_str_to_buffer(buffer, "App State Initialised"),
        Err(e) => write_str_to_buffer(buffer, &format!("error: {}", e)),
    }
}
