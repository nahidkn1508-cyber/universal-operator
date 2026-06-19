use super::AIProvider;

pub const PROVIDERS: &[AIProvider] = &[
    AIProvider {
        name: "ChatGPT",
        connector: "chatgpt",
        domains: &["chatgpt.com", "chat.openai.com"],
    },
    AIProvider {
        name: "Claude",
        connector: "claude",
        domains: &["claude.ai"],
    },
    AIProvider {
        name: "DeepSeek",
        connector: "deepseek",
        domains: &["chat.deepseek.com"],
    },
];
