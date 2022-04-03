#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestinationType {
    Japanese,
    NonJapanese,
}

impl From<u8> for DestinationType {
    fn from(code: u8) -> Self {
        use DestinationType::*;
        match code {
            0x00 => Japanese,
            _ => NonJapanese,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Destination {
    code: u8,
    destination_type: DestinationType,
}

impl From<u8> for Destination {
    fn from(code: u8) -> Self {
        Self {
            code,
            destination_type: code.into(),
        }
    }
}

const POSITION: usize = 0x014A;

impl Destination {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
