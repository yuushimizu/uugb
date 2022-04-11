use super::{DebugOperand, Operand, Read, Write};
use crate::cpu::{instruction::Context, Registers};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl Register {}

impl Register {
    fn value(&self, registers: &Registers) -> u8 {
        use Register::*;
        match self {
            A => registers.a,
            B => registers.b,
            C => registers.c,
            D => registers.d,
            E => registers.e,
            H => registers.h,
            L => registers.l,
        }
    }
}

impl Operand for Register {}

impl Read<u8> for Register {
    fn read(&self, context: &mut Context) -> u8 {
        self.value(context.registers())
    }
}

impl Write<u8> for Register {
    fn write(&self, context: &mut Context, value: u8) {
        use Register::*;
        let registers = context.registers_mut();
        match self {
            A => registers.a = value,
            B => registers.b = value,
            C => registers.c = value,
            D => registers.d = value,
            E => registers.e = value,
            H => registers.h = value,
            L => registers.l = value,
        }
    }
}

impl DebugOperand<u8> for Register {
    fn debug(&self, context: &Context) -> String {
        format!("{:?}={:02X}", self, self.value(context.registers()))
    }
}

pub use Register::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register16 {
    Af,
    Bc,
    De,
    Hl,
    Sp,
}

impl Register {}

impl Register16 {
    fn value(&self, registers: &Registers) -> u16 {
        use Register16::*;
        match self {
            Af => registers.af(),
            Bc => registers.bc(),
            De => registers.de(),
            Hl => registers.hl(),
            Sp => registers.sp,
        }
    }
}

impl Operand for Register16 {}

impl Read<u16> for Register16 {
    fn read(&self, context: &mut Context) -> u16 {
        self.value(context.registers())
    }
}

impl Write<u16> for Register16 {
    fn write(&self, context: &mut Context, value: u16) {
        use Register16::*;
        let registers = context.registers_mut();
        match self {
            Af => registers.set_af(value),
            Bc => registers.set_bc(value),
            De => registers.set_de(value),
            Hl => registers.set_hl(value),
            Sp => registers.sp = value,
        }
    }
}

impl DebugOperand<u16> for Register16 {
    fn debug(&self, context: &Context) -> String {
        format!(
            "{}={:04X}",
            format!("{:?}", self).to_uppercase(),
            self.value(context.registers())
        )
    }
}

pub use Register16::*;
