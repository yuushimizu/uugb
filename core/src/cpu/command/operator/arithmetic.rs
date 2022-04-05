use super::Operator;
use crate::cpu::{
    command::operand::{register, ReadRef, ReadWriteRef},
    registers::Flags,
};

fn add_u8(
    mnemonic: &'static str,
    lhs: ReadWriteRef<u8>,
    rhs: ReadRef<u8>,
    with_carry: bool,
) -> Operator {
    Operator::new(
        mnemonic,
        Box::new(move |context| {
            let (current, writer) = lhs.read_and_writer(context);
            let n = rhs.read(context);
            let carry = (with_carry && context.registers().f.c) as u8;
            let (result, overflow) = current.overflowing_add(n);
            let (result, carry_overflow) = result.overflowing_add(carry);
            writer(context, result);
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: (current & 0xF) + (n & 0xF) + carry > 0xF,
                c: overflow || carry_overflow,
            };
        }),
    )
}

pub fn add(lhs: ReadWriteRef<u8>, rhs: ReadRef<u8>) -> Operator {
    add_u8("ADD", lhs, rhs, false)
}

pub fn adc(lhs: ReadWriteRef<u8>, rhs: ReadRef<u8>) -> Operator {
    add_u8("ADC", lhs, rhs, true)
}

fn sub_u8(
    mnemonic: &'static str,
    lhs: ReadWriteRef<u8>,
    rhs: ReadRef<u8>,
    with_carry: bool,
    with_result: bool,
) -> Operator {
    Operator::new(
        mnemonic,
        Box::new(move |context| {
            let (current, writer) = lhs.read_and_writer(context);
            let n = rhs.read(context);
            let carry = (with_carry && context.registers().f.c) as u8;
            let (result, overflow) = current.overflowing_sub(n);
            let (result, carry_overflow) = result.overflowing_sub(carry);
            if with_result {
                writer(context, result);
            }
            let (half_result, half_overflow) = (current & 0xF).overflowing_sub(n & 0xF);
            context.registers_mut().f = Flags {
                z: result == 0,
                n: true,
                h: half_overflow || half_result.overflowing_sub(carry).1,
                c: overflow || carry_overflow,
            }
        }),
    )
}

pub fn sub(rhs: ReadRef<u8>) -> Operator {
    sub_u8("SUB", register::A, rhs, false, true)
}

pub fn sbc(lhs: ReadWriteRef<u8>, rhs: ReadRef<u8>) -> Operator {
    sub_u8("SBC", lhs, rhs, true, true)
}

pub fn cp(rhs: ReadRef<u8>) -> Operator {
    sub_u8("CP", register::A, rhs, false, false)
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

fn add_u16(mnemonic: &'static str, lhs: ReadWriteRef<u16>, rhs: ReadRef<u16>) -> Operator {
    Operator::new(
        mnemonic,
        Box::new(|context| {
            let (current, writer) = lhs.read_and_writer(context);
            let n = rhs.read(context);
            let (result, overflow) = current.overflowing_add(n);
            writer(context, result);
            context.registers_mut().f = Flags {
                n: false,
                h: (current & 0x0FFF) + (n & 0x0FFF) > 0x0FFF,
                c: overflow,
                ..context.registers().f
            };
        }),
    )
}

pub fn add16(lhs: ReadWriteRef<u16>, rhs: ReadRef<u16>) -> Operator {
    add_u16("ADD", lhs, rhs)
}
