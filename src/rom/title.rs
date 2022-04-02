use super::cgb_flag::CGBFlag;
use std::ops::RangeInclusive;

fn range(cgb_flag: CGBFlag) -> RangeInclusive<usize> {
    match cgb_flag {
        CGBFlag::None => 0x0134..=0x0143,
        _ => 0x0134..=0x0142,
    }
}

pub fn load_from(rom_bytes: &[u8]) -> String {
    let bytes = &rom_bytes[range(CGBFlag::load_from(rom_bytes))];
    let zero_index = bytes.iter().position(|&x| x == 0x00);
    std::str::from_utf8(match zero_index {
        Some(index) => &bytes[..index],
        None => bytes,
    })
    .unwrap_or("UNKNOWN")
    .into()
}
