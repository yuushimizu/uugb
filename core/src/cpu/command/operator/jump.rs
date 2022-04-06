use super::Operator;
use crate::cpu::{command::operand::ReadRef, registers::Flags};

pub fn jp(operand: ReadRef<u16>) -> Operator {
    Operator::new("JP", |context| {
        context.registers_mut().pc = operand.read(context);
    })
}

fn conditional_jump(operand: ReadRef<u16>, condition: fn(Flags) -> bool) -> Operator {
    Operator::new("JP", move |context| {
        let address = operand.read(context);
        if condition(context.flags()) {
            context.registers_mut().pc = address;
        }
    })
}

pub fn jp_nz(operand: ReadRef<u16>) -> Operator {
    conditional_jump(operand, |flags| !flags.z)
}

pub fn jp_z(operand: ReadRef<u16>) -> Operator {
    conditional_jump(operand, |flags| flags.z)
}

pub fn jp_nc(operand: ReadRef<u16>) -> Operator {
    conditional_jump(operand, |flags| !flags.c)
}

pub fn jp_c(operand: ReadRef<u16>) -> Operator {
    conditional_jump(operand, |flags| flags.c)
}
