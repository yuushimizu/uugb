use crate::cpu::{
    command::{parameter::SourceRef, Content},
    registers::Flags,
};

pub fn add(parameter: SourceRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "ADD",
        execute: Box::new(|context| {
            let current = context.registers().a;
            let lhs = parameter.read(context);
            let (result, overflow) = current.overflowing_add(lhs);
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: (current & 0xF) + (lhs & 0xF) > 0xF,
                c: overflow,
            };
        }),
        cycles,
    }
}
