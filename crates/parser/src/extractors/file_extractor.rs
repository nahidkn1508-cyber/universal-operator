pub struct FileExtractor;

impl FileExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract<'a>(&self, response: &'a str) -> Vec<&'a str> {
        response
            .lines()
            .map(str::trim)
            .filter(|line| {
                let extensions = [
                    ".rs",
                    ".toml",
                    ".json",
                    ".yaml",
                    ".yml",
                    ".ts",
                    ".tsx",
                    ".js",
                    ".jsx",
                    ".html",
                    ".css",
                    ".py",
                    ".java",
                    ".cpp",
                    ".c",
                    ".go",
                    ".php",
                    ".md",
                ];

                extensions.iter().any(|ext| line.ends_with(ext))
            })
            .collect()
    }
}