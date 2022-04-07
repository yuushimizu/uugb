use super::{Interrupt, States};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InterruptController {
    states: States,
}

impl InterruptController {
    pub fn enabled_bits(&self) -> u8 {
        self.states.enabled_bits()
    }

    pub fn set_enabled_bits(&mut self, bits: u8) {
        self.states.set_enabled_bits(bits)
    }

    pub fn requested_bits(&self) -> u8 {
        self.states.requested_bits()
    }

    pub fn set_requested_bits(&mut self, bits: u8) {
        self.states.set_requested_bits(bits)
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        self.states.state_mut(interrupt).is_requested = true;
    }
}
