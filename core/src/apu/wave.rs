use super::length::Length;
use crate::util::bits::Bits;

const PATTERN_FRAME_COUNT: usize = 32;

const RAM_SIZE: usize = PATTERN_FRAME_COUNT / 2;

const MAX_FREQUENCY: u16 = 2048;

const FREQUENCY_UNIT: u64 = 32;

const STEP_LENGTH_UNIT: u64 =
    super::SAMPLE_RATE / (MAX_FREQUENCY as u64 * FREQUENCY_UNIT * PATTERN_FRAME_COUNT as u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wave {
    is_enabled: bool,
    is_started: bool,
    length: Length,
    level: u8,
    pattern: [u8; RAM_SIZE],
    step: usize,
    frequency: u16,
    cycles: u64,
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            is_enabled: false,
            is_started: false,
            length: Length::new(256),
            level: 0,
            pattern: Default::default(),
            step: 0,
            frequency: 0,
            cycles: 0,
        }
    }
}

impl Wave {
    fn start(&mut self) {
        self.is_started = true;
        self.step = 0;
        self.length.restart();
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn step_length_cycles(&self) -> u64 {
        ((MAX_FREQUENCY - self.frequency) as u64) * STEP_LENGTH_UNIT
    }

    pub fn tick(&mut self) {
        if !self.is_enabled || !self.is_started {
            return;
        }
        self.cycles += 1;
        if self.cycles >= self.step_length_cycles() {
            self.cycles = 0;
            self.step = (self.step + 1) % PATTERN_FRAME_COUNT
        }
        self.length.tick();
        if self.length.is_expired() {
            self.is_started = false;
        }
    }

    pub fn output(&self) -> u8 {
        if self.is_enabled && self.is_started {
            let byte = self.pattern[self.step / 2];
            let frame = if self.step % 2 == 0 {
                byte >> 4
            } else {
                byte & 0xF
            };
            match self.level {
                0 => 0,
                level => frame >> (level - 1),
            }
        } else {
            0
        }
    }

    pub fn enabled_bits(&self) -> u8 {
        (self.is_enabled as u8) << 7 | 0b0111_1111
    }

    pub fn set_enabled_bits(&mut self, value: u8) {
        self.is_enabled = value.bit(7);
    }

    pub fn set_length(&mut self, value: u8) {
        self.length.set(value);
    }

    pub fn level_bits(&self) -> u8 {
        self.level << 5 | 0b1001_1111
    }

    pub fn set_level_bits(&mut self, value: u8) {
        self.level = (value >> 5) & 0b11;
    }

    pub fn set_frequency_lower_bits(&mut self, value: u8) {
        self.frequency = (self.frequency & 0xFF00) | value as u16;
    }

    pub fn frequency_upper_bits(&self) -> u8 {
        (self.length.is_enabled() as u8) << 6 | 0b1011_1111
    }

    pub fn set_frequency_upper_bits(&mut self, value: u8) {
        self.frequency = (self.frequency & 0xFF) | ((value as u16) & 0b111) << 8;
        self.length.set_is_enabled(value.bit(6));
        if value.bit(7) {
            self.start();
        }
    }

    pub fn pattern(&self) -> &[u8] {
        &self.pattern
    }

    pub fn pattern_mut(&mut self) -> &mut [u8] {
        &mut self.pattern
    }
}
