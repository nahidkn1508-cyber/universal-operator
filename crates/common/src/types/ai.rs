#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiProvider {
    ChatGPT,
    Claude,
    Gemini,
    DeepSeek,
    Grok,
    Local,
    Unknown,
}
