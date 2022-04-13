mod control;
mod divider;

use control::Control;
pub use divider::Divider;

use crate::interrupt::{Interrupt, InterruptController};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Timer {
    control: Control,
    counter: u8,
    modulo: u8,
    previous_output: bool,
    overflow: bool,
    is_reloading: bool,
}

impl Timer {
    pub fn tick(&mut self, divider: &Divider, interrupt_controller: &mut InterruptController) {
        self.is_reloading = false;
        if self.overflow {
            self.overflow = false;
            self.counter = self.modulo;
            interrupt_controller.request(Interrupt::Timer);
            self.is_reloading = true;
        }
        let output = self.control.is_enabled()
            && (divider.counter() & self.control.input_clock().bit_mask() != 0);
        if self.previous_output && !output {
            (self.counter, self.overflow) = self.counter.overflowing_add(1);
        }
        self.previous_output = output;
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn set_counter(&mut self, value: u8) {
        if !self.is_reloading {
            self.counter = value;
        }
    }

    pub fn modulo(&self) -> u8 {
        self.modulo
    }

    pub fn set_modulo(&mut self, value: u8) {
        self.modulo = value;
        if self.is_reloading {
            self.counter = value;
        }
    }

    pub fn control_bits(&self) -> u8 {
        self.control.bits()
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.control.set_bits(value);
    }
}
