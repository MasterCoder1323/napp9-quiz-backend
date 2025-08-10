use crate::db::*;
use std::sync::OnceLock; // assumes DbConn = rusqlite::Connection

pub struct AppState {
    pub db_conn: DbConn,
}

pub static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub fn init_app_state() -> Result<(), Box<dyn std::error::Error>> {
    let db_conn = init_db()?; // sync init_db returning rusqlite::Connection
    APP_STATE
        .set(AppState { db_conn })
        .map_err(|_| "AppState already initialized".into())
}
