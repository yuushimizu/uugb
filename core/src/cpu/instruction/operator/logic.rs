use super::Operator;
use crate::cpu::{instruction::operand::Read, registers::Flags};

pub fn and(operand: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let n = operand.read(context);
            let result = context.registers().a & n;
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: true,
                c: false,
            });
            context.registers_mut().a = result;
        },
        move |context| format!("AND {}", operand.debug(context)),
    )
}

pub fn or(operand: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let n = operand.read(context);
            let result = context.registers().a | n;
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: false,
                c: false,
            });
            context.registers_mut().a = result;
        },
        move |context| format!("OR {}", operand.debug(context)),
    )
}

pub fn xor(operand: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let n = operand.read(context);
            let result = context.registers().a ^ n;
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: false,
                c: false,
            });
            context.registers_mut().a = result;
        },
        move |context| format!("XOR {}", operand.debug(context)),
    )
}
