use crate::session::AISession;
use std::process::Command;

pub fn desktop_sessions() -> Vec<AISession> {
    let script = r#"
tell application "System Events"
    set output to ""

    repeat with p in application processes
        set appName to name of p

        if appName is "ChatGPT" or appName is "Claude" or appName is "LM Studio" then
            set output to output & appName & linefeed
        end if
    end repeat

    return output
end tell
"#;

    let output = Command::new("osascript").arg("-e").arg(script).output();

    let mut sessions = Vec::new();

    if let Ok(result) = output {
        let text = String::from_utf8_lossy(&result.stdout);

        for app in text.lines() {
            let app = app.trim();

            if app.is_empty() {
                continue;
            }

            let connector = match app {
                "ChatGPT" => "chatgpt",
                "Claude" => "claude",
                "LM Studio" => "lmstudio",
                _ => continue,
            };

            sessions.push(AISession {
                id: format!("desktop:{}", app),
                application: app.to_string(),
                window_title: app.to_string(),
                connector: connector.to_string(),
            });
        }
    }

    sessions
}
