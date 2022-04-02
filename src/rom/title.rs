use super::cgb_flag::CGBFlag;
use crate::util::ascii;
use std::ops::RangeInclusive;

fn range(cgb_flag: CGBFlag) -> RangeInclusive<usize> {
    match cgb_flag {
        CGBFlag::None => 0x0134..=0x0143,
        _ => 0x0134..=0x0142,
    }
}

pub fn load(rom_bytes: &[u8]) -> String {
    let bytes = &rom_bytes[range(CGBFlag::load(rom_bytes))];
    ascii::from_bytes(match bytes.iter().position(|&x| x == 0x00) {
        Some(index) => &bytes[..index],
        None => bytes,
    })
}
