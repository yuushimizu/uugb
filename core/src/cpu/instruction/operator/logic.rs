use super::Operator;
use crate::cpu::{instruction::operand::Read, registers::Flags};

pub fn and(operand: impl Read<u8>) -> Operator {
    Operator::new(format!("AND {}", operand), move |context| {
        let n = operand.read(context);
        let result = context.registers().a & n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: true,
            c: false,
        });
        context.registers_mut().a = result;
    })
}

pub fn or(operand: impl Read<u8>) -> Operator {
    Operator::new(format!("OR {}", operand), move |context| {
        let n = operand.read(context);
        let result = context.registers().a | n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
        context.registers_mut().a = result;
    })
}

pub fn xor(operand: impl Read<u8>) -> Operator {
    Operator::new(format!("XOR {}", operand), move |context| {
        let n = operand.read(context);
        let result = context.registers().a ^ n;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
        context.registers_mut().a = result;
    })
}
