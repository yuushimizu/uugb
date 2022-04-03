#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Destination {
    Japanese,
    NonJapanese,
}

impl From<u8> for Destination {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Self::NonJapanese,
            _ => Self::Japanese,
        }
    }
}

const POSITION: usize = 0x014A;

impl Destination {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
