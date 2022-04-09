use super::Segment;

pub const UNUSABLE: Segment = Segment::Leaf(|_, _| 0xFF, |_, _, _| {});
