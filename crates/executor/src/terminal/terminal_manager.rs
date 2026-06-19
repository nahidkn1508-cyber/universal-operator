use std::io;
use std::path::Path;
use std::process::Command;

pub struct TerminalManager;

impl TerminalManager {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, command: &str) -> io::Result<()> {
        self.run_in(command, ".")
    }

    pub fn run_in<P: AsRef<Path>>(&self, command: &str, directory: P) -> io::Result<()> {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(directory)
            .status()?;

        Ok(())
    }
}
