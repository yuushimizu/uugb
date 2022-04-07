use super::Interrupt;
use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct State {
    is_enabled: bool,
    is_requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InterruptController {
    states: [State; Interrupt::ORDERED.len()],
}

impl Interrupt {
    fn bit(&self) -> u8 {
        Self::ORDERED.iter().position(|x| x == self).unwrap() as u8
    }
}

impl InterruptController {
    fn state_mut(&mut self, interrupt: Interrupt) -> &mut State {
        &mut self.states[interrupt.bit() as usize]
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        self.state_mut(interrupt).is_requested = true;
    }

    pub fn clear(&mut self, interrupt: Interrupt) {
        self.state_mut(interrupt).is_requested = false;
    }

    fn bits<F: Fn(&State) -> bool>(&self, f: F) -> u8 {
        self.states
            .iter()
            .enumerate()
            .fold(0x00, |acc, (bit, state)| acc | (f(state) as u8) << bit)
    }

    fn set_bits<F: Fn(&mut State, bool)>(&mut self, f: F, bits: u8) {
        for (bit, state) in self.states.iter_mut().enumerate() {
            f(state, bits.bit(bit as u32))
        }
    }

    pub fn enabled_bits(&self) -> u8 {
        self.bits(|state| state.is_enabled)
    }

    pub fn set_enabled_bits(&mut self, bits: u8) {
        self.set_bits(|state, bit| state.is_enabled = bit, bits);
    }

    pub fn requested_bits(&self) -> u8 {
        self.bits(|state| state.is_requested)
    }

    pub fn set_requested_bits(&mut self, bits: u8) {
        self.set_bits(|state, bit| state.is_requested = bit, bits);
    }
}
