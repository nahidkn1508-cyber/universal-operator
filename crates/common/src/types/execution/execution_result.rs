#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionResult {
    Success,
    Failed,
    Cancelled,
}
