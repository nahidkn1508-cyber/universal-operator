use crate::browser_inspector::BrowserTab;

pub trait BrowserAdapter {
    fn name(&self) -> &'static str;

    fn tabs(&self) -> Vec<BrowserTab>;
}
