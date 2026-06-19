use std::process::Command;

pub fn run(script: &str) -> String {
    let output = Command::new("osascript").arg("-e").arg(script).output();

    match output {
        Ok(result) => String::from_utf8_lossy(&result.stdout).to_string(),
        Err(_) => String::new(),
    }
}
