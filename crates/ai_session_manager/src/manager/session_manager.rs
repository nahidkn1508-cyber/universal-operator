use crate::{
    browser_inspector::browser_tabs, desktop_inspector::desktop_sessions, registry::match_provider,
    session::AISession,
};

pub struct SessionManager;

impl SessionManager {
    pub fn discover() -> Vec<AISession> {
        let mut sessions = Vec::new();

        // Browser AI sessions
        for tab in browser_tabs() {
            if let Some(provider) = match_provider(&tab.url) {
                sessions.push(AISession {
                    id: format!("{}:{}", provider.name, tab.title),
                    application: tab.browser.clone(),
                    window_title: tab.title.clone(),
                    connector: provider.connector.to_string(),
                });
            }
        }

        // Desktop AI sessions
        sessions.extend(desktop_sessions());

        sessions
    }
}
