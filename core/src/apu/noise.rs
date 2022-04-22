use super::envelope::Envelop;
use super::length::Length;
use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Noise {
    is_started: bool,
    random: u16,
    output: bool,
    length: Length,
    envelope: Envelop,
    frequency_shift: u8,
    is_short: bool,
    division_ratio: u8,
    cycles: u64,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            is_started: false,
            random: 0xFFFF,
            output: false,
            length: Length::new(64),
            envelope: Default::default(),
            frequency_shift: 0,
            is_short: false,
            division_ratio: 0,
            cycles: 0,
        }
    }
}

impl Noise {
    pub fn is_started(&self) -> bool {
        self.is_started
    }

    fn start(&mut self) {
        self.is_started = true;
        self.random = 0xFFFF;
        self.output = false;
        self.cycles = 0;
        self.envelope.restart();
    }

    fn step_length(&self) -> u64 {
        (match self.division_ratio {
            0 => 1,
            n => 2 * n as u64,
        }) << (1 + self.frequency_shift)
    }

    fn step_width(&self) -> u32 {
        if self.is_short {
            7
        } else {
            15
        }
    }

    pub fn tick(&mut self) {
        if !self.is_started {
            return;
        }
        self.cycles += 1;
        if self.cycles >= self.step_length() {
            self.cycles = 0;
            if self.random == 0 {
                self.random = 1;
            }
            self.random += self.random
                + (((self.random >> (self.step_width() - 1))
                    ^ (self.random >> (self.step_width() - 2)))
                    & 1);
            self.output ^= (self.random & 0b1) == 0b1;
        }
        self.length.tick();
        self.envelope.tick();
        if self.length.is_expired() {
            self.is_started = false;
        }
    }

    pub fn output(&self) -> u8 {
        if self.is_started {
            (self.output as u8) * self.envelope.volume()
        } else {
            0x00
        }
    }

    pub fn set_length(&mut self, value: u8) {
        self.length.set(value);
    }

    pub fn envelope_bits(&self) -> u8 {
        self.envelope.bits()
    }

    pub fn set_envelope_bits(&mut self, value: u8) {
        self.envelope.set_bits(value);
    }

    pub fn frequency_bits(&self) -> u8 {
        self.frequency_shift << 4 | (self.is_short as u8) << 3 | self.division_ratio
    }

    pub fn set_frequency_bits(&mut self, value: u8) {
        self.frequency_shift = value >> 4;
        self.is_short = value.bit(3);
        self.division_ratio = value & 0b111;
    }

    pub fn control_bits(&self) -> u8 {
        0b1011_1111 | (self.length.is_enabled() as u8) << 6
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.length.set_is_enabled(value.bit(6));
        if value.bit(7) {
            self.start();
        }
    }
}
