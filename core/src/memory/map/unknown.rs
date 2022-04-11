use super::Segment;

pub const UNKNOWN: Segment = Segment::Leaf(
    |_, address| {
        log::debug!("Attempt to read from the unknown segment: {:04X}", address);
        0xFF
    },
    |_, address, _| {
        log::debug!("Attempt to write to the unknown segment: {:04X}", address);
    },
);
