use std::fs::{self, File};
use std::io;
use std::path::Path;

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_folder<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn create_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        File::create(path)?;
        Ok(())
    }

    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn rename<P: AsRef<Path>>(&self, from: P, to: P) -> io::Result<()> {
        fs::rename(from, to)
    }

    pub fn move_file<P: AsRef<Path>>(&self, from: P, to: P) -> io::Result<()> {
        fs::rename(from, to)
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> io::Result<()> {
        fs::write(path, content)
    }

    pub fn append_file<P: AsRef<Path>>(&self, path: P, content: &str) -> io::Result<()> {
        use std::io::Write;

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }
}
