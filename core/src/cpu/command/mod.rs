mod operand;
mod operator;

use super::Context;
use operand::ReadWrite;
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

enum RegisterOperandType {
    Register,
    Indirection,
}

struct RegisterOperand {
    operand: ReadWrite<u8>,
    operand_type: RegisterOperandType,
}

impl RegisterOperand {
    fn from_opcode(opcode: u8) -> RegisterOperand {
        use operand::register::*;
        use RegisterOperandType::*;
        let bits = opcode & 0b111;
        match bits {
            0b110 => RegisterOperand {
                operand: operand::indirection::HL,
                operand_type: Indirection,
            },
            _ => RegisterOperand {
                operand: match bits {
                    0b111 => A,
                    0b000 => B,
                    0b001 => C,
                    0b010 => D,
                    0b011 => E,
                    0b100 => H,
                    0b101 => L,
                    _ => unreachable!(),
                },
                operand_type: Register,
            },
        }
    }

    fn cycles(&self) -> u64 {
        use RegisterOperandType::*;
        match self.operand_type {
            Register => 0,
            Indirection => 4,
        }
    }
}

impl Command {
    pub fn execute(&self, context: &mut dyn Context) {
        self.operator.execute(context)
    }

    fn next_cb(context: &mut dyn Context, opcode: u8) -> Self {
        use operator::*;
        let sub_opcode = context.fetch_pc();
        let register_operand = RegisterOperand::from_opcode(sub_opcode);
        let operand = register_operand.operand;
        let bit_operand = sub_opcode >> 3 & 0b111;
        Self {
            opcode,
            sub_opcode: Some(sub_opcode),
            operator: match sub_opcode {
                // Miscellaneous
                0x30..=0x37 => swap(operand),
                // Rotates & Shifts
                0x00..=0x07 => rlc(operand),
                0x10..=0x17 => rl(operand),
                0x08..=0x0F => rrc(operand),
                0x18..=0x1F => rr(operand),
                0x20..=0x27 => sla(operand),
                0x28..=0x2F => sra(operand),
                0x38..=0x3F => srl(operand),
                0x40..=0x7F => bit(bit_operand, operand.as_read()),
                0xC0..=0xFF => set(bit_operand, operand.as_read_write()),
                0x80..=0xBF => res(bit_operand, operand.as_read_write()),
            },
            cycles: register_operand.cycles(),
        }
    }

    pub fn next(context: &mut dyn Context) -> Self {
        use operand::register::*;
        use operand::*;
        use operator::*;
        let opcode = context.fetch_pc();
        let register_operand = RegisterOperand::from_opcode(opcode);
        let (operator, cycles) = match opcode {
            // Miscellaneous (HALT)
            0x76 => (halt(), 4),
            // 8-Bit Loads
            0x06 => (ld(B, LITERAL), 8),
            0x0E => (ld(C, LITERAL), 8),
            0x16 => (ld(D, LITERAL), 8),
            0x1E => (ld(E, LITERAL), 8),
            0x26 => (ld(H, LITERAL), 8),
            0x2E => (ld(L, LITERAL), 8),
            0x40..=0x7F => {
                let lhs = RegisterOperand::from_opcode(opcode >> 3);
                (
                    ld(lhs.operand.as_write(), register_operand.operand.as_read()),
                    4 + lhs.cycles() + register_operand.cycles(),
                )
            }
            0x36 => (ld(indirection::HL, LITERAL), 12),
            0x0A => (ld(A, indirection::BC), 8),
            0x1A => (ld(A, indirection::DE), 8),
            0xFA => (ld(A, indirection::LITERAL), 16),
            0x3E => (ld(A, LITERAL), 8),
            0x02 => (ld(indirection::BC, A), 8),
            0x12 => (ld(indirection::DE, A), 8),
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
            0x80..=0x87 => (
                add(A, register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xC6 => (add(A, LITERAL), 8),
            0x88..=0x8F => (
                adc(A, register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xCE => (adc(A, LITERAL), 8),
            0x80..=0x97 => (
                sub(register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xD6 => (sub(LITERAL), 8),
            0x98..=0x9F => (
                sbc(A, register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xDE => (sbc(A, LITERAL), 8),
            0xA0..=0xA7 => (
                and(register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xE6 => (and(LITERAL), 8),
            0xB0..=0xB7 => (
                or(register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xF6 => (or(LITERAL), 8),
            0xA8..=0xAF => (
                xor(register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
            0xEE => (xor(LITERAL), 8),
            0xB8..=0xBF => (
                cp(register_operand.operand.as_read()),
                4 + register_operand.cycles(),
            ),
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
            0x10 => (stop(), 4),
            0xF3 => (di(), 4),
            0xFB => (ei(), 4),
            // Rotates & Shifts
            0x07 => (rlca(), 4),
            0x17 => (rla(), 4),
            0x0F => (rrca(), 4),
            0x1F => (rra(), 4),
            // Jumps
            0xC3 => (jp(LITERAL), 12),
            0xC2 => (jp_cc(jump::condition::NZ, LITERAL), 12),
            0xCA => (jp_cc(jump::condition::Z, LITERAL), 12),
            0xD2 => (jp_cc(jump::condition::NC, LITERAL), 12),
            0xDA => (jp_cc(jump::condition::C, LITERAL), 12),
            0xE9 => (jp(indirection::HL), 4),
            0x18 => (jr(LITERAL), 8),
            0x20 => (jr_cc(jump::condition::NZ, LITERAL), 8),
            0x28 => (jr_cc(jump::condition::Z, LITERAL), 8),
            0x30 => (jr_cc(jump::condition::NC, LITERAL), 8),
            0x38 => (jr_cc(jump::condition::C, LITERAL), 8),
            // Calls
            0xCD => (call(LITERAL), 12),
            0xC4 => (call_cc(jump::condition::NZ, LITERAL), 12),
            0xCC => (call_cc(jump::condition::Z, LITERAL), 12),
            0xD4 => (call_cc(jump::condition::NC, LITERAL), 12),
            0xDC => (call_cc(jump::condition::C, LITERAL), 12),
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
