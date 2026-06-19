use common::types::action::{Action, ActionId, ActionKind, ActionStatus};

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, response: &str) -> Vec<Action> {
        let mut actions = Vec::new();

        for (index, line) in response.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some(path) = line.strip_prefix("CREATE_FOLDER ") {
                actions.push(Action {
                    id: ActionId(index as u64),
                    kind: ActionKind::CreateFolder,
                    status: ActionStatus::Pending,
                    source: None,
                    target: Some(path.trim().to_string()),
                    content: None,
                    command: None,
                });
            } else if let Some(path) = line.strip_prefix("CREATE_FILE ") {
                actions.push(Action {
                    id: ActionId(index as u64),
                    kind: ActionKind::CreateFile,
                    status: ActionStatus::Pending,
                    source: None,
                    target: Some(path.trim().to_string()),
                    content: None,
                    command: None,
                });
            } else if let Some(cmd) = line.strip_prefix("RUN_COMMAND ") {
                actions.push(Action {
                    id: ActionId(index as u64),
                    kind: ActionKind::RunCommand,
                    status: ActionStatus::Pending,
                    source: None,
                    target: None,
                    content: None,
                    command: Some(cmd.trim().to_string()),
                });
            } else if let Some(url) = line.strip_prefix("OPEN_URL ") {
                actions.push(Action {
                    id: ActionId(index as u64),
                    kind: ActionKind::OpenUrl,
                    status: ActionStatus::Pending,
                    source: None,
                    target: Some(url.trim().to_string()),
                    content: None,
                    command: None,
                });
            } else if let Some(app) = line.strip_prefix("OPEN_APPLICATION ") {
                actions.push(Action {
                    id: ActionId(index as u64),
                    kind: ActionKind::OpenApplication,
                    status: ActionStatus::Pending,
                    source: None,
                    target: Some(app.trim().to_string()),
                    content: None,
                    command: None,
                });
            }
        }

        actions
    }
}