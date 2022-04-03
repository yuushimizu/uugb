use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RomSizeAmount {
    Unknown,
    Kb32,
    Kb64,
    Kb128,
    Kb256,
    Kb512,
    Mb1,
    Mb2,
    Mb4,
    Mb8,
    Mb1p1,
    Mb1p2,
    Mb1p5,
}

impl From<u8> for RomSizeAmount {
    fn from(code: u8) -> Self {
        use RomSizeAmount::*;
        match code {
            0x00 => Kb32,
            0x01 => Kb64,
            0x02 => Kb128,
            0x03 => Kb256,
            0x04 => Kb512,
            0x05 => Mb1,
            0x06 => Mb2,
            0x07 => Mb4,
            0x08 => Mb8,
            0x52 => Mb1p1,
            0x53 => Mb1p2,
            0x54 => Mb1p5,
            _ => Unknown,
        }
    }
}

impl fmt::Display for RomSizeAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RomSizeAmount::*;
        write!(
            f,
            "{}",
            match self {
                Unknown => "Unknown",
                Kb32 => "32KB",
                Kb64 => "64KB",
                Kb128 => "128KB",
                Kb256 => "256KB",
                Kb512 => "512KB",
                Mb1 => "1MB",
                Mb2 => "2MB",
                Mb4 => "4MB",
                Mb8 => "8MB",
                Mb1p1 => "1.1MB",
                Mb1p2 => "1.2MB",
                Mb1p5 => "1.5MB",
            }
        )
    }
}

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

    pub fn amount(&self) -> RomSizeAmount {
        self.code.into()
    }
}

impl fmt::Display for RomSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:02X})", self.amount(), self.code)
    }
}
