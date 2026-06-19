pub struct AIConnector;

impl AIConnector {
    pub fn new() -> Self {
        Self
    }

    pub fn receive_response<'a>(&self, response: &'a str) -> &'a str {
        response
    }
}
