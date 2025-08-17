use super::state::*;
use super::types::*;
use super::questions::*;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::accept;
use serde_json;
use tungstenite::Message;

pub fn ws_main() {
    let state = Arc::new(AppState::new());

    println!("Tungstenite Server Starting on port 56...");
    let server = TcpListener::bind("localhost:56").expect("Cannot bind port 56");
    println!("WebSocket Server running on port 56");

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                let ws_state = Arc::clone(&state);

                thread::spawn(move || {
                    let websocket = accept(stream).expect("Failed to accept WebSocket connection");
                    let ws_arc = Arc::new(Mutex::new(websocket));

                    // Expect first message to be username
                    let username = {
                        let mut ws_lock = ws_arc.lock().unwrap();
                        match ws_lock.read() {
                            Ok(Message::Text(name)) => name,
                            _ => {
                                println!("Failed to get username. Closing connection.");
                                return;
                            }
                        }
                    };

                    println!("{} connected", username);
                    ws_state.add_client(username.clone(), Arc::clone(&ws_arc));

                    loop {
                        // Pick a random question
                        let (question, correct_index) = QUESTIONS.select_random_question();

                        // Send the question as JSON
                        ws_state.send_to_client(
                            &username,
                            &serde_json::to_string(&question).unwrap(),
                        );

                        // Wait for client response (expected to be the selected index as string)
                        let msg = {
                            let mut ws_lock = ws_arc.lock().unwrap();
                            ws_lock.read()
                        };

                        match msg {
                            Ok(Message::Text(text)) => {
                                // Parse the selected index
                                if let Ok(selected_index) = text.trim().parse::<usize>() {
                                    if check_answer(selected_index, correct_index) {
                                        ws_state.add_or_update_points(&username, 1);
                                        ws_state.send_to_client(&username, "Correct!".into());
                                    } else {
                                        ws_state.send_to_client(&username, "Wrong!".into());
                                    }
                                } else {
                                    ws_state.send_to_client(&username, "Invalid answer!".into());
                                }
                            }
                            Ok(Message::Close(_)) => {
                                println!("{} disconnected", username);
                                ws_state.remove_client(&username);
                                break;
                            }
                            Err(_) => {
                                println!("Connection error for {}", username);
                                ws_state.remove_client(&username);
                                break;
                            }
                            _ => {}
                        }
                    }
                });
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
}
