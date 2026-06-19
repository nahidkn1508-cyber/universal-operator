use std::process::Command;

pub fn detect_running_apps() -> Vec<String> {
    let output = Command::new("osascript")
        .args([
            "-e",
            r#"
tell application "System Events"
    set visibleApps to {}
    repeat with p in (application processes)
        if background only of p is false then
            copy (name of p) to end of visibleApps
        end if
    end repeat
    return visibleApps
end tell
"#,
        ])
        .output()
        .expect("failed to execute osascript");

    let text = String::from_utf8_lossy(&output.stdout);

    text.split(',')
        .map(|app| app.trim().to_string())
        .filter(|app| !app.is_empty())
        .collect()
}
