use super::ExecutionStep;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionQueue {
    pub pending: Vec<ExecutionStep>,
}
