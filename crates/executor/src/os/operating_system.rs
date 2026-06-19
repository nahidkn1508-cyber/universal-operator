pub trait OperatingSystem {
    fn open_application(&self, name: &str);

    fn run_command(&self, command: &str);

    fn create_file(&self, path: &str);

    fn create_folder(&self, path: &str);
}
