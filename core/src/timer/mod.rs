pub mod divider;

use crate::interrupt::{Interrupt, InterruptController};
use crate::util::bits::Bits;
use divider::Divider;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct InputClock {
    ticks: u16,
}

impl InputClock {
    pub fn is_input(&self, divider: &Divider) -> bool {
        divider.counter() & (self.ticks - 1) == 0x00
    }
}

const INPUT_CLOCKS: [InputClock; 4] = [
    InputClock { ticks: 1024 },
    InputClock { ticks: 16 },
    InputClock { ticks: 64 },
    InputClock { ticks: 256 },
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Timer {
    divider: Divider,
    is_enabled: bool,
    input_clock_index: u8,
    counter: u8,
    modulo: u8,
}

impl Timer {
    fn input_clock(&self) -> InputClock {
        INPUT_CLOCKS[self.input_clock_index as usize]
    }

    pub fn tick(&mut self, interrupt_controller: &mut InterruptController) {
        self.divider.tick();
        if self.input_clock().is_input(&self.divider) {
            let (next, overflow) = self.counter.overflowing_add(1);
            if overflow {
                self.counter = self.modulo;
                interrupt_controller.request(Interrupt::Timer);
            } else {
                self.counter = next;
            }
        }
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
        self.modulo = value
    }

    pub fn control_bits(&self) -> u8 {
        (self.is_enabled as u8) << 2 | self.input_clock_index & 0b11
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.is_enabled = value.bit(2);
        self.input_clock_index = value & 0b11;
    }
}
