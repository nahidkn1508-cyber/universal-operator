use common::types::action::Action;
use std::collections::VecDeque;

pub struct ExecutionQueue {
    queue: VecDeque<Action>,
}

impl ExecutionQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.queue.push_back(action);
    }

    pub fn pop(&mut self) -> Option<Action> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
