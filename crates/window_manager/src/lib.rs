use std::io;
use std::process::Command;

pub struct WindowManager;

impl WindowManager {
    pub fn new() -> Self {
        Self
    }

    pub fn focus(&self, app: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{}" to activate"#, app))
            .status()?;

        Ok(())
    }

    pub fn close(&self, app: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{}" to quit"#, app))
            .status()?;

        Ok(())
    }

    pub fn minimize(&self, app: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "System Events"
    tell process "{}"
        set value of attribute "AXMinimized" of window 1 to true
    end tell
end tell"#,
                app
            ))
            .status()?;

        Ok(())
    }

    pub fn maximize(&self, app: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(r#"tell application "{}" to activate"#, app))
            .status()?;

        Ok(())
    }

    pub fn move_window(&self, _app: &str, _x: i32, _y: i32) -> io::Result<()> {
        Ok(())
    }

    pub fn resize_window(&self, _app: &str, _width: i32, _height: i32) -> io::Result<()> {
        Ok(())
    }
}
