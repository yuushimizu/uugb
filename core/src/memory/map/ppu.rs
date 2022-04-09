use super::{Segment, UNKNOWN};

pub const PPU: Segment = Segment::Nested(|address| match address {
    0x8000..=0x9FFF => &Segment::Offset(
        0x8000,
        &Segment::Leaf(
            |components, address| components.ppu.vram().read(address),
            |components, address, value| components.ppu.vram_mut().write(address, value),
        ),
    ),
    0xFE00..=0xFE9F => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    0xFF40 => &Segment::Leaf(
        |components, _| components.ppu.control_bits(),
        |components, _, value| components.ppu.set_control_bits(value),
    ),
    0xFF41 => &Segment::Leaf(
        |components, _| components.ppu.status_bits(),
        |components, _, value| components.ppu.set_status_bits(value),
    ),
    0xFF42..=0xFF4F => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    0xFF51..=0xFF55 => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    0xFF68..=0xFF6C => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    _ => &UNKNOWN,
});
