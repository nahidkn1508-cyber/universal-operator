use common::types::action::{Action, ActionKind};

pub struct ExecutionPlanner;

impl ExecutionPlanner {
    pub fn new() -> Self {
        Self
    }

    pub fn create_plan(&self, mut actions: Vec<Action>) -> Vec<Action> {
        actions.sort_by_key(|action| {
            match action.kind {
                ActionKind::CreateFolder => 0,

                ActionKind::CreateFile => 1,

                ActionKind::RunCommand => {
                    if let Some(cmd) = &action.command {
                        if cmd.starts_with("cargo new")
                            || cmd.starts_with("cargo init")
                            || cmd.starts_with("npm create")
                            || cmd.starts_with("npx create")
                        {
                            2
                        } else {
                            5
                        }
                    } else {
                        5
                    }
                }

                ActionKind::WriteFile => 3,

                ActionKind::AppendFile => 4,

                _ => 100,
            }
        });

        actions
    }
}