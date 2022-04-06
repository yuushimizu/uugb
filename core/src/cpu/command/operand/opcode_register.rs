use super::{indirection, Indirection, Operand, Read, ReadWrite, Register, Write, Writer};
use crate::cpu::Context;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperandType {
    Register,
    Indirection,
}

#[derive(Debug, Clone, Copy)]
pub enum OpcodeRegister {
    Register(Register<u8>),
    Indirection(Indirection),
}

impl OpcodeRegister {
    pub fn operand_type(&self) -> OperandType {
        match self {
            Self::Register(_) => OperandType::Register,
            Self::Indirection(_) => OperandType::Indirection,
        }
    }
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
    fn read(&self, context: &mut dyn Context) -> u8 {
        match self {
            Self::Register(register) => register.read(context),
            Self::Indirection(indirection) => indirection.read(context),
        }
    }
}

impl Write<u8> for OpcodeRegister {
    fn writer(&self, context: &mut dyn Context) -> Writer<u8> {
        match self {
            Self::Register(register) => register.writer(context),
            Self::Indirection(indirection) => indirection.writer(context),
        }
    }
}

impl ReadWrite<u8> for OpcodeRegister {
    fn read_write(&self, context: &mut dyn Context) -> (u8, Writer<u8>) {
        match self {
            Self::Register(register) => register.read_write(context),
            Self::Indirection(indirection) => indirection.read_write(context),
        }
    }
}
