use crate::cpu::{
    command::{parameter::SourceRef, Content},
    registers::Flags,
};

fn add_generic(parameter: SourceRef<u8>, cycles: u64, add_carry: bool) -> Content {
    Content {
        mnemonic: "ADD",
        execute: Box::new(move |context| {
            let current = context.registers().a;
            let lhs = parameter.read(context);
            let (result, overflow) = current.overflowing_add(lhs);
            let carry = (add_carry && context.registers().f.c) as u8;
            let (result, carry_overflow) = result.overflowing_add(carry);
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: (current & 0xF) + (lhs & 0xF) + carry > 0xF,
                c: overflow || carry_overflow,
            };
        }),
        cycles,
    }
}

pub fn add(parameter: SourceRef<u8>, cycles: u64) -> Content {
    add_generic(parameter, cycles, false)
}

pub fn adc(parameter: SourceRef<u8>, cycles: u64) -> Content {
    add_generic(parameter, cycles, true)
}
