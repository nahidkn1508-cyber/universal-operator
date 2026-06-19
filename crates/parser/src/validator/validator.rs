/// Validates normalized AI responses before extraction.
///
/// The validator never modifies the input.
/// It only determines whether the input is suitable for parsing.
pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, input: &str) -> bool {
        !input.trim().is_empty()
    }
}