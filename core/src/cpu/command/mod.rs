mod parameter;

use super::Context;
use parameter::*;
use std::fmt;

pub struct Command {
    opcode: u8,
    mnemonic: &'static str,
    execute: Box<dyn Fn(&mut dyn Context)>,
    cycles: u64,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("opcode", &self.opcode)
            .field("mnemonic", &self.mnemonic)
            .field("cycles", &self.cycles)
            .finish()
    }
}

fn ld_constructor<T: Copy>(
    opcode: u8,
) -> Box<dyn Fn(&'static dyn Destination<T>, &'static dyn Source<T>, u64) -> Command> {
    Box::new(move |destination, source, cycles| Command {
        opcode,
        mnemonic: "LD",
        execute: Box::new(|context| {
            let writer = destination.writer(context);
            let value = source.read(context);
            writer(context, value);
        }),
        cycles,
    })
}

fn push_constructor(opcode: u8) -> Box<dyn Fn(&'static dyn Source<u16>, u64) -> Command> {
    Box::new(move |source, cycles| Command {
        opcode,
        mnemonic: "PUSH",
        execute: Box::new(|context| {
            let value = source.read(context);
            let address = context.registers().sp;
            context.write16(address, value);
            context.registers_mut().sp = address.wrapping_sub(2);
        }),
        cycles,
    })
}

fn pop_constructor(opcode: u8) -> Box<dyn Fn(&'static dyn Destination<u16>, u64) -> Command> {
    Box::new(move |destination, cycles| Command {
        opcode,
        mnemonic: "POP",
        execute: Box::new(|context| {
            let writer = destination.writer(context);
            let address = context.registers().sp;
            let value = context.read16(address);
            writer(context, value);
            context.registers_mut().sp = address.wrapping_add(2);
        }),
        cycles,
    })
}

impl Command {
    pub fn execute(&self, context: &mut dyn Context) {
        (self.execute)(context)
    }

    pub fn next(context: &mut dyn Context) -> Self {
        use parameter::register::*;
        let opcode = context.pop_from_pc();
        let command = |mnemonic, execute: fn(&mut dyn Context), cycles| Self {
            opcode,
            mnemonic,
            execute: Box::new(execute),
            cycles,
        };
        let ld = ld_constructor::<u8>(opcode);
        let ld16 = ld_constructor::<u16>(opcode);
        let push = push_constructor(opcode);
        let pop = pop_constructor(opcode);
        match opcode {
            // 8-Bit Loads
            // LD r, n
            0x06 => ld(B, LITERAL, 8),
            0x0E => ld(C, LITERAL, 8),
            0x16 => ld(D, LITERAL, 8),
            0x1E => ld(E, LITERAL, 8),
            0x26 => ld(H, LITERAL, 8),
            0x2E => ld(L, LITERAL, 8),
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
            0x36 => ld(indirection::HL, LITERAL, 12),
            // LD A, n
            0x0A => ld(A, indirection::BC, 8),
            0x1A => ld(A, indirection::DE, 8),
            0xFA => ld(A, indirection::LITERAL, 16),
            0x3E => ld(A, LITERAL, 8),
            // LD n, A
            0x47 => ld(B, A, 4),
            0x4F => ld(C, A, 4),
            0x57 => ld(D, A, 4),
            0x5F => ld(E, A, 4),
            0x67 => ld(H, A, 4),
            0x6F => ld(L, A, 4),
            0x02 => ld(indirection::BC, A, 8),
            0x12 => ld(indirection::DE, A, 8),
            0x77 => ld(indirection::HL, A, 8),
            0xEA => ld(indirection::LITERAL, A, 16),
            // LD A, (C)
            0xF2 => ld(A, indirection::C, 8),
            // LD (C), A
            0xE2 => ld(indirection::C, A, 8),
            // LD A, (HLD)
            0x3A => ld(A, indirection::HLD, 8),
            // LD (HLD), A
            0x32 => ld(indirection::HLD, A, 8),
            // LD A, (HLI)
            0x2A => ld(A, indirection::HLI, 8),
            // LD (HLI), A
            0x22 => ld(indirection::HLI, A, 8),
            // LDH (n), A
            0xE0 => ld(indirection::LITERAL_8, A, 12),
            // LDH A, (n)
            0xF0 => ld(A, indirection::LITERAL_8, 12),
            // 16-Bit Loads
            // LD n, nn
            0x01 => ld16(BC, LITERAL, 12),
            0x11 => ld16(DE, LITERAL, 12),
            0x21 => ld16(HL, LITERAL, 12),
            0x31 => ld16(SP, LITERAL, 12),
            // LD SP, HL
            0xF9 => ld16(SP, HL, 8),
            // LD HL, SP+n
            0xF8 => ld16(HL, stack_pointer::ADD_LITERAL_8, 12),
            // LD (nn), SP
            0x08 => ld16(indirection::LITERAL, SP, 20),
            // PUSH nn
            0xF5 => push(AF, 16),
            0xC5 => push(BC, 16),
            0xD5 => push(DE, 16),
            0xE5 => push(HL, 16),
            // POP nn
            0xF1 => pop(AF, 12),
            0xC1 => pop(BC, 12),
            0xD1 => pop(DE, 12),
            0xE1 => pop(HL, 12),
            // Miscellaneous
            0x00 => command("NOP", |_| {}, 4),
            // Jumps
            0xC3 => command(
                "JP",
                |context| {
                    context.registers_mut().pc = context.pop16_from_pc();
                },
                12,
            ),
            // Not Implemented
            _ => panic!("This opcode is not implemented!: {:02X}", opcode),
        }
    }
}
