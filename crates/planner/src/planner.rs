use common::types::action::Action;

use crate::ExecutionPlan;

pub struct ExecutionPlanner;

impl ExecutionPlanner {
    pub fn new() -> Self {
        Self
    }

    pub fn create_plan(&self, actions: Vec<Action>) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new();

        for action in actions {
            plan.push(action);
        }

        plan
    }
}