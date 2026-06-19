use std::fs;
use std::path::Path;

pub struct FileManager;

impl FileManager {
    pub fn new() -> Self {
        Self
    }

    pub fn write_file(&self, path: &str, contents: &str) {
        if let Some(parent) = Path::new(path).parent() {
            let _ = fs::create_dir_all(parent);
        }

        fs::write(path, contents).expect("Failed to write file");

        println!("✓ Wrote file: {}", path);
    }
}
