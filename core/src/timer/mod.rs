mod control;
mod divider;

use control::Control;
use divider::Divider;

use crate::interrupt::{Interrupt, InterruptController};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Timer {
    divider: Divider,
    control: Control,
    counter: u8,
    modulo: u8,
    previous_output: bool,
}

impl Timer {
    pub fn tick(&mut self, interrupt_controller: &mut InterruptController) {
        self.divider.tick();
        let output = self.control.is_enabled()
            && (self.divider.counter() & self.control.input_clock().bit_mask() != 0);
        if self.previous_output && !output {
            if self.counter == 0xFF {
                self.counter = self.modulo;
                interrupt_controller.request(Interrupt::Timer);
            } else {
                self.counter = self.counter.wrapping_add(1);
            }
        }
        self.previous_output = output;
    }

    pub fn divider_register(&self) -> u8 {
        self.divider.register()
    }

    pub fn reset_divider(&mut self) {
        self.divider.reset();
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn set_counter(&mut self, value: u8) {
        self.counter = value;
    }

    pub fn modulo(&self) -> u8 {
        self.modulo
    }

    pub fn set_modulo(&mut self, value: u8) {
        self.modulo = value;
    }

    pub fn control_bits(&self) -> u8 {
        self.control.bits()
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.control.set_bits(value);
    }
}
