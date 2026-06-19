/// Normalizes raw AI responses before tokenization.
///
/// This component does not interpret meaning.
/// It only makes the input more consistent.
pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    pub fn normalize(&self, input: &str) -> String {
        input
            .replace("\r\n", "\n")
            .replace('\r', "\n")
            .trim()
            .to_string()
    }
}