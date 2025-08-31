use crate::db::*;
use std::sync::{Mutex, OnceLock};

pub struct AppState {
    pub db_conn: Mutex<DbConn>, // Wrap connection in Mutex
}

pub static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub fn init_app_state() -> Result<(), Box<dyn std::error::Error>> {
    let db_conn = init_db()?; // returns rusqlite::Connection

    APP_STATE
        .set(AppState {
            db_conn: Mutex::new(db_conn),
        })
        .map_err(|_| "AppState already initialized".into())
}
