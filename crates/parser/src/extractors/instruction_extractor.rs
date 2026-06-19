pub struct InstructionExtractor;

impl InstructionExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract<'a>(&self, response: &'a str) -> Vec<&'a str> {
        let mut instructions = Vec::new();

        for line in response.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with("```") {
                continue;
            }

            let line = Self::strip_prefix(line);

            if Self::is_instruction(line) {
                instructions.push(line);
            }
        }

        instructions
    }

    fn strip_prefix(line: &str) -> &str {
        let mut text = line.trim();

        while let Some(first) = text.chars().next() {
            match first {
                '-' | '*' | '>' => {
                    text = text[1..].trim_start();
                }
                '0'..='9' => {
                    if let Some(pos) = text.find('.') {
                        text = text[pos + 1..].trim_start();
                    } else if let Some(pos) = text.find(')') {
                        text = text[pos + 1..].trim_start();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        text
    }

    fn is_instruction(line: &str) -> bool {
        let lower = line.to_lowercase();

        lower.starts_with("create ")
            || lower.starts_with("write ")
            || lower.starts_with("append ")
            || lower.starts_with("delete ")
            || lower.starts_with("rename ")
            || lower.starts_with("move ")
            || lower.starts_with("copy ")
            || lower.starts_with("open ")
            || lower.starts_with("close ")
            || lower.starts_with("launch ")
            || lower.starts_with("run ")
            || lower.starts_with("execute ")
            || lower.starts_with("cargo ")
            || lower.starts_with("git ")
            || lower.starts_with("npm ")
            || lower.starts_with("pnpm ")
            || lower.starts_with("yarn ")
            || lower.starts_with("python ")
            || lower.starts_with("pip ")
            || lower.starts_with("brew ")
            || lower.starts_with("mkdir ")
            || lower.starts_with("touch ")
            || lower.starts_with("cd ")
    }
}