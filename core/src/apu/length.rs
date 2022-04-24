const UNIT_CYCLES: u64 = super::SAMPLE_RATE / 256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Length {
    max: u16,
    is_enabled: bool,
    rest_cycles: u64,
}

impl Length {
    pub fn new(max: u16) -> Self {
        Self {
            max,
            is_enabled: false,
            rest_cycles: 0,
        }
    }

    pub fn restart(&mut self) {
        if self.rest_cycles == 0 {
            self.rest_cycles = self.max as u64 * UNIT_CYCLES;
        }
    }

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
        self.rest_cycles = (self.max - ((value as u16) & (self.max - 1))) as u64 * UNIT_CYCLES;
    }
}
