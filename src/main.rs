#![allow(dead_code)]

mod db;
mod game;
mod handlers;
mod hash;
mod main_http;
mod sessions;
mod state;
mod users;

use game::server::ws_main;
use main_http::tiny_http_main;
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
