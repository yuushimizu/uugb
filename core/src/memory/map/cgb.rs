use super::Segment;

pub const CGB_REGISTERS: Segment = Segment::Nested(|address| match address {
    0xFF4D => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}),
    _ => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}),
});
