use super::{Segment, UNKNOWN};

pub const SERIAL: Segment = Segment::Nested(|address| match address {
    0xFF01 => &Segment::Leaf(
        |components, _| components.serial.data(),
        |components, _, value| components.serial.set_data(value),
    ),
    0xFF02 => &Segment::Leaf(
        |components, _| components.serial.control_bits(),
        |components, _, value| components.serial.set_control_bits(value),
    ),
    _ => &UNKNOWN,
});
