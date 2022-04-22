use crate::util::bits::Bits;

const STEP_UNIT_CYCLES: u64 = super::SAMPLE_RATE / 128;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Sweep {
    step_length: u8,
    decreases: bool,
    shift_amount: u8,
    cycles: u64,
}

impl Sweep {
    pub fn tick(&mut self, frequency: u16) -> u16 {
        if self.step_length == 0 {
            return frequency;
        }
        self.cycles += 1;
        if self.cycles >= self.step_length as u64 * STEP_UNIT_CYCLES {
            self.cycles = 0;
            if self.decreases {
                frequency.saturating_sub(frequency >> self.shift_amount)
            } else {
                frequency.saturating_add(frequency >> self.shift_amount)
            }
        } else {
            frequency
        }
    }

    pub fn restart(&mut self) {
        self.cycles = 0;
    }

    pub fn bits(&self) -> u8 {
        0b1 << 7 | self.step_length << 4 | (self.decreases as u8) << 3 | self.shift_amount
    }

    pub fn set_bits(&mut self, value: u8) {
        self.step_length = value >> 4;
        self.decreases = value.bit(3);
        self.shift_amount = value & 0b111;
    }
}
