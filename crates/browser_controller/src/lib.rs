use std::io;
use std::process::Command;

pub struct BrowserController;

impl BrowserController {
    pub fn new() -> Self {
        Self
    }

    pub fn open(&self, url: &str) -> io::Result<()> {
        Command::new("open").arg(url).status()?;
        Ok(())
    }
}
