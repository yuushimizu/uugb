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
    0xFF42 => &Segment::Leaf(
        |components, _| components.ppu.scroll_position().y,
        |components, _, value| components.ppu.scroll_position_mut().y = value,
    ),
    0xFF43 => &Segment::Leaf(
        |components, _| components.ppu.scroll_position().x,
        |components, _, value| components.ppu.scroll_position_mut().x = value,
    ),
    0xFF44 => &Segment::Leaf(|components, _| components.ppu.current_y(), |_, _, _| {}),
    0xFF45 => &Segment::Leaf(
        |components, _| components.ppu.y_compare(),
        |components, _, value| components.ppu.set_y_compare(value),
    ),
    0xFF46 => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // DMA
    0xFF47..=0xFF49 => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // CGB: Monochrome palettes
    0xFF4A => &Segment::Leaf(
        |components, _| components.ppu.window_position().y,
        |components, _, value| components.ppu.window_position_mut().y = value,
    ),
    0xFF4B => &Segment::Leaf(
        |components, _| components.ppu.window_position().x,
        |components, _, value| components.ppu.window_position_mut().x = value,
    ),
    0xFF4C => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    0xFF4D => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // CGB: Prepare Speed Switch
    0xFF4E => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
    0xFF4F => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // CGB: VRAM Bank
    0xFF50 => &UNKNOWN,
    0xFF51..=0xFF55 => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // CGB: DMA
    0xFF68..=0xFF6B => &Segment::Leaf(|_, _| 0, |_, _, _| {}), // CGB: Color Palettes
    0xFF6C => &Segment::Leaf(|_, _| 0, |_, _, _| {}),          // CGB: Object Priority Mode
    _ => &UNKNOWN,
});
