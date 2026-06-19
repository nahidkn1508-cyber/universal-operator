use crate::os::OperatingSystem;

pub struct MacOS;

impl OperatingSystem for MacOS {
    fn open_application(&self, name: &str) {
        println!("Opening application: {}", name);
    }

    fn run_command(&self, command: &str) {
        println!("Running command: {}", command);
    }

    fn create_file(&self, path: &str) {
        println!("Creating file: {}", path);
    }

    fn create_folder(&self, path: &str) {
        println!("Creating folder: {}", path);
    }
}