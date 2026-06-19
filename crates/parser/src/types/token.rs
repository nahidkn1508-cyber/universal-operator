#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
}

impl Token {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}