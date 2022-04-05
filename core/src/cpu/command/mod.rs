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
        let ld = |destination: U8Destination, source: U8Source, cycles: u64| {
            command(
                "LD",
                cycles,
                Box::new(move |context| {
                    let writer = destination.writer(context);
                    let value = source.read(context);
                    writer(context, value);
                }),
            )
        };
        match opcode {
            // 8-Bit Loads
            0x06 => ld(u8_destination::B, u8_source::LITERAL, 4),
            0x0E => ld(u8_destination::C, u8_source::LITERAL, 4),
            0x16 => ld(u8_destination::D, u8_source::LITERAL, 4),
            0x1E => ld(u8_destination::E, u8_source::LITERAL, 4),
            0x26 => ld(u8_destination::H, u8_source::LITERAL, 4),
            0x2E => ld(u8_destination::L, u8_source::LITERAL, 4),
            0x7F => ld(u8_destination::A, u8_source::A, 4),
            0x78 => ld(u8_destination::A, u8_source::B, 4),
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
