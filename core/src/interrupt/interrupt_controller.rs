use super::Interrupt;
use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct State {
    is_enabled: bool,
    is_requested: bool,
}

impl State {
    fn is_pending(&self) -> bool {
        self.is_enabled && self.is_requested
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InterruptController {
    states: [State; Interrupt::ORDERED.len()],
}

impl Default for InterruptController {
    fn default() -> Self {
        let mut result = Self {
            states: Default::default(),
        };
        result.request(Interrupt::VBlank);
        result
    }
}

impl Interrupt {
    fn bit(&self) -> u8 {
        Self::ORDERED.iter().position(|x| x == self).unwrap() as u8
    }
}

impl InterruptController {
    fn state(&self, interrupt: Interrupt) -> &State {
        &self.states[interrupt.bit() as usize]
    }

    fn state_mut(&mut self, interrupt: Interrupt) -> &mut State {
        &mut self.states[interrupt.bit() as usize]
    }

    pub fn request(&mut self, interrupt: Interrupt) {
        self.state_mut(interrupt).is_requested = true;
    }

    pub fn clear(&mut self, interrupt: Interrupt) {
        self.state_mut(interrupt).is_requested = false;
    }

    pub fn clear_all(&mut self) {
        for state in self.states.iter_mut() {
            state.is_requested = false;
        }
    }

    pub fn pending_interrupt(&self) -> Option<Interrupt> {
        Interrupt::ORDERED
            .iter()
            .find(|&interrupt| self.state(*interrupt).is_pending())
            .copied()
    }

    fn bits(&self, f: impl Fn(&State) -> bool) -> u8 {
        0xFF << self.states.len()
            | self
                .states
                .iter()
                .enumerate()
                .fold(0x00, |acc, (bit, state)| acc | (f(state) as u8) << bit)
    }

    fn set_bits(&mut self, f: impl Fn(&mut State, bool), bits: u8) {
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
