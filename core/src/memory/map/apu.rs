use super::Segment;

pub const APU: Segment = Segment::Leaf(|_, _| 0xFF, |_, _, _| {});
