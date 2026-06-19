use super::BrowserTab;
use super::providers::*;

pub fn browser_tabs() -> Vec<BrowserTab> {
    let mut tabs = Vec::new();

    tabs.extend(chrome_tabs());
    tabs.extend(safari_tabs());
    tabs.extend(arc_tabs());
    tabs.extend(brave_tabs());
    tabs.extend(edge_tabs());
    tabs.extend(firefox_tabs());
    tabs.extend(opera_tabs());
    tabs.extend(vivaldi_tabs());

    tabs
}
