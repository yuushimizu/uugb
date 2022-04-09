use super::Segment;

pub const INTERRUPT_REQUESTED: Segment = Segment::Leaf(
    |components, _| components.interrupt_controller.requested_bits(),
    |components, _, value| components.interrupt_controller.set_requested_bits(value),
);

pub const INTERRUPT_ENABLED: Segment = Segment::Leaf(
    |components, _| components.interrupt_controller.enabled_bits(),
    |components, _, value| components.interrupt_controller.set_enabled_bits(value),
);
