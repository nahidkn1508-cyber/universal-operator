use crate::browser_inspector::{BrowserTab, apple_script, parser};

pub fn arc_tabs() -> Vec<BrowserTab> {
    let script = r#"
tell application "Arc"
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

    parser::parse("Arc", &output)
}
