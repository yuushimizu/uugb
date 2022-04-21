const UNIT_CYCLES: u64 = 4096;

const MAX: u8 = 0b0100_0000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Length {
    rest_cycles: u64,
}

impl Length {
    pub fn tick(&mut self) {
        self.rest_cycles = self.rest_cycles.saturating_sub(1);
    }

    pub fn is_expired(&self) -> bool {
        self.rest_cycles == 0
    }

    pub fn set(&mut self, value: u8) {
        self.rest_cycles = (MAX - (value & (MAX - 1))) as u64 * UNIT_CYCLES;
    }
}
