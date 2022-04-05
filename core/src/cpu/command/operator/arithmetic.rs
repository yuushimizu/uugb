use crate::cpu::{
    command::{parameter::SourceRef, Content},
    registers::Flags,
};

fn add_generic(
    mnemonic: &'static str,
    parameter: SourceRef<u8>,
    cycles: u64,
    with_carry: bool,
) -> Content {
    Content {
        mnemonic,
        execute: Box::new(move |context| {
            let lhs = context.registers().a;
            let rhs = parameter.read(context);
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

pub fn add(parameter: SourceRef<u8>, cycles: u64) -> Content {
    add_generic("ADD", parameter, cycles, false)
}

pub fn adc(parameter: SourceRef<u8>, cycles: u64) -> Content {
    add_generic("ADC", parameter, cycles, true)
}

fn sub_generic(
    mnemonic: &'static str,
    parameter: SourceRef<u8>,
    cycles: u64,
    with_carry: bool,
) -> Content {
    Content {
        mnemonic,
        execute: Box::new(move |context| {
            let current = context.registers().a;
            let rhs = parameter.read(context);
            let (result, overflow) = current.overflowing_sub(rhs);
            let carry = (with_carry && context.registers().f.c) as u8;
            let (result, carry_overflow) = result.overflowing_sub(carry);
            context.registers_mut().a = result;
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

pub fn sub(parameter: SourceRef<u8>, cycles: u64) -> Content {
    sub_generic("SUB", parameter, cycles, false)
}

pub fn sbc(parameter: SourceRef<u8>, cycles: u64) -> Content {
    sub_generic("SBC", parameter, cycles, true)
}
