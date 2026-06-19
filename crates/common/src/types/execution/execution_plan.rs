use super::ExecutionStep;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
}
