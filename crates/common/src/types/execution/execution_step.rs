use crate::types::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionStep {
    pub action: Action,
}
