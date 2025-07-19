use std::sync::OnceLock;
use crate::db::*;

pub struct AppState {
    pub db_pool: DbPool,
}

pub static APP_STATE: OnceLock<AppState> = OnceLock::new();

pub async fn init_app_state() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = init_db().await?;
    APP_STATE.set(AppState { db_pool })
        .map_err(|_| "AppState already initialized".into())
}
