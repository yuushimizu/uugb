use super::length::Length;
use crate::util::bits::Bits;

const DUTY_CYCLE_LENGTH: usize = 8;

const FREQUENCY_UNIT_CYCLES: u64 = 64;

const MAX_FREQUENCY: u16 = 2048;

pub const SAMPLE_RATE: u64 =
    DUTY_CYCLE_LENGTH as u64 * MAX_FREQUENCY as u64 * FREQUENCY_UNIT_CYCLES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DutyCycle {
    bits: u8,
}

impl Default for DutyCycle {
    fn default() -> Self {
        Self { bits: 0b10 }
    }
}

impl From<u8> for DutyCycle {
    fn from(value: u8) -> Self {
        Self { bits: value & 0b11 }
    }
}

impl From<DutyCycle> for u8 {
    fn from(value: DutyCycle) -> Self {
        value.bits
    }
}

impl DutyCycle {
    pub fn bits(&self) -> u8 {
        self.bits
    }

    fn pattern(&self) -> &[bool; DUTY_CYCLE_LENGTH] {
        match self.bits {
            0b00 => &[false, true, false, false, false, false, false, false],
            0b01 => &[false, true, true, false, false, false, false, false],
            0b10 => &[false, true, true, true, true, false, false, false],
            0b11 => &[false, true, true, true, true, true, true, false],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RectWave {
    duty_cycle: DutyCycle,
    duty_cycle_step: usize,
    length: Length,
    frequency: u16,
    cycles: u16,
}

impl RectWave {
    fn initialize(&mut self) {}

    pub fn tick(&mut self) {
        self.cycles = self.cycles + 1;
        if self.cycles >= self.step_length_cycles() {
            self.cycles = 0;
            self.duty_cycle_step = (self.duty_cycle_step + 1) % DUTY_CYCLE_LENGTH;
        }
        self.length.tick();
    }

    pub fn output(&self) -> u8 {
        if self.length.is_expired() {
            0
        } else {
            (self.duty_cycle.pattern()[self.duty_cycle_step] as u8) * 64
        }
    }

    fn step_length_cycles(&self) -> u16 {
        MAX_FREQUENCY - self.frequency
    }

    pub fn sweep_bits(&self) -> u8 {
        0xFF
    }

    pub fn set_sweep_bits(&mut self, value: u8) {}

    pub fn length_wave_bits(&self) -> u8 {
        self.duty_cycle.bits() << 6 | 0b0011_1111
    }

    pub fn set_length_wave_bits(&mut self, value: u8) {
        self.duty_cycle = (value >> 6).into();
        self.length.set(value);
    }

    pub fn envelop_bits(&self) -> u8 {
        0xFF
    }

    pub fn set_envelop_bits(&mut self, value: u8) {}

    pub fn set_frequency_lower_bits(&mut self, value: u8) {
        self.frequency = (self.frequency & 0xFF00) | value as u16;
    }

    pub fn frequency_upper_bits(&self) -> u8 {
        0b1011_1111 | (self.length.is_enabled() as u8) << 6
    }

    pub fn set_frequency_upper_bits(&mut self, value: u8) {
        self.frequency = (self.frequency & 0xFF) | (value as u16 & 0b111) << 8;
        self.length.set_is_enabled(value.bit(6));
        if value.bit(7) {
            self.initialize();
        }
    }
}
