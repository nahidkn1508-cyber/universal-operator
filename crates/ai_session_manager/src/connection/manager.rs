use std::collections::HashMap;

use super::Connection;

pub struct ConnectionManager {
    connections: HashMap<String, Connection>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub fn connect(&mut self, session_id: String) {
        self.connections.insert(
            session_id.clone(),
            Connection {
                session_id,
                connected: true,
            },
        );
    }

    pub fn disconnect(&mut self, session_id: &str) {
        self.connections.remove(session_id);
    }

    pub fn connected_sessions(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    pub fn is_connected(&self, session_id: &str) -> bool {
        self.connections.contains_key(session_id)
    }
}
