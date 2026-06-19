pub trait AIConnector {
    fn connect(&mut self) -> bool;

    fn disconnect(&mut self);

    fn is_connected(&self) -> bool;

    fn send_prompt(&self, prompt: &str) -> Result<(), String>;

    fn receive_response(&self) -> Result<String, String>;
}
