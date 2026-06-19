mod apple_script;
mod browser;
mod browser_adapter;
mod parser;
mod providers;

pub use browser::*;
pub use browser_adapter::*;

#[derive(Clone, Debug)]
pub struct BrowserTab {
    pub browser: String,
    pub title: String,
    pub url: String,
}
