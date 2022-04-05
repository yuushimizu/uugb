use crate::cpu::{
    command::{
        operand::{ReadRef, ReadWriteRef},
        Content,
    },
    registers::Flags,
};

fn add_generic(
    mnemonic: &'static str,
    operand: ReadRef<u8>,
    cycles: u64,
    with_carry: bool,
) -> Content {
    Content {
        mnemonic,
        execute: Box::new(move |context| {
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
        cycles,
    }
}

pub fn add(operand: ReadRef<u8>, cycles: u64) -> Content {
    add_generic("ADD", operand, cycles, false)
}

pub fn adc(operand: ReadRef<u8>, cycles: u64) -> Content {
    add_generic("ADC", operand, cycles, true)
}

fn sub_generic(
    mnemonic: &'static str,
    operand: ReadRef<u8>,
    cycles: u64,
    with_carry: bool,
    with_result: bool,
) -> Content {
    Content {
        mnemonic,
        execute: Box::new(move |context| {
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
        cycles,
    }
}

pub fn sub(operand: ReadRef<u8>, cycles: u64) -> Content {
    sub_generic("SUB", operand, cycles, false, true)
}

pub fn sbc(operand: ReadRef<u8>, cycles: u64) -> Content {
    sub_generic("SBC", operand, cycles, true, true)
}

pub fn cp(operand: ReadRef<u8>, cycles: u64) -> Content {
    sub_generic("CP", operand, cycles, false, false)
}

pub fn inc(operand: ReadWriteRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "INC",
        execute: Box::new(|context| {
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
        cycles,
    }
}

pub fn dec(operand: ReadWriteRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "INC",
        execute: Box::new(|context| {
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
        cycles,
    }
}
