#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SGBSupport {
    None,
    Supported,
}

impl From<u8> for SGBSupport {
    fn from(code: u8) -> Self {
        match code {
            0x03 => Self::Supported,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SGBFlag {
    code: u8,
    support: SGBSupport,
}

impl From<u8> for SGBFlag {
    fn from(code: u8) -> Self {
        Self {
            code,
            support: code.into(),
        }
    }
}

const POSITION: usize = 0x0146;

impl SGBFlag {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
