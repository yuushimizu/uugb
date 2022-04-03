#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Context {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}
