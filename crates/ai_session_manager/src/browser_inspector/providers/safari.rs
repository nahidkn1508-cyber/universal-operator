use crate::browser_inspector::{BrowserTab, apple_script, parser};

pub fn safari_tabs() -> Vec<BrowserTab> {
    let script = r#"
tell application "Safari"
    set output to ""
    repeat with w in windows
        repeat with t in tabs of w
            set output to output & (name of t) & "|" & (URL of t) & linefeed
        end repeat
    end repeat
    return output
end tell
"#;

    let output = apple_script::run(script);

    parser::parse("Safari", &output)
}
