use std::io;
use std::process::Command;

pub struct KeyboardController;

impl KeyboardController {
    pub fn new() -> Self {
        Self
    }

    pub fn type_text(&self, text: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "System Events" to keystroke "{}""#,
                text
            ))
            .status()?;

        Ok(())
    }

    pub fn press_key(&self, key: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "System Events" to key code {}"#,
                key
            ))
            .status()?;

        Ok(())
    }

    pub fn press_enter(&self) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events" to key code 36"#)
            .status()?;

        Ok(())
    }

    pub fn press_escape(&self) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events" to key code 53"#)
            .status()?;

        Ok(())
    }

    pub fn shortcut(&self, key: &str, modifier: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                r#"tell application "System Events" to keystroke "{}" using {{{} down}}"#,
                key, modifier
            ))
            .status()?;

        Ok(())
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
