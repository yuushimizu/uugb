#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SGBFlag {
    None,
    Supported,
}

impl From<u8> for SGBFlag {
    fn from(value: u8) -> Self {
        match value {
            0x03 => Self::Supported,
            _ => Self::None,
        }
    }
}

const POSITION: usize = 0x0146;

impl SGBFlag {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
