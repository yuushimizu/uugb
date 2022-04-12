use super::Operator;
use crate::cpu::{
    instruction::operand::{register, DebugOperand, Read},
    registers::Flags,
};

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
        move |context| {
            format!(
                "AND {}, {}",
                register::A.debug(context),
                operand.debug(context)
            )
        },
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
        move |context| {
            format!(
                "OR {}, {}",
                register::A.debug(context),
                operand.debug(context)
            )
        },
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
        move |context| {
            format!(
                "XOR {}, {}",
                register::A.debug(context),
                operand.debug(context)
            )
        },
    )
}
