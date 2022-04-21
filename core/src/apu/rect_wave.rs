use super::length::Length;

const DUTY_CYCLE_LENGTH: usize = 8;

const FREQUENCY_UNIT_CYCLES: u64 = 64;

const MAX_FREQUENCY: u16 = 2048;

pub const SAMPLE_RATE: u64 =
    DUTY_CYCLE_LENGTH as u64 * MAX_FREQUENCY as u64 * FREQUENCY_UNIT_CYCLES;

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
    supports_sweep: bool,
    duty_cycle: DutyCycle,
    duty_cycle_step: usize,
    length: Length,
    frequency: u16,
    cycles: u16,
}

impl RectWave {
    pub fn new(supports_sweep: bool) -> Self {
        Self {
            supports_sweep,
            duty_cycle: Default::default(),
            duty_cycle_step: 0,
            length: Default::default(),
            frequency: 0,
            cycles: 0,
        }
    }

    pub fn duty_cycle(&self) -> DutyCycle {
        self.duty_cycle
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: DutyCycle) {
        self.duty_cycle = duty_cycle;
    }

    pub fn set_length(&mut self, value: u8) {
        self.length.set(value)
    }

    pub fn frequency(&self) -> u16 {
        self.frequency
    }

    pub fn set_frequency(&mut self, value: u16) {
        self.frequency = value & (MAX_FREQUENCY - 1);
    }

    fn step_length_cycles(&self) -> u16 {
        MAX_FREQUENCY - self.frequency
    }

    pub fn tick(&mut self) {
        self.cycles = self.cycles + 1;
        if self.cycles >= self.step_length_cycles() {
            self.cycles = 0;
            self.duty_cycle_step = (self.duty_cycle_step + 1) % DUTY_CYCLE_LENGTH;
        }
        self.length.tick();
    }

    pub fn output(&self) -> (u8, u8) {
        if self.length.is_expired() {
            (0, 0)
        } else {
            (
                0,
                (self.duty_cycle.pattern()[self.duty_cycle_step] as u8) * 128,
            )
        }
    }
}
