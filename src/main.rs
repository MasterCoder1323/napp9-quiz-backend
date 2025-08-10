mod db;
mod game;
mod handlers;
mod sessions;
mod hash;
mod state;
mod users;

use crate::handlers;
use crate::state::APP_STATE;
use chrono::Utc;
use std::io::Read;
use std::sync::OnceLock;
use tiny_http::{Request, Response, Server};

static INIT: OnceLock<()> = OnceLock::new();

fn main() {
    env_logger::init();

    println!("Starting server on http://0.0.0.0:8080");

    // Initialize app state once, blocking
    INIT.get_or_init(|| {
        state::init_app_state().expect("Failed to init app state");
    });

    let server = Server::http("0.0.0.0:8080").expect("Failed to start server");

    for request in server.incoming_requests() {
        log_request(&request);

        let response = route_request(&request);

        let _ = request.respond(response);
    }
}

fn route_request(request: &Request) -> Response<std::io::Cursor<Vec<u8>>> {
    match (request.method().as_str(), request.url()) {
        ("GET", "/") => {
            let body = handlers::root();
            Response::from_string(body)
        }
        ("GET", "/user-list") => {
            let body = handlers::user_list();
            Response::from_string(body)
        }
        ("POST", "/signup") => {
            let body = get_body(request);
            let response = handlers::signup(&body);
            Response::from_string(response)
        }
        ("POST", "/login") => {
            let body = get_body(request);
            let response = handlers::login(&body);
            Response::from_string(response)
        }
        ("POST", "/get_user") => {
            let body = get_body(request);
            let response = handlers::get_user(&body);
            Response::from_string(response)
        }
        _ => Response::empty(404),
    }
}

fn get_body(request: &Request) -> String {
    let mut buf = String::new();
    let mut reader = request.as_reader();
    reader.read_to_string(&mut buf).unwrap_or(0);
    buf
}

fn log_request(req: &Request) {
    let ip = req
        .remote_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|| "unknown".into());

    println!(
        "[{}] {} {} {}",
        Utc::now().to_rfc3339(),
        ip,
        req.method(),
        req.url()
    );
}
