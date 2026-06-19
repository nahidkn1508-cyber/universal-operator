use std::time::{SystemTime, UNIX_EPOCH};

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self
    }

    fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn info(&self, message: &str) {
        println!("[{}] INFO  {}", Self::timestamp(), message);
    }

    pub fn success(&self, message: &str) {
        println!("[{}] SUCCESS {}", Self::timestamp(), message);
    }

    pub fn warning(&self, message: &str) {
        println!("[{}] WARNING {}", Self::timestamp(), message);
    }

    pub fn error(&self, message: &str) {
        println!("[{}] ERROR {}", Self::timestamp(), message);
    }

    pub fn session_started(&self, session_id: &str, ai: &str) {
        self.info(&format!("Session {} started (AI: {})", session_id, ai));
    }

    pub fn session_finished(&self, session_id: &str) {
        self.success(&format!("Session {} finished", session_id));
    }

    pub fn action_started(&self, action: &str) {
        self.info(&format!("Executing: {}", action));
    }

    pub fn action_finished(&self, action: &str) {
        self.success(&format!("Completed: {}", action));
    }

    pub fn action_failed(&self, action: &str, error: &str) {
        self.error(&format!("{} ({})", action, error));
    }
}
