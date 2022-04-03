#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CGBSupport {
    None,
    Supported,
    Only,
}

impl From<u8> for CGBSupport {
    fn from(code: u8) -> Self {
        use CGBSupport::*;
        match code {
            0x80 => Supported,
            0xC0 => Only,
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CGBFlag {
    pub code: u8,
    pub support: CGBSupport,
}

impl From<u8> for CGBFlag {
    fn from(code: u8) -> Self {
        Self {
            code,
            support: code.into(),
        }
    }
}

const POSITION: usize = 0x0143;

impl CGBFlag {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
