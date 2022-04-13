use super::Segment;

pub const TIMER: Segment = Segment::Nested(|address| match address {
    0xFF04 => &Segment::Leaf(
        |components, _| components.divider.register(),
        |components, _, _| components.divider.reset(),
    ),
    0xFF05 => &Segment::Leaf(
        |components, _| components.timer.counter(),
        |components, _, value| components.timer.set_counter(value),
    ),
    0xFF06 => &Segment::Leaf(
        |components, _| components.timer.modulo(),
        |components, _, value| components.timer.set_modulo(value),
    ),
    0xFF07 => &Segment::Leaf(
        |components, _| components.timer.control_bits(),
        |components, _, value| components.timer.set_control_bits(value),
    ),
    _ => unreachable!(),
});
