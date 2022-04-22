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
        |components, _| components.apu.rect_wave1().envelope_bits(),
        |components, _, value| components.apu.rect_wave1_mut().set_envelope_bits(value),
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
        |components, _| components.apu.rect_wave2().envelope_bits(),
        |components, _, value| components.apu.rect_wave2_mut().set_envelope_bits(value),
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
    0xFF1A => &Segment::Leaf(
        |components, _| components.apu.wave().enabled_bits(),
        |components, _, value| components.apu.wave_mut().set_enabled_bits(value),
    ),
    0xFF1B => &Segment::Leaf(
        |_, _| 0xFF,
        |components, _, value| components.apu.wave_mut().set_length(value),
    ),
    0xFF1C => &Segment::Leaf(
        |components, _| components.apu.wave().level_bits(),
        |components, _, value| components.apu.wave_mut().set_level_bits(value),
    ),
    0xFF1D => &Segment::Leaf(
        |_, _| 0xFF,
        |components, _, value| components.apu.wave_mut().set_frequency_lower_bits(value),
    ),
    0xFF1E => &Segment::Leaf(
        |components, _| components.apu.wave().frequency_upper_bits(),
        |components, _, value| components.apu.wave_mut().set_frequency_upper_bits(value),
    ),
    0xFF20 => &Segment::Leaf(
        |_, _| 0xFF,
        |components, _, value| components.apu.noise_mut().set_length(value),
    ),
    0xFF21 => &Segment::Leaf(
        |components, _| components.apu.noise().envelope_bits(),
        |components, _, value| components.apu.noise_mut().set_envelope_bits(value),
    ),
    0xFF22 => &Segment::Leaf(
        |components, _| components.apu.noise().frequency_bits(),
        |components, _, value| components.apu.noise_mut().set_frequency_bits(value),
    ),
    0xFF23 => &Segment::Leaf(
        |components, _| components.apu.noise().control_bits(),
        |components, _, value| components.apu.noise_mut().set_control_bits(value),
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
    0xFF30..=0xFF3F => &Segment::Offset(
        0xFF30,
        &Segment::Leaf(
            |components, address| components.apu.wave().pattern()[address as usize],
            |components, address, value| {
                components.apu.wave_mut().pattern_mut()[address as usize] = value
            },
        ),
    ),
    0xFF76 => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}), // CGB Register
    0xFF77 => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}), // CGB Register
    _ => &Segment::Leaf(|_, _| 0xFF, |_, _, _| {}),
});
