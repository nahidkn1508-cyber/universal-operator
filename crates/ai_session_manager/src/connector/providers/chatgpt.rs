use crate::connector::{AIConnection, AIConnector};

pub struct ChatGPTConnector {
    connection: AIConnection,
}

impl ChatGPTConnector {
    pub fn new(session_id: String) -> Self {
        Self {
            connection: AIConnection::new(session_id),
        }
    }
}

impl AIConnector for ChatGPTConnector {
    fn connect(&mut self) -> bool {
        self.connection.connected = true;
        true
    }

    fn disconnect(&mut self) {
        self.connection.connected = false;
    }

    fn is_connected(&self) -> bool {
        self.connection.connected
    }

    fn send_prompt(&self, prompt: &str) -> Result<(), String> {
        println!("Sending prompt:");
        println!("{}", prompt);

        Ok(())
    }

    fn receive_response(&self) -> Result<String, String> {
        Ok(String::new())
    }
}
