use super::Segment;

pub const CARTRIDGE: Segment = Segment::Leaf(
    |components, address| components.cartridge.read(address),
    |components, address, value| components.cartridge.write(address, value),
);
