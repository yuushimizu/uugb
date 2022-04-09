use super::Segment;

pub const UNKNOWN: Segment = Segment::Leaf(
    |_, address| {
        log::warn!("Attempt to read from the unknown segment: {:04X}", address);
        0xFF
    },
    |_, address, _| {
        log::warn!("Attempt to write to the unknown segment: {:04X}", address);
    },
);
