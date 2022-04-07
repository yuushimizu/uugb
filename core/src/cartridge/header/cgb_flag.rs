use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CgbSupport {
    None,
    Supported,
    Only,
}

impl From<u8> for CgbSupport {
    fn from(code: u8) -> Self {
        use CgbSupport::*;
        match code {
            0x80 => Supported,
            0xC0 => Only,
            _ => None,
        }
    }
}

impl fmt::Display for CgbSupport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CgbSupport::*;
        write!(
            f,
            "{}",
            match self {
                None => "None",
                Supported => "Supported",
                Only => "Only",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CgbFlag {
    code: u8,
}

impl From<u8> for CgbFlag {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const ADDRESS: usize = 0x0143;

impl CgbFlag {
    pub fn load(rom: &[u8]) -> Self {
        rom[ADDRESS].into()
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn support(&self) -> CgbSupport {
        self.code.into()
    }
}

impl fmt::Display for CgbFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:02X})", self.support(), self.code)
    }
}
