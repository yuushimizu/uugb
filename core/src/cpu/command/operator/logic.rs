use crate::cpu::{
    command::{parameter::SourceRef, Content},
    registers::Flags,
};

pub fn and(parameter: SourceRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "AND",
        execute: Box::new(|context| {
            let lhs = context.registers().a;
            let rhs = parameter.read(context);
            let result = lhs & rhs;
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: true,
                c: false,
            }
        }),
        cycles,
    }
}
