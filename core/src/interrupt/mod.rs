pub mod interrupt_controller;

pub use interrupt_controller::InterruptController;

use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interrupt {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    fn definition(&self) -> (u8, u8) {
        use Interrupt::*;
        match self {
            VBlank => (0, 0x40),
            LcdStat => (1, 0x48),
            Timer => (2, 0x50),
            Serial => (3, 0x58),
            Joypad => (4, 0x60),
        }
    }

    pub fn bit(&self) -> u8 {
        self.definition().0
    }

    pub fn address(&self) -> u8 {
        self.definition().1
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct State {
    enabled: bool,
    requested: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct States {
    states: [State; 5],
}

impl States {
    pub fn new(enabled_bits: u8, requested_bits: u8) -> Self {
        let mut states = States::default();
        states.set_enabled_bits(enabled_bits);
        states.set_requested_bits(requested_bits);
        states
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
        self.bits(|state| state.enabled)
    }

    pub fn set_enabled_bits(&mut self, bits: u8) {
        self.set_bits(|state, bit| state.enabled = bit, bits);
    }

    pub fn requested_bits(&self) -> u8 {
        self.bits(|state| state.requested)
    }

    pub fn set_requested_bits(&mut self, bits: u8) {
        self.set_bits(|state, bit| state.requested = bit, bits);
    }

    pub fn state(&self, interrupt: Interrupt) -> &State {
        &self.states[interrupt.bit() as usize]
    }

    pub fn state_mut(&mut self, interrupt: Interrupt) -> &mut State {
        &mut self.states[interrupt.bit() as usize]
    }
}
