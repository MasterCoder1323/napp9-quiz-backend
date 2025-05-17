use crate::{users, AppState};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignupRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct UserRequest {
    token: String,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body("New Apps 9 Lean Server API <br> <a href='/user-list'>User List</a>")
}

#[get("/user-list")]
async fn user_list(data: web::Data<AppState>) -> impl Responder {
    match users::get_user_list(&data.db_pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/signup")]
async fn signup(data: web::Data<AppState>, form: web::Json<SignupRequest>) -> impl Responder {
    match users::add_user(&data.db_pool, &form.username, &form.password).await {
        Ok(token) => HttpResponse::Ok().content_type("text/plain").body(token),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/login")]
async fn login(data: web::Data<AppState>, form: web::Json<UserRequest>) -> impl Responder {
	match users::login_user(&data.db_pool, &form.username, &form.password).await {
		Ok(Some(token)) => HttpResponse::Ok().content_type("text/plain").body(token),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
	}
}

#[post("/get-user")]
async fn get_user_post(data: web::Data<AppState>, form: web::Json<UserRequest>) -> impl Responder {
    let token: &str = &form.token;
    match users::get_user_from_token(&data.db_pool, token).await {
        Ok(Some(user)) => HttpResponse::Ok()
            .content_type("application/json")
            .body(user.sterilise()), // assumes User implements Serialize
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(root);
    cfg.service(user_list);
    cfg.service(signup);
    cfg.service(get_user_post);
}
