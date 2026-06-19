#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorState {
    Idle,
    Connected,
    Waiting,
    Executing,
    Paused,
    Completed,
    Error,
}