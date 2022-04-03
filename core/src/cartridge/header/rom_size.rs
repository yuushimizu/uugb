use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RomSize {
    code: u8,
}

impl From<u8> for RomSize {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const POSITION: usize = 0x0148;

impl RomSize {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn code(&self) -> u8 {
        self.code
    }

    pub fn amount(&self) -> usize {
        (match self.code {
            0x00..=0x08 => 32 * 2usize.pow(self.code.into()),
            0x52 => (1.1 * 1024 as f64) as usize,
            0x53 => (1.2 * 1024 as f64) as usize,
            0x54 => (1.5 * 1024 as f64) as usize,
            _ => 0,
        } * 1024)
    }
}

impl fmt::Display for RomSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}KB ({:02X})", self.amount() / 1024, self.code)
    }
}
