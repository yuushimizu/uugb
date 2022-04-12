use super::{
    Segment::{self, *},
    UNKNOWN,
};

pub const VRAM: Segment = Offset(
    0x8000,
    &Leaf(
        |components, address| components.ppu.vram().read(address),
        |components, address, value| components.ppu.vram_mut().write(address, value),
    ),
);

pub const OAM: Segment = Offset(
    0xFE00,
    &Leaf(
        |components, address| components.ppu.oam().read(address),
        |components, address, value| components.ppu.oam_mut().write(address, value),
    ),
);

pub const PPU: Segment = Nested(|address| match address {
    0xFF40 => &Leaf(
        |components, _| components.ppu.control_bits(),
        |components, _, value| components.ppu.set_control_bits(value),
    ),
    0xFF41 => &Leaf(
        |components, _| components.ppu.status_bits(),
        |components, _, value| components.ppu.set_status_bits(value),
    ),
    0xFF42 => &Leaf(
        |components, _| components.ppu.scroll_position().y,
        |components, _, value| components.ppu.scroll_position_mut().y = value,
    ),
    0xFF43 => &Leaf(
        |components, _| components.ppu.scroll_position().x,
        |components, _, value| components.ppu.scroll_position_mut().x = value,
    ),
    0xFF44 => &Leaf(|components, _| components.ppu.current_y(), |_, _, _| {}),
    0xFF45 => &Leaf(
        |components, _| components.ppu.y_compare(),
        |components, _, value| components.ppu.set_y_compare(value),
    ),
    0xFF46 => &Leaf(
        |components, _| components.ppu.oam().dma_source_address_upper(),
        |components, _, value| components.ppu.oam_mut().request_dma(value, components.dma),
    ), // DMA
    0xFF47 => &Leaf(
        |components, _| components.ppu.background_palette().bits(),
        |components, _, value| components.ppu.background_palette_mut().set_bits(value),
    ),
    0xFF48 => &Leaf(
        |components, _| components.ppu.object_palette0().bits(),
        |components, _, value| components.ppu.object_palette0_mut().set_bits(value),
    ),
    0xFF49 => &Leaf(
        |components, _| components.ppu.object_palette1().bits(),
        |components, _, value| components.ppu.object_palette1_mut().set_bits(value),
    ),
    0xFF4A => &Leaf(
        |components, _| components.ppu.window_position().y,
        |components, _, value| components.ppu.window_position_mut().y = value,
    ),
    0xFF4B => &Leaf(
        |components, _| components.ppu.window_position().x,
        |components, _, value| components.ppu.window_position_mut().x = value,
    ),
    0xFF4C => &Leaf(|_, _| 0, |_, _, _| {}),
    0xFF4D => &Leaf(|_, _| 0, |_, _, _| {}), // CGB: Prepare Speed Switch
    0xFF4E => &Leaf(|_, _| 0, |_, _, _| {}),
    0xFF4F => &Leaf(|_, _| 0, |_, _, _| {}), // CGB: VRAM Bank
    0xFF50 => &UNKNOWN,
    0xFF51..=0xFF55 => &Leaf(|_, _| 0, |_, _, _| {}), // CGB: DMA
    0xFF68..=0xFF6B => &Leaf(|_, _| 0, |_, _, _| {}), // CGB: Color Palettes
    0xFF6C => &Leaf(|_, _| 0, |_, _, _| {}),          // CGB: Object Priority Mode
    _ => &UNKNOWN,
});
