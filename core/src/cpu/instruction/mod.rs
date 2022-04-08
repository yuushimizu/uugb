mod operand;
mod operator;

use super::CpuContext;
use once_cell::sync::Lazy;
use operator::Operator;
use std::fmt;

#[derive(Debug)]
pub struct Instruction {
    opcode: u8,
    sub_opcode: Option<u8>,
    operator: Operator,
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

const NESTED_OPCODE: u8 = 0xCB;

static NESTED_INSTRUCTION: Lazy<Vec<Instruction>> = Lazy::new(|| {
    (0x00..=0xFF)
        .map(|opcode| {
            use operand::*;
            use operator::*;
            let opcode_register = OpcodeRegister::from(opcode);
            let bit_operand = opcode >> 3 & 0b111;
            Instruction {
                opcode: NESTED_OPCODE,
                sub_opcode: Some(opcode),
                operator: match opcode {
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
            }
        })
        .collect()
});

static INSTRUCTIONS: Lazy<Vec<Instruction>> = Lazy::new(|| {
    (0x00..=0xFF)
        .map(|opcode| {
            use operand::register::*;
            use operand::*;
            use operator::*;
            let opcode_register = OpcodeRegister::from(opcode);
            Instruction {
                opcode,
                sub_opcode: None,
                operator: match opcode {
                    NESTED_OPCODE => unused(opcode),
                    // Miscellaneous (HALT)
                    0x76 => halt(),
                    // 8-Bit Loads
                    0x06 => ld(B, LITERAL),
                    0x0E => ld(C, LITERAL),
                    0x16 => ld(D, LITERAL),
                    0x1E => ld(E, LITERAL),
                    0x26 => ld(H, LITERAL),
                    0x2E => ld(L, LITERAL),
                    0x40..=0x7F => ld(OpcodeRegister::from(opcode >> 3), opcode_register),
                    0x36 => ld(indirection::HL, LITERAL),
                    0x0A => ld(A, indirection::BC),
                    0x1A => ld(A, indirection::DE),
                    0xFA => ld(A, indirection::LITERAL),
                    0x3E => ld(A, LITERAL),
                    0x02 => ld(indirection::BC, A),
                    0x12 => ld(indirection::DE, A),
                    0xEA => ld(indirection::LITERAL, A),
                    0xF2 => ld(A, indirection::C),
                    0xE2 => ld(indirection::C, A),
                    0x3A => ld(A, indirection::HLD),
                    0x32 => ld(indirection::HLD, A),
                    0x2A => ld(A, indirection::HLI),
                    0x22 => ld(indirection::HLI, A),
                    0xE0 => ld(indirection::LITERAL_8, A),
                    0xF0 => ld(A, indirection::LITERAL_8),
                    // 16-Bit Loads
                    0x01 => ld16(BC, LITERAL),
                    0x11 => ld16(DE, LITERAL),
                    0x21 => ld16(HL, LITERAL),
                    0x31 => ld16(SP, LITERAL),
                    0xF9 => ld16_sp_hl(),
                    0xF8 => ld16(HL, stack_pointer::ADD_LITERAL_8),
                    0x08 => ld16(indirection::LITERAL, SP),
                    0xF5 => push(AF),
                    0xC5 => push(BC),
                    0xD5 => push(DE),
                    0xE5 => push(HL),
                    0xF1 => pop(AF),
                    0xC1 => pop(BC),
                    0xD1 => pop(DE),
                    0xE1 => pop(HL),
                    // 8-Bit ALU
                    0x80..=0x87 => add(A, opcode_register),
                    0xC6 => add(A, LITERAL),
                    0x88..=0x8F => adc(A, opcode_register),
                    0xCE => adc(A, LITERAL),
                    0x80..=0x97 => sub(opcode_register),
                    0xD6 => sub(LITERAL),
                    0x98..=0x9F => sbc(A, opcode_register),
                    0xDE => sbc(A, LITERAL),
                    0xA0..=0xA7 => and(opcode_register),
                    0xE6 => and(LITERAL),
                    0xB0..=0xB7 => or(opcode_register),
                    0xF6 => or(LITERAL),
                    0xA8..=0xAF => xor(opcode_register),
                    0xEE => xor(LITERAL),
                    0xB8..=0xBF => cp(opcode_register),
                    0xFE => cp(LITERAL),
                    0x3C => inc(A),
                    0x04 => inc(B),
                    0x0C => inc(C),
                    0x14 => inc(D),
                    0x1C => inc(E),
                    0x24 => inc(H),
                    0x2C => inc(L),
                    0x34 => inc(indirection::HL),
                    0x3D => dec(A),
                    0x05 => dec(B),
                    0x0D => dec(C),
                    0x15 => dec(D),
                    0x1D => dec(E),
                    0x25 => dec(H),
                    0x2D => dec(L),
                    0x35 => dec(indirection::HL),
                    // 16-Bit Arithmetic
                    0x09 => add16(HL, BC),
                    0x19 => add16(HL, DE),
                    0x29 => add16(HL, HL),
                    0x39 => add16(HL, SP),
                    0xE8 => add_sp(LITERAL),
                    0x03 => inc16(BC),
                    0x13 => inc16(DE),
                    0x23 => inc16(HL),
                    0x33 => inc16(SP),
                    0x0B => dec16(BC),
                    0x1B => dec16(DE),
                    0x2B => dec16(HL),
                    0x3B => dec16(SP),
                    // Miscellaneous
                    0x27 => daa(),
                    0x2F => cpl(),
                    0x3F => ccf(),
                    0x37 => scf(),
                    0x00 => nop(),
                    0x10 => stop(),
                    0xF3 => di(),
                    0xFB => ei(),
                    // Rotates & Shifts
                    0x07 => rlca(),
                    0x17 => rla(),
                    0x0F => rrca(),
                    0x1F => rra(),
                    // Jumps
                    0xC3 => jp_nn(),
                    0xC2 => jp_cc(jump::condition::NZ, LITERAL),
                    0xCA => jp_cc(jump::condition::Z, LITERAL),
                    0xD2 => jp_cc(jump::condition::NC, LITERAL),
                    0xDA => jp_cc(jump::condition::C, LITERAL),
                    0xE9 => jp_hl(),
                    0x18 => jr(LITERAL),
                    0x20 => jr_cc(jump::condition::NZ, LITERAL),
                    0x28 => jr_cc(jump::condition::Z, LITERAL),
                    0x30 => jr_cc(jump::condition::NC, LITERAL),
                    0x38 => jr_cc(jump::condition::C, LITERAL),
                    // Calls
                    0xCD => call(LITERAL),
                    0xC4 => call_cc(jump::condition::NZ, LITERAL),
                    0xCC => call_cc(jump::condition::Z, LITERAL),
                    0xD4 => call_cc(jump::condition::NC, LITERAL),
                    0xDC => call_cc(jump::condition::C, LITERAL),
                    // Restarts
                    0xC7 => rst(0x00),
                    0xCF => rst(0x08),
                    0xD7 => rst(0x10),
                    0xDF => rst(0x18),
                    0xE7 => rst(0x20),
                    0xEF => rst(0x28),
                    0xF7 => rst(0x30),
                    0xFF => rst(0x38),
                    // Returns
                    0xC9 => ret(),
                    0xC0 => ret_cc(jump::condition::NZ),
                    0xC8 => ret_cc(jump::condition::Z),
                    0xD0 => ret_cc(jump::condition::NC),
                    0xD8 => ret_cc(jump::condition::C),
                    0xD9 => reti(),
                    // Not Implemented
                    0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                        unused(opcode)
                    }
                },
            }
        })
        .collect()
});

impl Instruction {
    pub fn execute(&self, context: &mut CpuContext) {
        self.operator.execute(context);
    }

    fn fetch_nested(context: &mut CpuContext) -> &'static Self {
        &NESTED_INSTRUCTION[context.fetch() as usize]
    }

    pub fn fetch(context: &mut CpuContext) -> &'static Self {
        match context.fetch() {
            NESTED_OPCODE => Self::fetch_nested(context),
            opcode => &INSTRUCTIONS[opcode as usize],
        }
    }
}
