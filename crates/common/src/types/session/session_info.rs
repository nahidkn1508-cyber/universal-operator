use super::{SessionId, SessionKind, SessionState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionInfo {
    pub id: SessionId,
    pub kind: SessionKind,
    pub state: SessionState,
}
