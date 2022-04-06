use super::Operator;
use crate::cpu::{
    command::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn add_u8(mnemonic: &'static str, lhs: ReadWrite<u8>, rhs: Read<u8>, with_carry: bool) -> Operator {
    Operator::new(mnemonic, move |context| {
        let (current, writer) = lhs.read_write(context);
        let n = rhs.read(context);
        let carry = (with_carry && context.flags().c) as u8;
        let (result, overflow) = current.overflowing_add(n);
        let (result, carry_overflow) = result.overflowing_add(carry);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: (current & 0xF) + (n & 0xF) + carry > 0xF,
            c: overflow || carry_overflow,
        });
    })
}

pub fn add(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    add_u8("ADD", lhs, rhs, false)
}

pub fn adc(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    add_u8("ADC", lhs, rhs, true)
}

fn sub_u8(
    mnemonic: &'static str,
    lhs: ReadWrite<u8>,
    rhs: Read<u8>,
    with_carry: bool,
    with_result: bool,
) -> Operator {
    Operator::new(mnemonic, move |context| {
        let (current, writer) = lhs.read_write(context);
        let n = rhs.read(context);
        let carry = (with_carry && context.flags().c) as u8;
        let (result, overflow) = current.overflowing_sub(n);
        let (result, carry_overflow) = result.overflowing_sub(carry);
        if with_result {
            writer(context, result);
        }
        let (half_result, half_overflow) = (current & 0xF).overflowing_sub(n & 0xF);
        context.set_flags(Flags {
            z: result == 0,
            n: true,
            h: half_overflow || half_result.overflowing_sub(carry).1,
            c: overflow || carry_overflow,
        });
    })
}

pub fn sub(rhs: Read<u8>) -> Operator {
    sub_u8("SUB", register::A, rhs, false, true)
}

pub fn sbc(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    sub_u8("SBC", lhs, rhs, true, true)
}

pub fn cp(rhs: Read<u8>) -> Operator {
    sub_u8("CP", register::A, rhs, false, false)
}

pub fn inc(operand: ReadWrite<u8>) -> Operator {
    Operator::new("INC", |context| {
        let (current, writer) = operand.read_write(context);
        let result = current.wrapping_add(1);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: (current & 0xF) + 1 > 0xF,
            ..context.flags()
        });
    })
}

pub fn dec(operand: ReadWrite<u8>) -> Operator {
    Operator::new("DEC", |context| {
        let (current, writer) = operand.read_write(context);
        let result = current.wrapping_sub(1);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: true,
            h: (current & 0xF).overflowing_sub(1).1,
            ..context.flags()
        });
    })
}

fn add_u16(mnemonic: &'static str, lhs: ReadWrite<u16>, rhs: Read<u16>) -> Operator {
    Operator::new(mnemonic, |context| {
        let (current, writer) = lhs.read_write(context);
        let n = rhs.read(context);
        let (result, overflow) = current.overflowing_add(n);
        writer(context, result);
        context.set_flags(Flags {
            n: false,
            h: (current & 0x0FFF) + (n & 0x0FFF) > 0x0FFF,
            c: overflow,
            ..context.flags()
        });
    })
}

pub fn add16(lhs: ReadWrite<u16>, rhs: Read<u16>) -> Operator {
    add_u16("ADD", lhs, rhs)
}

pub fn inc16(operand: ReadWrite<u16>) -> Operator {
    Operator::new("INC", |context| {
        let (current, writer) = operand.read_write(context);
        writer(context, current.wrapping_add(1))
    })
}

pub fn dec16(operand: ReadWrite<u16>) -> Operator {
    Operator::new("DEC", |context| {
        let (current, writer) = operand.read_write(context);
        writer(context, current.wrapping_sub(1))
    })
}
