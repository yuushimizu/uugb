use super::Segment;

pub const JOYPAD: Segment = Segment::Leaf(
    |components, _| components.joypad.bits(),
    |components, _, value| {
        components
            .joypad
            .set_bits(value, components.interrupt_controller)
    },
);
