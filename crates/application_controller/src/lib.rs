use std::io;
use std::process::Command;

pub struct ApplicationController;

impl ApplicationController {
    pub fn new() -> Self {
        Self
    }

    pub fn open(&self, application: &str) -> io::Result<()> {
        Command::new("open").arg("-a").arg(application).status()?;

        Ok(())
    }

    pub fn close(&self, application: &str) -> io::Result<()> {
        Command::new("osascript")
            .arg("-e")
            .arg(format!("tell application \"{}\" to quit", application))
            .status()?;

        Ok(())
    }
}
