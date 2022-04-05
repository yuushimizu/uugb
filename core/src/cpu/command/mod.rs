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
        use parameter::register::*;
        let opcode = context.pop_from_pc();
        let command =
            |mnemonic: &'static str, cycles: u64, execute: Box<dyn Fn(&mut dyn Context)>| Self {
                opcode,
                mnemonic,
                cycles,
                execute,
            };
        let ld = |destination: &'static dyn U8Destination,
                  source: &'static dyn U8Source,
                  cycles: u64| {
            command(
                "LD",
                cycles,
                Box::new(|context| {
                    let writer = destination.writer(context);
                    let value = source.read(context);
                    writer(context, value);
                }),
            )
        };
        match opcode {
            // 8-Bit Loads
            // LD r, n
            0x06 => ld(B, U8_LITERAL, 4),
            0x0E => ld(C, U8_LITERAL, 4),
            0x16 => ld(D, U8_LITERAL, 4),
            0x1E => ld(E, U8_LITERAL, 4),
            0x26 => ld(H, U8_LITERAL, 4),
            0x2E => ld(L, U8_LITERAL, 4),
            // LD r1, r2
            0x7F => ld(A, A, 4),
            0x78 => ld(A, B, 4),
            0x79 => ld(A, C, 4),
            0x7A => ld(A, D, 4),
            0x7B => ld(A, E, 4),
            0x7C => ld(A, H, 4),
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
