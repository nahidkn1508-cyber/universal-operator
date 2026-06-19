#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Connected,
    Idle,
    Executing,
    Paused,
    Completed,
    Disconnected,
}
