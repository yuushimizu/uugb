use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RamSize {
    code: u8,
}

impl From<u8> for RamSize {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const POSITION: usize = 0x0149;

impl RamSize {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn amount(&self) -> usize {
        (match self.code {
            0x00 => 0,
            0x02 => 8,
            0x03 => 32,
            0x04 => 128,
            0x05 => 64,
            _ => 0,
        } * 1024)
    }
}

impl fmt::Display for RamSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}KB ({:02X})", self.amount() / 1024, self.code)
    }
}
