#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Skipped,
}
