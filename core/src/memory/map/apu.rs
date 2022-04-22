use super::Segment;

pub const APU: Segment = Segment::Nested(|address| match address {
    0xFF10 => &Segment::Leaf(
        |components, _| components.apu.rect_wave1().sweep_bits(),
        |components, _, value| components.apu.rect_wave1_mut().set_sweep_bits(value),
    ),
    0xFF11 => &Segment::Leaf(
        |components, _| components.apu.rect_wave1().length_wave_bits(),
        |components, _, value| components.apu.rect_wave1_mut().set_length_wave_bits(value),
    ),
    0xFF12 => &Segment::Leaf(
        |components, _| components.apu.rect_wave1().envelop_bits(),
        |components, _, value| components.apu.rect_wave1_mut().set_envelop_bits(value),
    ),
    0xFF13 => &Segment::Leaf(
        |_, _| 0xFF,
        |components, _, value| {
            components
                .apu
                .rect_wave1_mut()
                .set_frequency_lower_bits(value)
        },
    ),
    0xFF14 => &Segment::Leaf(
        |components, _| components.apu.rect_wave1().frequency_upper_bits(),
        |components, _, value| {
            components
                .apu
                .rect_wave1_mut()
                .set_frequency_upper_bits(value)
        },
    ),
    0xFF16 => &Segment::Leaf(
        |components, _| components.apu.rect_wave2().length_wave_bits(),
        |components, _, value| components.apu.rect_wave2_mut().set_length_wave_bits(value),
    ),
    0xFF17 => &Segment::Leaf(
        |components, _| components.apu.rect_wave2().envelop_bits(),
        |components, _, value| components.apu.rect_wave2_mut().set_envelop_bits(value),
    ),
    0xFF18 => &Segment::Leaf(
        |_, _| 0xFF,
        |components, _, value| {
            components
                .apu
                .rect_wave2_mut()
                .set_frequency_lower_bits(value)
        },
    ),
    0xFF19 => &Segment::Leaf(
        |components, _| components.apu.rect_wave2().frequency_upper_bits(),
        |components, _, value| {
            components
                .apu
                .rect_wave2_mut()
                .set_frequency_upper_bits(value)
        },
    ),
    0xFF24 => &Segment::Leaf(
        |components, _| components.apu.channel_control_bits(),
        |components, _, value| components.apu.set_channel_control_bits(value),
    ),
    0xFF25 => &Segment::Leaf(
        |components, _| components.apu.output_terminal_selection_bits(),
        |components, _, value| components.apu.set_output_terminal_selection_bits(value),
    ),
    0xFF26 => &Segment::Leaf(
        |components, _| components.apu.enabled_bits(),
        |components, _, value| components.apu.set_enabled_bits(value),
    ),
    0xFF76 => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}), // CGB Register
    0xFF77 => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}), // CGB Register
    _ => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}),
});
