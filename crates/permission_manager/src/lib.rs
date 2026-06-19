pub struct PermissionManager;

impl PermissionManager {
    pub fn new() -> Self {
        Self
    }

    pub fn can_execute(&self) -> bool {
        true
    }
}
