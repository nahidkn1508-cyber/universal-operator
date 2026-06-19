use common::types::action::{Action, ActionKind};

pub struct ExecutionEngine;

impl ExecutionEngine {
    pub fn execute(action: Action) {
        match action.kind {
            ActionKind::CreateFolder => {
                println!("📁 CREATE_FOLDER {:?}", action.target);
            }

            ActionKind::CreateFile => {
                println!("📄 CREATE_FILE {:?}", action.target);
            }

            ActionKind::WriteFile => {
                println!("📝 WRITE_FILE {:?}", action.target);
                println!("{:?}", action.content);
            }

            ActionKind::RunCommand => {
                println!("💻 RUN_COMMAND {:?}", action.command);
            }

            ActionKind::OpenApplication => {
                println!("🚀 OPEN_APPLICATION {:?}", action.target);
            }

            ActionKind::OpenUrl => {
                println!("🌐 OPEN_URL {:?}", action.target);
            }

            ActionKind::Wait => {
                println!("⏳ WAIT");
            }

            _ => {
                println!("Unsupported action");
            }
        }
    }
}