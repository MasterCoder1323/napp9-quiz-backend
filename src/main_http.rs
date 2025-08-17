use crate::handlers as routes;
use chrono::Utc;
use std::sync::OnceLock;
use crate::state;
use tiny_http::{Request, Response, Server};

static INIT: OnceLock<()> = OnceLock::new();

pub fn tiny_http_main() {

    println!("Starting server on http://localhost:55");

    // Initialize app state once, blocking
    INIT.get_or_init(|| {
        state::init_app_state().expect("Failed to init app state");
    });

    let server = Server::http("localhost:55").expect("Failed to start server");

    for mut request in server.incoming_requests() {
        log_request(&request);

        let response = route_request(&mut request);

        let _ = request.respond(response);
    }
}

fn route_request(request: &mut Request) -> Response<std::io::Cursor<Vec<u8>>> {
    match (request.method().as_str(), request.url()) {
        ("GET", "/") => {
            let body = routes::root();
            Response::from_string(body)
        }
        ("GET", "/user-list") => {
            let body = routes::user_list();
            Response::from_string(body)
        }
        ("POST", "/signup") => {
            let body = get_body(request);
            let response = routes::signup(&body);
            Response::from_string(response)
        }
        ("POST", "/login") => {
            let body = get_body(request);
            let response = routes::login(&body);
            Response::from_string(response)
        }
        ("POST", "/get_user") => {
            let body = get_body(request);
            let response = routes::get_user(&body);
            Response::from_string(response)
        }
        _ => Response::from_string("").with_status_code(404),
    }
}

fn get_body(request: &mut Request) -> String {
    let mut buf = String::new();
    let reader = request.as_reader();
    reader.read_to_string(&mut buf).unwrap_or(0);
    buf
}

fn log_request(req: &Request) {
    println!(
        "[{}] {} {}",
        Utc::now().to_rfc3339(),
        req.method(),
        req.url()
    );
}
