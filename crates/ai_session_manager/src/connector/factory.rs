use crate::connector::{AIConnector, ChatGPTConnector};

pub fn create_connector(connector: &str, session_id: String) -> Option<Box<dyn AIConnector>> {
    match connector {
        "chatgpt" => Some(Box::new(ChatGPTConnector::new(session_id))),
        _ => None,
    }
}
