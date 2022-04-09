use super::Segment;

pub const HRAM: Segment = Segment::Offset(
    0xFF80,
    &Segment::Leaf(
        |components, address| components.hram.read(address),
        |components, address, value| {
            components.hram.write(address, value);
        },
    ),
);
