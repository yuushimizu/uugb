mod operand;
//mod operator;

use super::CpuContext;
//use operator::Operator;
use std::fmt;

/*
#[derive(Debug)]
pub struct Instruction {
    opcode: u8,
    sub_opcode: Option<u8>,
    operator: Operator,
    cycles: u64,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:02X}{}) {}",
            self.opcode,
            self.sub_opcode
                .map_or("".into(), |sub_opcode| format!(" {:02X}", sub_opcode)),
            self.operator
        )
    }
}

trait OpcodeRegisterCycles {
    fn cycles(&self) -> u64;
}

impl OpcodeRegisterCycles for operand::OpcodeRegister {
    fn cycles(&self) -> u64 {
        use operand::opcode_register::OperandType::*;
        match self.operand_type() {
            Register => 0,
            Indirection => 1,
        }
    }
}

impl Instruction {
    pub fn execute(&self, context: &mut dyn CpuContext) {
        self.operator.execute(context)
    }

    fn next_cb(context: &mut dyn CpuContext, opcode: u8) -> Self {
        use operand::*;
        use operator::*;
        let sub_opcode = context.fetch();
        let opcode_register = OpcodeRegister::from(sub_opcode);
        let bit_operand = sub_opcode >> 3 & 0b111;
        Self {
            opcode,
            sub_opcode: Some(sub_opcode),
            operator: match sub_opcode {
                // Miscellaneous
                0x30..=0x37 => swap(opcode_register),
                // Rotates & Shifts
                0x00..=0x07 => rlc(opcode_register),
                0x10..=0x17 => rl(opcode_register),
                0x08..=0x0F => rrc(opcode_register),
                0x18..=0x1F => rr(opcode_register),
                0x20..=0x27 => sla(opcode_register),
                0x28..=0x2F => sra(opcode_register),
                0x38..=0x3F => srl(opcode_register),
                0x40..=0x7F => bit(bit_operand, opcode_register),
                0xC0..=0xFF => set(bit_operand, opcode_register),
                0x80..=0xBF => res(bit_operand, opcode_register),
            },
            cycles: opcode_register.cycles(),
        }
    }

    pub fn next(context: &mut dyn CpuContext) -> Self {
        use operand::register::*;
        use operand::*;
        use operator::*;
        let opcode = context.fetch();
        let opcode_register = OpcodeRegister::from(opcode);
        let (operator, cycles) = match opcode {
            // Miscellaneous (HALT)
            0x76 => (halt(), 1),
            // 8-Bit Loads
            0x06 => (ld(B, LITERAL), 2),
            0x0E => (ld(C, LITERAL), 2),
            0x16 => (ld(D, LITERAL), 2),
            0x1E => (ld(E, LITERAL), 2),
            0x26 => (ld(H, LITERAL), 2),
            0x2E => (ld(L, LITERAL), 2),
            0x40..=0x7F => {
                let lhs = OpcodeRegister::from(opcode >> 3);
                (
                    ld(lhs, opcode_register),
                    1 + lhs.cycles() + opcode_register.cycles(),
                )
            }
            0x36 => (ld(indirection::HL, LITERAL), 3),
            0x0A => (ld(A, indirection::BC), 2),
            0x1A => (ld(A, indirection::DE), 2),
            0xFA => (ld(A, indirection::LITERAL), 4),
            0x3E => (ld(A, LITERAL), 2),
            0x02 => (ld(indirection::BC, A), 2),
            0x12 => (ld(indirection::DE, A), 2),
            0xEA => (ld(indirection::LITERAL, A), 4),
            0xF2 => (ld(A, indirection::C), 2),
            0xE2 => (ld(indirection::C, A), 2),
            0x3A => (ld(A, indirection::HLD), 2),
            0x32 => (ld(indirection::HLD, A), 2),
            0x2A => (ld(A, indirection::HLI), 2),
            0x22 => (ld(indirection::HLI, A), 2),
            0xE0 => (ld(indirection::LITERAL_8, A), 3),
            0xF0 => (ld(A, indirection::LITERAL_8), 3),
            // 16-Bit Loads
            0x01 => (ld16(BC, LITERAL), 3),
            0x11 => (ld16(DE, LITERAL), 3),
            0x21 => (ld16(HL, LITERAL), 3),
            0x31 => (ld16(SP, LITERAL), 3),
            0xF9 => (ld16(SP, HL), 2),
            0xF8 => (ld16(HL, stack_pointer::ADD_LITERAL_8), 3),
            0x08 => (ld16(indirection::LITERAL, SP), 5),
            0xF5 => (push(AF), 4),
            0xC5 => (push(BC), 4),
            0xD5 => (push(DE), 4),
            0xE5 => (push(HL), 4),
            0xF1 => (pop(AF), 3),
            0xC1 => (pop(BC), 3),
            0xD1 => (pop(DE), 3),
            0xE1 => (pop(HL), 3),
            // 8-Bit ALU
            0x80..=0x87 => (add(A, opcode_register), 1 + opcode_register.cycles()),
            0xC6 => (add(A, LITERAL), 2),
            0x88..=0x8F => (adc(A, opcode_register), 1 + opcode_register.cycles()),
            0xCE => (adc(A, LITERAL), 2),
            0x80..=0x97 => (sub(opcode_register), 1 + opcode_register.cycles()),
            0xD6 => (sub(LITERAL), 2),
            0x98..=0x9F => (sbc(A, opcode_register), 1 + opcode_register.cycles()),
            0xDE => (sbc(A, LITERAL), 2),
            0xA0..=0xA7 => (and(opcode_register), 1 + opcode_register.cycles()),
            0xE6 => (and(LITERAL), 2),
            0xB0..=0xB7 => (or(opcode_register), 1 + opcode_register.cycles()),
            0xF6 => (or(LITERAL), 2),
            0xA8..=0xAF => (xor(opcode_register), 1 + opcode_register.cycles()),
            0xEE => (xor(LITERAL), 2),
            0xB8..=0xBF => (cp(opcode_register), 1 + opcode_register.cycles()),
            0xFE => (cp(LITERAL), 2),
            0x3C => (inc(A), 1),
            0x04 => (inc(B), 1),
            0x0C => (inc(C), 1),
            0x14 => (inc(D), 1),
            0x1C => (inc(E), 1),
            0x24 => (inc(H), 1),
            0x2C => (inc(L), 1),
            0x34 => (inc(indirection::HL), 3),
            0x3D => (dec(A), 1),
            0x05 => (dec(B), 1),
            0x0D => (dec(C), 1),
            0x15 => (dec(D), 1),
            0x1D => (dec(E), 1),
            0x25 => (dec(H), 1),
            0x2D => (dec(L), 1),
            0x35 => (dec(indirection::HL), 3),
            // 16-Bit Arithmetic
            0x09 => (add16(HL, BC), 2),
            0x19 => (add16(HL, DE), 2),
            0x29 => (add16(HL, HL), 2),
            0x39 => (add16(HL, SP), 2),
            0xE8 => (add_sp(LITERAL), 4),
            0x03 => (inc16(BC), 2),
            0x13 => (inc16(DE), 2),
            0x23 => (inc16(HL), 2),
            0x33 => (inc16(SP), 2),
            0x0B => (dec16(BC), 2),
            0x1B => (dec16(DE), 2),
            0x2B => (dec16(HL), 2),
            0x3B => (dec16(SP), 2),
            // Miscellaneous
            0xCB => return Self::next_cb(context, opcode),
            0x27 => (daa(), 1),
            0x2F => (cpl(), 1),
            0x3F => (ccf(), 1),
            0x37 => (scf(), 1),
            0x00 => (nop(), 1),
            0x10 => (stop(), 1),
            0xF3 => (di(), 1),
            0xFB => (ei(), 1),
            // Rotates & Shifts
            0x07 => (rlca(), 1),
            0x17 => (rla(), 1),
            0x0F => (rrca(), 1),
            0x1F => (rra(), 1),
            // Jumps
            0xC3 => (jp(LITERAL), 3),
            0xC2 => (jp_cc(jump::condition::NZ, LITERAL), 3),
            0xCA => (jp_cc(jump::condition::Z, LITERAL), 3),
            0xD2 => (jp_cc(jump::condition::NC, LITERAL), 3),
            0xDA => (jp_cc(jump::condition::C, LITERAL), 3),
            0xE9 => (jp(HL), 1),
            0x18 => (jr(LITERAL), 2),
            0x20 => (jr_cc(jump::condition::NZ, LITERAL), 2),
            0x28 => (jr_cc(jump::condition::Z, LITERAL), 2),
            0x30 => (jr_cc(jump::condition::NC, LITERAL), 2),
            0x38 => (jr_cc(jump::condition::C, LITERAL), 2),
            // Calls
            0xCD => (call(LITERAL), 3),
            0xC4 => (call_cc(jump::condition::NZ, LITERAL), 3),
            0xCC => (call_cc(jump::condition::Z, LITERAL), 3),
            0xD4 => (call_cc(jump::condition::NC, LITERAL), 3),
            0xDC => (call_cc(jump::condition::C, LITERAL), 3),
            // Restarts
            0xC7 => (rst(0x00), 8),
            0xCF => (rst(0x08), 8),
            0xD7 => (rst(0x10), 8),
            0xDF => (rst(0x18), 8),
            0xE7 => (rst(0x20), 8),
            0xEF => (rst(0x28), 8),
            0xF7 => (rst(0x30), 8),
            0xFF => (rst(0x38), 8),
            // Returns
            0xC9 => (ret(), 2),
            0xC0 => (ret_cc(jump::condition::NZ), 2),
            0xC8 => (ret_cc(jump::condition::Z), 2),
            0xD0 => (ret_cc(jump::condition::NC), 2),
            0xD8 => (ret_cc(jump::condition::C), 2),
            0xD9 => (reti(), 2),
            // Not Implemented
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                (unused(opcode), 0)
            }
        };
        Self {
            opcode,
            sub_opcode: None,
            operator,
            cycles,
        }
    }
}
*/
