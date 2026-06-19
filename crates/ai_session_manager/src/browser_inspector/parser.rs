use crate::browser_inspector::BrowserTab;

pub fn parse(browser: &str, output: &str) -> Vec<BrowserTab> {
    let mut tabs = Vec::new();

    for line in output.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts: Vec<_> = line.split('|').collect();

        if parts.len() != 2 {
            continue;
        }

        tabs.push(BrowserTab {
            browser: browser.to_string(),
            title: parts[0].trim().to_string(),
            url: parts[1].trim().to_string(),
        });
    }

    tabs
}
