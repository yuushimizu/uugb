use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn add_u8<L: ReadWrite<u8>, R: Read<u8>>(
    mnemonic: &'static str,
    lhs: L,
    rhs: R,
    with_carry: bool,
) -> Operator {
    Operator::new(format!("{} {}, {}", mnemonic, lhs, rhs), move |context| {
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

pub fn add<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    add_u8("ADD", lhs, rhs, false)
}

pub fn adc<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    add_u8("ADC", lhs, rhs, true)
}

fn sub_u8<L: ReadWrite<u8>, R: Read<u8>>(
    format: String,
    lhs: L,
    rhs: R,
    with_carry: bool,
    with_result: bool,
) -> Operator {
    Operator::new(format, move |context| {
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

pub fn sub<O: Read<u8>>(operand: O) -> Operator {
    sub_u8(
        format!("SUB {}", operand),
        register::A,
        operand,
        false,
        true,
    )
}

pub fn sbc<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    sub_u8(format!("SBC {}, {}", lhs, rhs), lhs, rhs, true, true)
}

pub fn cp<O: Read<u8>>(operand: O) -> Operator {
    sub_u8(
        format!("CP {}", operand),
        register::A,
        operand,
        false,
        false,
    )
}

pub fn inc<O: ReadWrite<u8>>(operand: O) -> Operator {
    Operator::new(format!("INC {}", operand), move |context| {
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

pub fn dec<O: ReadWrite<u8>>(operand: O) -> Operator {
    Operator::new(format!("DEC {}", operand), move |context| {
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

fn add_u16<L: ReadWrite<u16>, R: Read<u16>>(lhs: L, rhs: R) -> Operator {
    Operator::new(format!("ADD {}, {}", lhs, rhs), move |context| {
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

pub fn add16<L: ReadWrite<u16>, R: Read<u16>>(lhs: L, rhs: R) -> Operator {
    add_u16(lhs, rhs)
}

pub fn inc16<O: ReadWrite<u16>>(operand: O) -> Operator {
    Operator::new(format!("INC {}", operand), move |context| {
        let (current, writer) = operand.read_write(context);
        writer(context, current.wrapping_add(1))
    })
}

pub fn dec16<O: ReadWrite<u16>>(operand: O) -> Operator {
    Operator::new(format!("DEC {}", operand), move |context| {
        let (current, writer) = operand.read_write(context);
        writer(context, current.wrapping_sub(1))
    })
}
