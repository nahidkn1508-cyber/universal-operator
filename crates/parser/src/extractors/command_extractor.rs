pub struct CommandExtractor;

impl CommandExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract<'a>(&self, response: &'a str) -> Vec<&'a str> {
        const PREFIXES: [&str; 13] = [
            "cargo ",
            "git ",
            "npm ",
            "pnpm ",
            "yarn ",
            "python ",
            "pip ",
            "brew ",
            "rustup ",
            "docker ",
            "make ",
            "./",
            "chmod ",
        ];

        response
            .lines()
            .map(str::trim)
            .filter(|line| PREFIXES.iter().any(|p| line.starts_with(p)))
            .collect()
    }
}