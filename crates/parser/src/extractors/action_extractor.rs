use common::types::action::{
    Action,
    ActionId,
    ActionKind,
    ActionStatus,
};

pub struct ActionExtractor;

impl ActionExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract<'a>(&self, instructions: &[&'a str]) -> Vec<Action> {
        instructions
            .iter()
            .enumerate()
            .map(|(id, instruction)| {
                let text = instruction.trim();
                let lower = text.to_lowercase();

                let mut action = Action {
    id: ActionId(id as u64),
    kind: ActionKind::Unknown,
    status: ActionStatus::Pending,
    source: None,
    target: None,
    content: None,
    command: None,
};

                if lower.starts_with("create folder ")
                    || lower.starts_with("create a folder ")
                {
                    action.kind = ActionKind::CreateFolder;

                    action.target = Some(
                        text.replace("Create folder ", "")
                            .replace("Create Folder ", "")
                            .replace("Create a folder ", "")
                            .replace("Create A Folder ", "")
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("create file ")
                    || lower.starts_with("create a file ")
                {
                    action.kind = ActionKind::CreateFile;

                    action.target = Some(
                        text.replace("Create file ", "")
                            .replace("Create File ", "")
                            .replace("Create a file ", "")
                            .replace("Create A File ", "")
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("write ") {
                    action.kind = ActionKind::WriteFile;

                    action.target = Some(
                        text.strip_prefix("Write ")
                            .or_else(|| text.strip_prefix("write "))
                            .unwrap_or(text)
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("append ") {
                    action.kind = ActionKind::AppendFile;

                    action.target = Some(
                        text.strip_prefix("Append ")
                            .or_else(|| text.strip_prefix("append "))
                            .unwrap_or(text)
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("delete ") {
                    action.kind = ActionKind::DeleteFile;

                    action.target = Some(
                        text.strip_prefix("Delete ")
                            .or_else(|| text.strip_prefix("delete "))
                            .unwrap_or(text)
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("rename ") {
                    action.kind = ActionKind::RenameFile;
                    action.target = Some(text.to_string());
                } else if lower.starts_with("move ") {
                    action.kind = ActionKind::MoveFile;
                    action.target = Some(text.to_string());
                } else if lower.starts_with("run command ") {
                    action.kind = ActionKind::RunCommand;

                    action.command = Some(
                        text.strip_prefix("Run command ")
                            .or_else(|| text.strip_prefix("run command "))
                            .unwrap_or(text)
                            .trim()
                            .to_string(),
                    );
                } else if lower.starts_with("cargo ")
                    || lower.starts_with("git ")
                    || lower.starts_with("npm ")
                    || lower.starts_with("pnpm ")
                    || lower.starts_with("yarn ")
                    || lower.starts_with("python ")
                    || lower.starts_with("pip ")
                    || lower.starts_with("brew ")
                {
                    action.kind = ActionKind::RunCommand;
                    action.command = Some(text.to_string());
                }

                action
            })
            .collect()
    }
}