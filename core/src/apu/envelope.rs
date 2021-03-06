use crate::util::bits::Bits;

const STEP_UNIT_CYCLES: u64 = super::SAMPLE_RATE / 64;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Envelop {
    bits: u8,
    volume: u8,
    increases: bool,
    step_length: u8,
    cycles: u64,
}

impl Envelop {
    fn is_enabled(&self) -> bool {
        self.bits & 0b1111_1000 != 0
    }
    pub fn volume(&self) -> u8 {
        if self.is_enabled() {
            self.volume
        } else {
            0
        }
    }

    pub fn tick(&mut self) {
        if self.step_length == 0 {
            return;
        }
        self.cycles += 1;
        if self.cycles >= self.step_length as u64 * STEP_UNIT_CYCLES {
            self.cycles = 0;
            self.volume = if self.increases {
                std::cmp::min(0xF, self.volume() + 1)
            } else {
                self.volume.saturating_sub(1)
            };
        }
    }

    pub fn restart(&mut self) {
        self.volume = self.bits >> 4;
        self.increases = self.bits.bit(3);
        self.step_length = self.bits & 0b111;
        self.cycles = 0;
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn set_bits(&mut self, value: u8) {
        self.bits = value;
    }
}
