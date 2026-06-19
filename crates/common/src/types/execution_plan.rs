use super::Action;

#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    pub actions: Vec<Action>,
}