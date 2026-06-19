#[derive(Clone)]
pub struct AIProvider {
    pub name: &'static str,
    pub connector: &'static str,
    pub domains: &'static [&'static str],
}
