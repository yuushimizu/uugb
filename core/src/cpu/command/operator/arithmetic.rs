use super::Operator;
use crate::cpu::{
    command::operand::{ReadRef, ReadWriteRef},
    registers::Flags,
};

fn add_generic(mnemonic: &'static str, operand: ReadRef<u8>, with_carry: bool) -> Operator {
    Operator::new(
        mnemonic,
        Box::new(move |context| {
            let lhs = context.registers().a;
            let rhs = operand.read(context);
            let (result, overflow) = lhs.overflowing_add(rhs);
            let carry = (with_carry && context.registers().f.c) as u8;
            let (result, carry_overflow) = result.overflowing_add(carry);
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: (lhs & 0xF) + (rhs & 0xF) + carry > 0xF,
                c: overflow || carry_overflow,
            };
        }),
    )
}

pub fn add(operand: ReadRef<u8>) -> Operator {
    add_generic("ADD", operand, false)
}

pub fn adc(operand: ReadRef<u8>) -> Operator {
    add_generic("ADC", operand, true)
}

fn sub_generic(
    mnemonic: &'static str,
    operand: ReadRef<u8>,
    with_carry: bool,
    with_result: bool,
) -> Operator {
    Operator::new(
        mnemonic,
        Box::new(move |context| {
            let current = context.registers().a;
            let rhs = operand.read(context);
            let (result, overflow) = current.overflowing_sub(rhs);
            let carry = (with_carry && context.registers().f.c) as u8;
            let (result, carry_overflow) = result.overflowing_sub(carry);
            if with_result {
                context.registers_mut().a = result;
            }
            let (half_result, half_overflow) = (current & 0xF).overflowing_sub(rhs & 0xF);
            context.registers_mut().f = Flags {
                z: result == 0,
                n: true,
                h: half_overflow || half_result.overflowing_sub(carry).1,
                c: overflow || carry_overflow,
            }
        }),
    )
}

pub fn sub(operand: ReadRef<u8>) -> Operator {
    sub_generic("SUB", operand, false, true)
}

pub fn sbc(operand: ReadRef<u8>) -> Operator {
    sub_generic("SBC", operand, true, true)
}

pub fn cp(operand: ReadRef<u8>) -> Operator {
    sub_generic("CP", operand, false, false)
}

pub fn inc(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new(
        "INC",
        Box::new(|context| {
            let (current, writer) = operand.read_and_writer(context);
            let result = current.wrapping_add(1);
            writer(context, result);
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: (current & 0xF) + 1 > 0xF,
                ..context.registers().f
            };
        }),
    )
}

pub fn dec(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new(
        "DEC",
        Box::new(|context| {
            let (current, writer) = operand.read_and_writer(context);
            let result = current.wrapping_sub(1);
            writer(context, result);
            context.registers_mut().f = Flags {
                z: result == 0,
                n: true,
                h: (current & 0xF).overflowing_sub(1).1,
                ..context.registers().f
            };
        }),
    )
}
