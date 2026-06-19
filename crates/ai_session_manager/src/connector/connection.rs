#[derive(Debug, Clone)]
pub struct AIConnection {
    pub session_id: String,
    pub connected: bool,
}

impl AIConnection {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            connected: false,
        }
    }
}
