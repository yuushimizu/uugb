use super::{indirection, DebugOperand, Indirection, Operand, Read, Register, Write};
use crate::cpu::instruction::Context;

#[derive(Debug, Clone, Copy)]
pub enum OpcodeRegister {
    Register(Register),
    Indirection(Indirection),
}

impl From<u8> for OpcodeRegister {
    fn from(opcode: u8) -> Self {
        use super::register::*;
        let bits = opcode & 0b111;
        match bits {
            0b110 => Self::Indirection(indirection::HL),
            _ => Self::Register(match bits {
                0b111 => A,
                0b000 => B,
                0b001 => C,
                0b010 => D,
                0b011 => E,
                0b100 => H,
                0b101 => L,
                _ => unreachable!(),
            }),
        }
    }
}

impl Operand for OpcodeRegister {}

impl Read<u8> for OpcodeRegister {
    fn read(&self, context: &mut Context) -> u8 {
        match self {
            Self::Register(register) => register.read(context),
            Self::Indirection(indirection) => indirection.read(context),
        }
    }
}

impl Write<u8> for OpcodeRegister {
    fn write(&self, context: &mut Context, value: u8) {
        match self {
            Self::Register(register) => register.write(context, value),
            Self::Indirection(indirection) => indirection.write(context, value),
        }
    }
}

impl DebugOperand<u8> for OpcodeRegister {
    fn debug(&self, context: &Context) -> String {
        match self {
            Self::Register(register) => DebugOperand::<u8>::debug(register, context),
            Self::Indirection(indirection) => DebugOperand::<u8>::debug(indirection, context),
        }
    }
}
