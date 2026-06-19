use crate::discovery::detect_running_apps;

pub struct SessionDetector;

impl SessionDetector {
    pub fn detect() -> Vec<String> {
        detect_running_apps()
    }
}
