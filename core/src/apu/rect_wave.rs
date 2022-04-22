use super::envelope::Envelop;
use super::length::Length;
use super::sweep::Sweep;
use crate::util::bits::Bits;

const DUTY_CYCLE_LENGTH: usize = 8;

const FREQUENCY_UNIT_CYCLES: u64 = 64;

const MAX_LIMIT: u16 = 2048;

pub const SAMPLE_RATE: u64 = DUTY_CYCLE_LENGTH as u64 * MAX_LIMIT as u64 * FREQUENCY_UNIT_CYCLES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DutyCycle {
    bits: u8,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RectWave {
    is_started: bool,
    duty_cycle: DutyCycle,
    duty_cycle_step: usize,
    length: Length,
    frequency: u16,
    sweep: Sweep,
    envelope: Envelop,
    cycles: u16,
}

impl Default for RectWave {
    fn default() -> Self {
        Self {
            is_started: false,
            duty_cycle: Default::default(),
            duty_cycle_step: 0,
            length: Length::new(64),
            frequency: 0,
            sweep: Default::default(),
            envelope: Default::default(),
            cycles: 0,
        }
    }
}

impl RectWave {
    fn start(&mut self) {
        self.is_started = true;
        self.sweep.restart();
        self.envelope.restart();
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    fn step_length_cycles(&self) -> u16 {
        MAX_LIMIT - self.frequency
    }

    pub fn tick(&mut self) {
        if !self.is_started {
            return;
        }
        self.cycles += 1;
        if self.cycles >= self.step_length_cycles() {
            self.cycles = 0;
            self.duty_cycle_step = (self.duty_cycle_step + 1) % DUTY_CYCLE_LENGTH;
        }
        self.length.tick();
        self.frequency = self.sweep.tick(self.frequency);
        self.envelope.tick();
        if self.length.is_expired() || self.frequency >= MAX_LIMIT {
            self.is_started = false;
        }
        if self.frequency >= MAX_LIMIT {
            self.frequency = MAX_LIMIT - 1;
        }
    }

    pub fn output(&self) -> u8 {
        if self.is_started {
            (self.duty_cycle.pattern()[self.duty_cycle_step] as u8) * self.envelope.volume()
        } else {
            0
        }
    }

    pub fn reset(&mut self) {
        self.is_started = false;
        self.cycles = 0;
        self.duty_cycle_step = 0;
    }

    pub fn sweep_bits(&self) -> u8 {
        self.sweep.bits()
    }

    pub fn set_sweep_bits(&mut self, value: u8) {
        self.sweep.set_bits(value);
    }

    pub fn length_wave_bits(&self) -> u8 {
        self.duty_cycle.bits() << 6 | 0b0011_1111
    }

    pub fn set_length_wave_bits(&mut self, value: u8) {
        self.duty_cycle = (value >> 6).into();
        self.length.set(value);
    }

    pub fn envelope_bits(&self) -> u8 {
        self.envelope.bits()
    }

    pub fn set_envelope_bits(&mut self, value: u8) {
        self.envelope.set_bits(value)
    }

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
            self.start();
        }
    }
}
