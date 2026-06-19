use common::types::action::Action;

#[derive(Debug, Clone, Default)]
pub struct ExecutionPlan {
    pub actions: Vec<Action>,
}

impl ExecutionPlan {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}