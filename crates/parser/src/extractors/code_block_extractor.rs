pub struct CodeBlockExtractor;

impl CodeBlockExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract(&self, response: &str) -> Vec<String> {
        let mut blocks = Vec::new();

        let mut inside_block = false;
        let mut current = String::new();

        for line in response.lines() {
            let trimmed = line.trim_start();

            if trimmed.starts_with("```") {
                if inside_block {
                    blocks.push(current.clone());
                    current.clear();
                    inside_block = false;
                } else {
                    inside_block = true;
                }

                continue;
            }

            if inside_block {
                current.push_str(line);
                current.push('\n');
            }
        }

        blocks
    }
}