#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionId(pub u64);

impl ActionId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
