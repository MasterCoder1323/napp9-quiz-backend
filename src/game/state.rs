use serde::Serialize;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tungstenite::WebSocket;

/// Type alias for user points
pub type UserPoints = (String, u16);

/// Represents a connected client
pub struct Client {
    pub username: String,
    pub socket: WebSocket<TcpStream>,
}

/// Shared app state
#[derive(Clone)]
pub struct AppState {
    /// Points for users
    pub points: Arc<Mutex<Vec<UserPoints>>>,
    /// Connected clients, indexed by username
    pub clients: Arc<Mutex<HashMap<String, Arc<Mutex<WebSocket<TcpStream>>>>>>,
}

impl AppState {
    /// Create new empty state
    pub fn new() -> Self {
        Self {
            points: Arc::new(Mutex::new(Vec::new())),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add or update points for a user
    pub fn add_or_update_points(&self, username: &str, points: u16) {
        let mut pts = self.points.lock().unwrap();
        if let Some(entry) = pts.iter_mut().find(|(name, _)| name == username) {
            entry.1 = entry.1.saturating_add(points);
        } else {
            pts.push((username.to_string(), points));
        }
    }

    /// Add a connected client
    pub fn add_client(&self, username: String, socket: Arc<Mutex<WebSocket<TcpStream>>>) {
        self.clients.lock().unwrap().insert(username, socket);
    }

    /// Remove a client by username
    pub fn remove_client(&self, username: &str) {
        self.clients.lock().unwrap().remove(username);
    }

    /// Send a message to a specific client
    pub fn send_to_client(&self, username: &str, message: &str) {
        if let Some(client) = self.clients.lock().unwrap().get(username) {
            let mut ws = client.lock().unwrap();
            let _ = ws.send(tungstenite::Message::Text(message.to_string()));
        }
    }

    /// Broadcast a message to all connected clients
    pub fn broadcast(&self, message: &str) {
        let clients = self.clients.lock().unwrap();
        for client in clients.values() {
            let mut ws = client.lock().unwrap();
            let _ = ws.send(tungstenite::Message::Text(message.to_string()));
        }
    }

    /// Get a snapshot of points
    pub fn get_points_snapshot(&self) -> Vec<UserPoints> {
        self.points.lock().unwrap().clone()
    }

    /// Get list of connected usernames
    pub fn list_connected_users(&self) -> Vec<String> {
        self.clients.lock().unwrap().keys().cloned().collect()
    }
}

#[derive(Serialize)]
pub struct PointsSnapshot {
    pub points: Vec<UserPoints>,
}

impl AppState {
    pub fn to_points_snapshot(&self) -> PointsSnapshot {
        PointsSnapshot {
            points: self.get_points_snapshot(),
        }
    }
}
