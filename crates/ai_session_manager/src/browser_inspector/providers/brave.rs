use crate::browser_inspector::{BrowserTab, apple_script, parser};

pub fn brave_tabs() -> Vec<BrowserTab> {
    let script = r#"
tell application "Brave Browser"
    set output to ""
    repeat with w in windows
        repeat with t in tabs of w
            set output to output & (title of t) & "|" & (URL of t) & linefeed
        end repeat
    end repeat
    return output
end tell
"#;

    let output = apple_script::run(script);

    parser::parse("Brave", &output)
}
