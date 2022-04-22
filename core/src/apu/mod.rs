mod envelope;
mod length;
mod noise;
mod rect_wave;
mod sweep;

pub use rect_wave::SAMPLE_RATE;

use noise::Noise;
use rect_wave::RectWave;

use crate::util::bits::Bits;

pub trait AudioTerminal {
    fn output(&mut self, volume: (u8, u8));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TerminalControl {
    pub is_enabled: bool,
    pub level: u8,
}

impl Default for TerminalControl {
    fn default() -> Self {
        Self {
            is_enabled: false,
            level: 0b111,
        }
    }
}

impl TerminalControl {
    pub fn bits(&self) -> u8 {
        (self.is_enabled as u8) << 3 | self.level
    }

    pub fn set_bits(&mut self, value: u8) {
        self.is_enabled = value.bit(3);
        self.level = value & 0b111;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Apu {
    is_enabled: bool,
    left_control: TerminalControl,
    right_control: TerminalControl,
    output_terminal_selection: u8,
    rect_wave1: RectWave,
    rect_wave2: RectWave,
    noise: Noise,
}

impl Default for Apu {
    fn default() -> Self {
        let mut rect_wave1 = RectWave::default();
        rect_wave1.set_length_wave_bits(0xBF);
        rect_wave1.set_envelope_bits(0xF3);
        Self {
            is_enabled: true,
            left_control: Default::default(),
            right_control: Default::default(),
            output_terminal_selection: 0xF3,
            rect_wave1,
            rect_wave2: Default::default(),
            noise: Default::default(),
        }
    }
}

impl Apu {
    pub fn tick(&mut self, terminal: &mut impl AudioTerminal) {
        if self.is_enabled {
            self.rect_wave1.tick();
            self.rect_wave2.tick();
            self.noise.tick();
            terminal.output(self.output());
        } else {
            terminal.output((0, 0));
        }
    }

    fn output(&self) -> (u8, u8) {
        let outputs = [
            self.rect_wave1.output(),
            self.rect_wave2.output(),
            0,
            self.noise.output(),
        ];
        let mix = |offset: u32| {
            outputs
                .iter()
                .enumerate()
                .fold(0u8, |acc, (index, &output)| {
                    acc.saturating_add(
                        if self.output_terminal_selection.bit(index as u32 + offset) {
                            output / 4
                        } else {
                            0
                        },
                    )
                })
        };
        (mix(4), mix(0))
    }

    pub fn rect_wave1(&self) -> &RectWave {
        &self.rect_wave1
    }

    pub fn rect_wave1_mut(&mut self) -> &mut RectWave {
        &mut self.rect_wave1
    }

    pub fn rect_wave2(&self) -> &RectWave {
        &self.rect_wave2
    }

    pub fn rect_wave2_mut(&mut self) -> &mut RectWave {
        &mut self.rect_wave2
    }

    pub fn noise(&self) -> &Noise {
        &self.noise
    }

    pub fn noise_mut(&mut self) -> &mut Noise {
        &mut self.noise
    }

    pub fn channel_control_bits(&self) -> u8 {
        self.left_control.bits() << 4 | self.right_control.bits()
    }

    pub fn set_channel_control_bits(&mut self, value: u8) {
        self.left_control.set_bits(value >> 4);
        self.right_control.set_bits(value);
    }

    pub fn output_terminal_selection_bits(&self) -> u8 {
        self.output_terminal_selection
    }

    pub fn set_output_terminal_selection_bits(&mut self, value: u8) {
        self.output_terminal_selection = value;
    }

    pub fn enabled_bits(&self) -> u8 {
        u8::from_bits(&[
            self.is_enabled,
            true,
            true,
            true,
            self.noise.is_started(),
            false, // sound 3,
            self.rect_wave2.is_started(),
            self.rect_wave1.is_started(),
        ])
    }

    pub fn set_enabled_bits(&mut self, value: u8) {
        let current_enabled = self.is_enabled;
        self.is_enabled = value.bit(7);
        if !current_enabled && self.is_enabled {
            self.rect_wave1.reset();
            self.rect_wave2.reset();
        }
    }
}
