use super::cgb_flag::{CGBFlag, CGBSupport};
use crate::util::ascii;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Title {
    bytes: Vec<u8>,
    ascii: String,
}

fn range(cgb_flag: CGBFlag) -> RangeInclusive<usize> {
    match cgb_flag.support {
        CGBSupport::None => 0x0134..=0x0143,
        _ => 0x0134..=0x0142,
    }
}

impl Title {
    pub fn load(rom_bytes: &[u8]) -> Self {
        let bytes: Vec<u8> = rom_bytes[range(CGBFlag::load(rom_bytes))].into();
        let ascii = ascii::from_bytes(
            bytes
                .iter()
                .position(|&x| x == 0x00)
                .map_or(&bytes, |index| &bytes[..index]),
        );
        Self { bytes, ascii }
    }
}
