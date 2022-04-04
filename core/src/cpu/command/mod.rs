mod parameter;

use super::Context;
use parameter::*;
use std::fmt;

pub struct Command {
    opcode: u8,
    mnemonic: &'static str,
    cycles: u64,
    execute: Box<dyn Fn(&mut dyn Context)>,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("U8Destination")
            .field("name", &self.opcode)
            .field("mnemonic", &self.mnemonic)
            .field("cycles", &self.cycles)
            .finish()
    }
}

impl Command {
    pub fn execute(&self, context: &mut dyn Context) {
        (self.execute)(context)
    }

    pub fn next(context: &mut dyn Context) -> Self {
        let opcode = context.pop_from_pc();
        let command =
            |mnemonic: &'static str, cycles: u64, execute: Box<dyn Fn(&mut dyn Context)>| Self {
                opcode,
                mnemonic,
                cycles,
                execute,
            };
        let ld_n = |destination: U8Destination| {
            command(
                "LD",
                8,
                Box::new(move |context| {
                    let value = context.pop_from_pc();
                    destination.write(context, value);
                }),
            )
        };
        match opcode {
            // 8-Bit Loads
            0x06 => ld_n(u8_destination::B),
            0x0E => ld_n(u8_destination::C),
            0x16 => ld_n(u8_destination::D),
            // Miscellaneous
            0x00 => command("NOP", 4, Box::new(|_| {})),
            // Jumps
            0xC3 => command(
                "JP",
                12,
                Box::new(|context| {
                    context.registers_mut().pc = context.pop16_from_pc();
                }),
            ),
            // Not Implemented
            _ => panic!("This opcode is not implemented!: {:02X}", opcode),
        }
    }
}
