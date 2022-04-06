mod operand;
mod operator;

use super::Context;
use operand::{IndirectionRef, RegisterRef};
use operator::Operator;
use std::fmt;

pub struct Command {
    opcode: u8,
    sub_opcode: Option<u8>,
    operator: Operator,
    cycles: u64,
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Command")
            .field("opcode", &self.opcode)
            .field("sub_opcode", &self.sub_opcode)
            .field("operator", &self.operator)
            .field("cycles", &self.cycles)
            .finish()
    }
}

enum RegisterOperand {
    Register(RegisterRef<u8>),
    Indirection(IndirectionRef),
}

fn register_operand(opcode: u8) -> RegisterOperand {
    use operand::register::*;
    use RegisterOperand::*;
    match opcode & 0b111 {
        0b111 => Register(A),
        0b000 => Register(B),
        0b001 => Register(C),
        0b010 => Register(D),
        0b011 => Register(E),
        0b100 => Register(H),
        0b101 => Register(L),
        0b111 => Indirection(operand::indirection::HL),
        _ => unreachable!(),
    }
}

impl Command {
    pub fn execute(&self, context: &mut dyn Context) {
        self.operator.execute(context)
    }

    fn next_cb(context: &mut dyn Context, opcode: u8) -> Self {
        use operand::register::*;
        use operand::*;
        use operator::*;
        let sub_opcode = context.fetch_pc();
        let (operator, cycles) = match sub_opcode {
            // Miscellaneous
            0x37 => (swap(A), 8),
            0x30 => (swap(B), 8),
            0x31 => (swap(C), 8),
            0x32 => (swap(D), 8),
            0x33 => (swap(E), 8),
            0x34 => (swap(H), 8),
            0x35 => (swap(L), 8),
            0x36 => (swap(indirection::HL), 16),
            // Rotates & Shifts
            0x07 => (rlc(A), 8),
            0x00 => (rlc(B), 8),
            0x01 => (rlc(C), 8),
            0x02 => (rlc(D), 8),
            0x03 => (rlc(E), 8),
            0x04 => (rlc(H), 8),
            0x05 => (rlc(L), 8),
            0x06 => (rlc(indirection::HL), 16),
            0x17 => (rl(A), 8),
            0x10 => (rl(B), 8),
            0x11 => (rl(C), 8),
            0x12 => (rl(D), 8),
            0x13 => (rl(E), 8),
            0x14 => (rl(H), 8),
            0x15 => (rl(L), 8),
            0x16 => (rl(indirection::HL), 16),
            0x0F => (rrc(A), 8),
            0x08 => (rrc(B), 8),
            0x09 => (rrc(C), 8),
            0x0A => (rrc(D), 8),
            0x0B => (rrc(E), 8),
            0x0C => (rrc(H), 8),
            0x0D => (rrc(L), 8),
            0x0E => (rrc(indirection::HL), 16),
            0x1F => (rr(A), 8),
            0x18 => (rr(B), 8),
            0x19 => (rr(C), 8),
            0x1A => (rr(D), 8),
            0x1B => (rr(E), 8),
            0x1C => (rr(H), 8),
            0x1D => (rr(L), 8),
            0x1E => (rr(indirection::HL), 16),
            0x27 => (sla(A), 8),
            0x20 => (sla(B), 8),
            0x21 => (sla(C), 8),
            0x22 => (sla(D), 8),
            0x23 => (sla(E), 8),
            0x24 => (sla(H), 8),
            0x25 => (sla(L), 8),
            0x26 => (sla(indirection::HL), 16),
            0x2F => (sra(A), 8),
            0x28 => (sra(B), 8),
            0x29 => (sra(C), 8),
            0x2A => (sra(D), 8),
            0x2B => (sra(E), 8),
            0x2C => (sra(H), 8),
            0x2D => (sra(L), 8),
            0x2E => (sra(indirection::HL), 16),
            0x3F => (srl(A), 8),
            0x38 => (srl(B), 8),
            0x39 => (srl(C), 8),
            0x3A => (srl(D), 8),
            0x3B => (srl(E), 8),
            0x3C => (srl(H), 8),
            0x3D => (srl(L), 8),
            0x3E => (srl(indirection::HL), 16),
            0x40..=0x7F => {
                use RegisterOperand::*;
                let lhs = sub_opcode >> 3 & 0b111;
                match register_operand(sub_opcode) {
                    Register(register) => (bit(lhs, register.as_read()), 8),
                    Indirection(operand) => (bit(lhs, operand), 16),
                }
            }
            // Not Implemented
            _ => panic!(
                "This opcode is not implemented!: {:02X} {:02X}",
                opcode, sub_opcode
            ),
        };
        Self {
            opcode,
            sub_opcode: Some(sub_opcode),
            operator,
            cycles,
        }
    }

    pub fn next(context: &mut dyn Context) -> Self {
        use operand::register::*;
        use operand::*;
        use operator::*;
        let opcode = context.fetch_pc();
        let (operator, cycles) = match opcode {
            // 8-Bit Loads
            0x06 => (ld(B, LITERAL), 8),
            0x0E => (ld(C, LITERAL), 8),
            0x16 => (ld(D, LITERAL), 8),
            0x1E => (ld(E, LITERAL), 8),
            0x26 => (ld(H, LITERAL), 8),
            0x2E => (ld(L, LITERAL), 8),
            0x7F => (ld(A, A), 4),
            0x78 => (ld(A, B), 4),
            0x79 => (ld(A, C), 4),
            0x7A => (ld(A, D), 4),
            0x7B => (ld(A, E), 4),
            0x7C => (ld(A, H), 4),
            0x7D => (ld(A, L), 4),
            0x7E => (ld(A, indirection::HL), 8),
            0x40 => (ld(B, B), 4),
            0x41 => (ld(B, C), 4),
            0x42 => (ld(B, D), 4),
            0x43 => (ld(B, E), 4),
            0x44 => (ld(B, H), 4),
            0x45 => (ld(B, L), 4),
            0x46 => (ld(B, indirection::HL), 8),
            0x48 => (ld(C, B), 4),
            0x49 => (ld(C, C), 4),
            0x4A => (ld(C, D), 4),
            0x4B => (ld(C, E), 4),
            0x4C => (ld(C, H), 4),
            0x4D => (ld(C, L), 4),
            0x4E => (ld(C, indirection::HL), 8),
            0x50 => (ld(D, B), 4),
            0x51 => (ld(D, C), 4),
            0x52 => (ld(D, D), 4),
            0x53 => (ld(D, E), 4),
            0x54 => (ld(D, H), 4),
            0x55 => (ld(D, L), 4),
            0x56 => (ld(D, indirection::HL), 8),
            0x58 => (ld(E, B), 4),
            0x59 => (ld(E, C), 4),
            0x5A => (ld(E, D), 4),
            0x5B => (ld(E, E), 4),
            0x5C => (ld(E, H), 4),
            0x5D => (ld(E, L), 4),
            0x5E => (ld(E, indirection::HL), 8),
            0x60 => (ld(H, B), 4),
            0x61 => (ld(H, C), 4),
            0x62 => (ld(H, D), 4),
            0x63 => (ld(H, E), 4),
            0x64 => (ld(H, H), 4),
            0x65 => (ld(H, L), 4),
            0x66 => (ld(H, indirection::HL), 8),
            0x68 => (ld(L, B), 4),
            0x69 => (ld(L, C), 4),
            0x6A => (ld(L, D), 4),
            0x6B => (ld(L, E), 4),
            0x6C => (ld(L, H), 4),
            0x6D => (ld(L, L), 4),
            0x6E => (ld(L, indirection::HL), 8),
            0x70 => (ld(indirection::HL, B), 8),
            0x71 => (ld(indirection::HL, C), 8),
            0x72 => (ld(indirection::HL, D), 8),
            0x73 => (ld(indirection::HL, E), 8),
            0x74 => (ld(indirection::HL, H), 8),
            0x75 => (ld(indirection::HL, L), 8),
            0x36 => (ld(indirection::HL, LITERAL), 12),
            0x0A => (ld(A, indirection::BC), 8),
            0x1A => (ld(A, indirection::DE), 8),
            0xFA => (ld(A, indirection::LITERAL), 16),
            0x3E => (ld(A, LITERAL), 8),
            0x47 => (ld(B, A), 4),
            0x4F => (ld(C, A), 4),
            0x57 => (ld(D, A), 4),
            0x5F => (ld(E, A), 4),
            0x67 => (ld(H, A), 4),
            0x6F => (ld(L, A), 4),
            0x02 => (ld(indirection::BC, A), 8),
            0x12 => (ld(indirection::DE, A), 8),
            0x77 => (ld(indirection::HL, A), 8),
            0xEA => (ld(indirection::LITERAL, A), 16),
            0xF2 => (ld(A, indirection::C), 8),
            0xE2 => (ld(indirection::C, A), 8),
            0x3A => (ld(A, indirection::HLD), 8),
            0x32 => (ld(indirection::HLD, A), 8),
            0x2A => (ld(A, indirection::HLI), 8),
            0x22 => (ld(indirection::HLI, A), 8),
            0xE0 => (ld(indirection::LITERAL_8, A), 12),
            0xF0 => (ld(A, indirection::LITERAL_8), 12),
            // 16-Bit Loads
            0x01 => (ld16(BC, LITERAL), 12),
            0x11 => (ld16(DE, LITERAL), 12),
            0x21 => (ld16(HL, LITERAL), 12),
            0x31 => (ld16(SP, LITERAL), 12),
            0xF9 => (ld16(SP, HL), 8),
            0xF8 => (ld16(HL, stack_pointer::ADD_LITERAL_8), 12),
            0x08 => (ld16(indirection::LITERAL, SP), 20),
            0xF5 => (push(AF), 16),
            0xC5 => (push(BC), 16),
            0xD5 => (push(DE), 16),
            0xE5 => (push(HL), 16),
            0xF1 => (pop(AF), 12),
            0xC1 => (pop(BC), 12),
            0xD1 => (pop(DE), 12),
            0xE1 => (pop(HL), 12),
            // 8-Bit ALU
            0x87 => (add(A, A), 4),
            0x80 => (add(A, B), 4),
            0x81 => (add(A, C), 4),
            0x82 => (add(A, D), 4),
            0x83 => (add(A, E), 4),
            0x84 => (add(A, H), 4),
            0x85 => (add(A, L), 4),
            0x86 => (add(A, indirection::HL), 8),
            0xC6 => (add(A, LITERAL), 8),
            0x8F => (adc(A, A), 4),
            0x88 => (adc(A, B), 4),
            0x89 => (adc(A, C), 4),
            0x8A => (adc(A, D), 4),
            0x8B => (adc(A, E), 4),
            0x8C => (adc(A, H), 4),
            0x8D => (adc(A, L), 4),
            0x8E => (adc(A, indirection::HL), 8),
            0xCE => (adc(A, LITERAL), 8),
            0x97 => (sub(A), 4),
            0x90 => (sub(B), 4),
            0x91 => (sub(C), 4),
            0x92 => (sub(D), 4),
            0x93 => (sub(E), 4),
            0x94 => (sub(H), 4),
            0x95 => (sub(L), 4),
            0x96 => (sub(indirection::HL), 8),
            0xD6 => (sub(LITERAL), 8),
            0x9F => (sbc(A, A), 4),
            0x98 => (sbc(A, B), 4),
            0x99 => (sbc(A, C), 4),
            0x9A => (sbc(A, D), 4),
            0x9B => (sbc(A, E), 4),
            0x9C => (sbc(A, H), 4),
            0x9D => (sbc(A, L), 4),
            0x9E => (sbc(A, indirection::HL), 8),
            0xDE => (sbc(A, LITERAL), 8),
            0xA7 => (and(A), 4),
            0xA0 => (and(B), 4),
            0xA1 => (and(C), 4),
            0xA2 => (and(D), 4),
            0xA3 => (and(E), 4),
            0xA4 => (and(H), 4),
            0xA5 => (and(L), 4),
            0xA6 => (and(indirection::HL), 8),
            0xE6 => (and(LITERAL), 8),
            0xB7 => (or(A), 4),
            0xB0 => (or(B), 4),
            0xB1 => (or(C), 4),
            0xB2 => (or(D), 4),
            0xB3 => (or(E), 4),
            0xB4 => (or(H), 4),
            0xB5 => (or(L), 4),
            0xB6 => (or(indirection::HL), 8),
            0xF6 => (or(LITERAL), 8),
            0xAF => (xor(A), 4),
            0xA8 => (xor(B), 4),
            0xA9 => (xor(C), 4),
            0xAA => (xor(D), 4),
            0xAB => (xor(E), 4),
            0xAC => (xor(H), 4),
            0xAD => (xor(L), 4),
            0xAE => (xor(indirection::HL), 8),
            0xEE => (xor(LITERAL), 8),
            0xBF => (cp(A), 4),
            0xB8 => (cp(B), 4),
            0xB9 => (cp(C), 4),
            0xBA => (cp(D), 4),
            0xBB => (cp(E), 4),
            0xBC => (cp(H), 4),
            0xBD => (cp(L), 4),
            0xBE => (cp(indirection::HL), 8),
            0xFE => (cp(LITERAL), 8),
            0x3C => (inc(A), 4),
            0x04 => (inc(B), 4),
            0x0C => (inc(C), 4),
            0x14 => (inc(D), 4),
            0x1C => (inc(E), 4),
            0x24 => (inc(H), 4),
            0x2C => (inc(L), 4),
            0x34 => (inc(indirection::HL), 12),
            0x3D => (dec(A), 4),
            0x05 => (dec(B), 4),
            0x0D => (dec(C), 4),
            0x15 => (dec(D), 4),
            0x1D => (dec(E), 4),
            0x25 => (dec(H), 4),
            0x2D => (dec(L), 4),
            0x35 => (dec(indirection::HL), 12),
            // 16-Bit Arithmetic
            0x09 => (add16(HL, BC), 8),
            0x19 => (add16(HL, DE), 8),
            0x29 => (add16(HL, HL), 8),
            0x39 => (add16(HL, SP), 8),
            0xE8 => (add_sp(LITERAL), 16),
            0x03 => (inc16(BC), 8),
            0x13 => (inc16(DE), 8),
            0x23 => (inc16(HL), 8),
            0x33 => (inc16(SP), 8),
            0x0B => (dec16(BC), 8),
            0x1B => (dec16(DE), 8),
            0x2B => (dec16(HL), 8),
            0x3B => (dec16(SP), 8),
            // Miscellaneous
            0xCB => return Self::next_cb(context, opcode),
            0x27 => (daa(), 4),
            0x2F => (cpl(), 4),
            0x3F => (ccf(), 4),
            0x37 => (scf(), 4),
            0x00 => (nop(), 4),
            0x76 => (halt(), 4),
            0x10 => (stop(), 4),
            0xF3 => (di(), 4),
            0xFB => (ei(), 4),
            // Rotates & Shifts
            0x07 => (rlca(), 4),
            0x17 => (rla(), 4),
            0x0F => (rrca(), 4),
            0x1F => (rra(), 4),
            // Jumps
            0xC3 => (
                Operator::new("JP", |context| {
                    context.registers_mut().pc = context.fetch16_pc();
                }),
                12,
            ),
            // Not Implemented
            _ => panic!("This opcode is not implemented!: {:02X}", opcode),
        };
        Self {
            opcode,
            sub_opcode: None,
            operator,
            cycles,
        }
    }
}
