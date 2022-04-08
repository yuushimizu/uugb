use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, Write},
    registers::Flags,
};

fn add_u8(
    mnemonic: &'static str,
    lhs: impl Read<u8> + Write<u8>,
    rhs: impl Read<u8>,
    with_carry: bool,
) -> Operator {
    Operator::new(format!("{} {}, {}", mnemonic, lhs, rhs), move |context| {
        let current = lhs.read(context);
        let n = rhs.read(context);
        let carry = (with_carry && context.flags().c) as u8;
        let (result, overflow) = current.overflowing_add(n);
        let (result, carry_overflow) = result.overflowing_add(carry);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: (current & 0xF) + (n & 0xF) + carry > 0xF,
            c: overflow || carry_overflow,
        });
        lhs.write(context, result);
    })
}

pub fn add(lhs: impl Read<u8> + Write<u8>, rhs: impl Read<u8>) -> Operator {
    add_u8("ADD", lhs, rhs, false)
}

pub fn adc(lhs: impl Read<u8> + Write<u8>, rhs: impl Read<u8>) -> Operator {
    add_u8("ADC", lhs, rhs, true)
}

fn sub_u8(
    format: String,
    lhs: impl Read<u8> + Write<u8>,
    rhs: impl Read<u8>,
    with_carry: bool,
    with_result: bool,
) -> Operator {
    Operator::new(format, move |context| {
        let current = lhs.read(context);
        let n = rhs.read(context);
        let carry = (with_carry && context.flags().c) as u8;
        let (result, overflow) = current.overflowing_sub(n);
        let (result, carry_overflow) = result.overflowing_sub(carry);
        let (half_result, half_overflow) = (current & 0xF).overflowing_sub(n & 0xF);
        context.set_flags(Flags {
            z: result == 0,
            n: true,
            h: half_overflow || half_result.overflowing_sub(carry).1,
            c: overflow || carry_overflow,
        });
        if with_result {
            lhs.write(context, result);
        }
    })
}

pub fn sub(operand: impl Read<u8>) -> Operator {
    sub_u8(
        format!("SUB {}", operand),
        register::A,
        operand,
        false,
        true,
    )
}

pub fn sbc(lhs: impl Read<u8> + Write<u8>, rhs: impl Read<u8>) -> Operator {
    sub_u8(format!("SBC {}, {}", lhs, rhs), lhs, rhs, true, true)
}

pub fn cp(operand: impl Read<u8>) -> Operator {
    sub_u8(
        format!("CP {}", operand),
        register::A,
        operand,
        false,
        false,
    )
}

pub fn inc(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("INC {}", operand), move |context| {
        let current = operand.read(context);
        let result = current.wrapping_add(1);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: (current & 0xF) + 1 > 0xF,
            ..*context.flags()
        });
        operand.write(context, result);
    })
}

pub fn dec(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("DEC {}", operand), move |context| {
        let current = operand.read(context);
        let result = current.wrapping_sub(1);
        context.set_flags(Flags {
            z: result == 0,
            n: true,
            h: (current & 0xF).overflowing_sub(1).1,
            ..*context.flags()
        });
        operand.write(context, result);
    })
}

pub fn add16(lhs: impl Read<u16> + Write<u16>, rhs: impl Read<u16>) -> Operator {
    Operator::new(format!("ADD {}, {}", lhs, rhs), move |context| {
        let current = lhs.read(context);
        let n = rhs.read(context);
        let (result, overflow) = current.overflowing_add(n);
        context.set_flags(Flags {
            n: false,
            h: (current & 0x0FFF) + (n & 0x0FFF) > 0x0FFF,
            c: overflow,
            ..*context.flags()
        });
        lhs.write(context, result);
        context.wait();
    })
}

pub fn inc16(operand: impl Read<u16> + Write<u16>) -> Operator {
    Operator::new(format!("INC {}", operand), move |context| {
        let current = operand.read(context);
        operand.write(context, current.wrapping_add(1));
        context.wait();
    })
}

pub fn dec16(operand: impl Read<u16> + Write<u16>) -> Operator {
    Operator::new(format!("DEC {}", operand), move |context| {
        let current = operand.read(context);
        operand.write(context, current.wrapping_sub(1));
        context.wait();
    })
}
