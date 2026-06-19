use crate::session::AISession;
use std::process::Command;

pub fn chatgpt_desktop() -> Vec<AISession> {
    let output = Command::new("osascript")
        .args([
            "-e",
            r#"tell application "System Events"
                return (name of every process)
            end tell"#,
        ])
        .output()
        .ok();

    let Some(output) = output else {
        return Vec::new();
    };

    let processes = String::from_utf8_lossy(&output.stdout);

    if !processes.contains("ChatGPT") {
        return Vec::new();
    }

    vec![AISession {
        id: "chatgpt-desktop".to_string(),
        application: "ChatGPT".to_string(),
        window_title: "ChatGPT Desktop".to_string(),
        connector: "chatgpt".to_string(),
    }]
}
