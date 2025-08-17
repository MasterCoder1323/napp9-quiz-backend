mod db;
mod game;
mod handlers;
mod sessions;
mod hash;
mod state;
mod users;
mod main_http;

use main_http::tiny_http_main;
use game::server::ws_main;
use std::thread;

fn main() {
    let http_handle = thread::spawn(|| {
        tiny_http_main();
    });

    let ws_handle = thread::spawn(|| {
        ws_main();
    });

    http_handle.join().unwrap();
    ws_handle.join().unwrap();
}