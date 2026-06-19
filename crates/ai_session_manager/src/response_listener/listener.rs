use std::process::Command;

pub struct ResponseListener;

impl ResponseListener {
    pub fn wait_for_response(session_id: &str) -> Option<String> {
        println!("Waiting for AI response from {}", session_id);

        let output = Command::new("xcrun")
            .args(["swift", "tools/chatgpt_read.swift"])
            .output()
            .ok()?;

        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if text.is_empty() {
            println!("No AI response yet.");
            None
        } else {
            println!("AI response received.");
            Some(text)
        }
    }
}
