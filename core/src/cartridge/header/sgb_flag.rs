use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SgbSupport {
    None,
    Supported,
}

impl From<u8> for SgbSupport {
    fn from(code: u8) -> Self {
        match code {
            0x03 => Self::Supported,
            _ => Self::None,
        }
    }
}

impl fmt::Display for SgbSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SgbSupport::*;
        write!(
            f,
            "{}",
            match self {
                None => "None",
                Supported => "Supported",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SgbFlag {
    code: u8,
}

impl From<u8> for SgbFlag {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const POSITION: usize = 0x0146;

impl SgbFlag {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn support(&self) -> SgbSupport {
        self.code.into()
    }
}

impl fmt::Display for SgbFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:02X})", self.support(), self.code)
    }
}
