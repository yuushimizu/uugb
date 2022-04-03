#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CGBFlag {
    None,
    Supported,
    Only,
}

impl From<u8> for CGBFlag {
    fn from(value: u8) -> Self {
        match value {
            0x80 => Self::Supported,
            0xC0 => Self::Only,
            _ => Self::None,
        }
    }
}

const POSITION: usize = 0x0143;

impl CGBFlag {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}