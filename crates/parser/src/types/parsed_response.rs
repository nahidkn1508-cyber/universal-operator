use common::types::action::Action;

#[derive(Debug, Clone, Default)]
pub struct ParsedResponse {
    pub raw_response: String,

    pub code_blocks: Vec<String>,

    pub commands: Vec<String>,

    pub files: Vec<String>,

    pub actions: Vec<Action>,
}

impl ParsedResponse {
    pub fn new(raw_response: impl Into<String>) -> Self {
        Self {
            raw_response: raw_response.into(),
            code_blocks: Vec::new(),
            commands: Vec::new(),
            files: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_code_block(&mut self, block: impl Into<String>) {
        self.code_blocks.push(block.into());
    }

    pub fn add_command(&mut self, command: impl Into<String>) {
        self.commands.push(command.into());
    }

    pub fn add_file(&mut self, file: impl Into<String>) {
        self.files.push(file.into());
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn code_block_count(&self) -> usize {
    self.code_blocks.len()
}

pub fn command_count(&self) -> usize {
    self.commands.len()
}

pub fn file_count(&self) -> usize {
    self.files.len()
}
}