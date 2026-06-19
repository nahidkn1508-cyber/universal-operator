use super::WindowInfo;

pub fn visible_windows() -> Vec<WindowInfo> {
    // Temporary placeholder.
    // Next step we'll replace this with CoreGraphics.
    vec![
        WindowInfo {
            owner: "ChatGPT".into(),
            title: "New Chat".into(),
        },
        WindowInfo {
            owner: "Visual Studio Code".into(),
            title: "universal-operator".into(),
        },
    ]
}
