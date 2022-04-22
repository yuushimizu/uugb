const UNIT_CYCLES: u64 = 4096;

const MAX: u8 = 0b0100_0000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Length {
    is_enabled: bool,
    rest_cycles: u64,
}

impl Length {
    pub fn tick(&mut self) {
        self.rest_cycles = self.rest_cycles.saturating_sub(1);
    }

    pub fn is_expired(&self) -> bool {
        self.is_enabled && self.rest_cycles == 0
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn set_is_enabled(&mut self, value: bool) {
        self.is_enabled = value;
    }

    pub fn set(&mut self, value: u8) {
        self.rest_cycles = (MAX - (value & (MAX - 1))) as u64 * UNIT_CYCLES;
    }
}
