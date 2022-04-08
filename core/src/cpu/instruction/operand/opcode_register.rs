use super::{indirection, Indirection, Operand, Read, ReadWrite, Register, Write, Writer};
use crate::cpu::CpuContext;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum OpcodeRegister {
    Register(Register<u8>),
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

impl fmt::Display for OpcodeRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Register(register) => register.fmt(f),
            Self::Indirection(indirection) => indirection.fmt(f),
        }
    }
}

impl Operand for OpcodeRegister {}

impl Read<u8> for OpcodeRegister {
    fn read(&self, context: &mut dyn CpuContext) -> u8 {
        match self {
            Self::Register(register) => register.read(context),
            Self::Indirection(indirection) => indirection.read(context),
        }
    }
}

impl Write<u8> for OpcodeRegister {
    fn prepare(&self, context: &mut dyn CpuContext) -> Writer<u8> {
        match self {
            Self::Register(register) => register.prepare(context),
            Self::Indirection(indirection) => indirection.prepare(context),
        }
    }
}

impl ReadWrite<u8> for OpcodeRegister {
    fn prepare_and_read(&self, context: &mut dyn CpuContext) -> (u8, Writer<u8>) {
        match self {
            Self::Register(register) => register.prepare_and_read(context),
            Self::Indirection(indirection) => indirection.prepare_and_read(context),
        }
    }
}
