use common::types::action::{Action, ActionKind};

pub struct ExecutionPlanner;

impl ExecutionPlanner {
    pub fn new() -> Self {
        Self
    }

    pub fn create_plan(&self, mut actions: Vec<Action>) -> Vec<Action> {
        actions.sort_by_key(|action| match action.kind {
            ActionKind::CreateFolder => 0,
            ActionKind::CreateFile => 1,
            ActionKind::WriteFile => 2,
            ActionKind::AppendFile => 3,
            ActionKind::DeleteFile => 4,
            ActionKind::RenameFile => 5,
            ActionKind::MoveFile => 6,
            ActionKind::RunCommand => 7,
            ActionKind::OpenApplication => 8,
            ActionKind::CloseApplication => 9,
            ActionKind::OpenUrl => 10,
            ActionKind::Wait => 11,
            ActionKind::UserConfirmation => 12,
            ActionKind::Unknown => 13,
        });

        actions
    }
}
