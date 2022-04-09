use super::cgb_flag::{CgbFlag, CgbSupport};
use crate::util::ascii;
use std::{fmt, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Title {
    bytes: Vec<u8>,
}

fn range(cgb_flag: CgbFlag) -> RangeInclusive<usize> {
    match cgb_flag.support() {
        CgbSupport::None => 0x0134..=0x0143,
        _ => 0x0134..=0x0142,
    }
}

impl Title {
    pub fn load(rom: &[u8]) -> Self {
        let bytes: Vec<u8> = rom[range(CgbFlag::load(rom))].into();
        Self { bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn ascii(&self) -> String {
        ascii::from_bytes(
            self.bytes
                .iter()
                .position(|&x| x == 0x00)
                .map_or(&self.bytes, |index| &self.bytes[..index]),
        )
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ascii())
    }
}
