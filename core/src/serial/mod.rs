pub mod connection;

pub use connection::*;

use crate::{
    interrupt::{Interrupt, InterruptController},
    util::bits::Bits,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Serial {
    buffer: u8,
    is_started: bool,
    is_completed: bool,
    is_fast: bool,
    uses_internal_clock: bool,
    transfered_bits: u8,
    rest_cycles: u64,
}

impl Serial {
    fn cycles(&self) -> u64 {
        if self.is_fast {
            16
        } else {
            512
        }
    }

    fn start(&mut self) {
        self.is_started = true;
        self.is_completed = false;
        self.transfered_bits = 0;
        self.rest_cycles = self.cycles();
    }

    fn transfer(&mut self, connection: &mut impl SerialConnection) {
        let input = connection.send();
        let output = self.buffer.bit(7);
        self.buffer = self.buffer << 1 | (input as u8);
        connection.receive(output);
        self.transfered_bits += 1;
        if self.transfered_bits >= 8 {
            self.is_started = false;
            self.is_completed = true;
        }
    }

    pub fn tick(
        &mut self,
        interrupt_controller: &mut InterruptController,
        connection: &mut impl SerialConnection,
    ) {
        self.rest_cycles = self.rest_cycles.saturating_sub(1);
        if self.rest_cycles > 0 {
            return;
        }
        self.rest_cycles = self.cycles();
        if self.is_completed {
            self.is_completed = false;
            interrupt_controller.request(Interrupt::Serial);
        }
        if self.is_started {
            self.transfer(connection);
        }
    }

    pub fn data(&self) -> u8 {
        self.buffer
    }

    pub fn set_data(&mut self, value: u8) {
        self.buffer = value;
    }

    pub fn control_bits(&self) -> u8 {
        (self.is_started as u8) << 7 | (self.is_fast as u8) << 1 | self.uses_internal_clock as u8
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.is_fast = value.bit(1);
        self.uses_internal_clock = value.bit(0);
        if value.bit(7) {
            self.start();
        }
    }

    pub fn receive(&mut self, connection: &mut impl SerialConnection) {
        if !self.uses_internal_clock {
            if !self.is_started {
                self.start();
            }
            self.transfer(connection);
        }
    }
}
