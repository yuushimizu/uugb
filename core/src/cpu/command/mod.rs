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
            0x06 => ld(B, U8_LITERAL, 8),
            0x0E => ld(C, U8_LITERAL, 8),
            0x16 => ld(D, U8_LITERAL, 8),
            0x1E => ld(E, U8_LITERAL, 8),
            0x26 => ld(H, U8_LITERAL, 8),
            0x2E => ld(L, U8_LITERAL, 8),
            // LD r1, r2
            0x7F => ld(A, A, 4),
            0x78 => ld(A, B, 4),
            0x79 => ld(A, C, 4),
            0x7A => ld(A, D, 4),
            0x7B => ld(A, E, 4),
            0x7C => ld(A, H, 4),
            0x7D => ld(A, L, 4),
            0x7E => ld(A, indirection::HL, 8),
            0x40 => ld(B, B, 4),
            0x41 => ld(B, C, 4),
            0x42 => ld(B, D, 4),
            0x43 => ld(B, E, 4),
            0x44 => ld(B, H, 4),
            0x45 => ld(B, L, 4),
            0x46 => ld(B, indirection::HL, 8),
            0x48 => ld(C, B, 4),
            0x49 => ld(C, C, 4),
            0x4A => ld(C, D, 4),
            0x4B => ld(C, E, 4),
            0x4C => ld(C, H, 4),
            0x4D => ld(C, L, 4),
            0x4E => ld(C, indirection::HL, 8),
            0x50 => ld(D, B, 4),
            0x51 => ld(D, C, 4),
            0x52 => ld(D, D, 4),
            0x53 => ld(D, E, 4),
            0x54 => ld(D, H, 4),
            0x55 => ld(D, L, 4),
            0x56 => ld(D, indirection::HL, 8),
            0x58 => ld(E, B, 4),
            0x59 => ld(E, C, 4),
            0x5A => ld(E, D, 4),
            0x5B => ld(E, E, 4),
            0x5C => ld(E, H, 4),
            0x5D => ld(E, L, 4),
            0x5E => ld(E, indirection::HL, 8),
            0x60 => ld(H, B, 4),
            0x61 => ld(H, C, 4),
            0x62 => ld(H, D, 4),
            0x63 => ld(H, E, 4),
            0x64 => ld(H, H, 4),
            0x65 => ld(H, L, 4),
            0x66 => ld(H, indirection::HL, 8),
            0x68 => ld(L, B, 4),
            0x69 => ld(L, C, 4),
            0x6A => ld(L, D, 4),
            0x6B => ld(L, E, 4),
            0x6C => ld(L, H, 4),
            0x6D => ld(L, L, 4),
            0x6E => ld(L, indirection::HL, 8),
            0x70 => ld(indirection::HL, B, 8),
            0x71 => ld(indirection::HL, C, 8),
            0x72 => ld(indirection::HL, D, 8),
            0x73 => ld(indirection::HL, E, 8),
            0x74 => ld(indirection::HL, H, 8),
            0x75 => ld(indirection::HL, L, 8),
            0x36 => ld(indirection::HL, U8_LITERAL, 12),
            // LD A, n
            0x0A => ld(A, indirection::BC, 8),
            0x1A => ld(A, indirection::DE, 8),
            0xFA => ld(A, indirection::LITERAL, 16),
            0x3E => ld(A, U8_LITERAL, 8),
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
