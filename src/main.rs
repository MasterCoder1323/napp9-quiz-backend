mod db;
mod routes;
mod sessions;
mod users;

use actix_web::{web, App, HttpServer};
use db::init_db;

pub struct AppState {
    pub db_pool: db::DbPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = init_db().await.expect("Failed to init DB");

    println!("Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db_pool: db_pool.clone(),
            }))
            .configure(routes::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
