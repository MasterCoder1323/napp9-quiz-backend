use crate::handlers as routes;
use crate::state;
use chrono::Utc;
use std::io::Cursor;
use std::sync::OnceLock;
use tiny_http::{Header, Request, Response, Server};

static INIT: OnceLock<()> = OnceLock::new();

pub fn tiny_http_main() {
    println!("Starting server on http://localhost:2330");

    // Initialize app state once, blocking
    INIT.get_or_init(|| {
        state::init_app_state().expect("Failed to init app state");
    });

    let server = Server::http("localhost:2330").expect("Failed to start server");

    for mut request in server.incoming_requests() {
        log_request(&request);

        let response = route_request(&mut request);

        let _ = request.respond(response);
    }
}

fn corsify(mut resp: Response<Cursor<Vec<u8>>>) -> Response<Cursor<Vec<u8>>> {
    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
    resp
}

fn preflight_response() -> Response<Cursor<Vec<u8>>> {
    let mut resp = Response::from_string(""); // use empty string body instead of `Response::empty`
    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"POST, OPTIONS, GET"[..]).unwrap());
    resp.add_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap());
    resp
}

fn route_request(request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    match (request.method().as_str(), request.url()) {
        // Handle OPTIONS preflight for *any* POST route
        ("OPTIONS", "/signup") | ("OPTIONS", "/login") | ("OPTIONS", "/get_user") => {
            preflight_response()
        }

        ("GET", "/") => corsify(Response::from_string(routes::root())),
        ("GET", "/user-list") => corsify(Response::from_string(routes::user_list())),

        ("POST", "/signup") => {
            let body = get_body(request);
            corsify(Response::from_string(routes::signup(&body)))
        }
        ("POST", "/login") => {
            let body = get_body(request);
            corsify(Response::from_string(routes::login(&body)))
        }
        ("POST", "/get_user") => {
            let body = get_body(request);
            corsify(Response::from_string(routes::get_user(&body)))
        }

        _ => corsify(Response::from_string("").with_status_code(404)),
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
