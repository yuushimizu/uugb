use super::Segment;

pub const CARTRIDGE: Segment = Segment::Nested(|address| match address {
    0x0000..=0x7FFF => &Segment::Leaf(
        |components, address| components.cartridge.read_rom(address),
        |components, address, value| components.cartridge.write_rom(address, value),
    ),
    0xA000..=0xBFFF => &Segment::Offset(
        0xA000,
        &Segment::Leaf(
            |components, address| components.cartridge.read_ram(address),
            |components, address, value| components.cartridge.write_ram(address, value),
        ),
    ),
    _ => unreachable!(),
});
