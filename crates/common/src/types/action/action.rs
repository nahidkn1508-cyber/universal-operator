use super::{ActionId, ActionKind, ActionStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: ActionId,
    pub kind: ActionKind,
    pub status: ActionStatus,

    pub source: Option<String>,
    pub target: Option<String>,
    pub content: Option<String>,
    pub command: Option<String>,
}
