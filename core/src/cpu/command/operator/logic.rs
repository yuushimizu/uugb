use crate::cpu::{
    command::{parameter::ReadRef, Content},
    registers::Flags,
};

pub fn and(parameter: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "AND",
        execute: Box::new(|context| {
            let result = context.registers().a & parameter.read(context);
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

pub fn or(parameter: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "OR",
        execute: Box::new(|context| {
            let result = context.registers().a | parameter.read(context);
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: false,
                c: false,
            }
        }),
        cycles,
    }
}

pub fn xor(parameter: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "XOR",
        execute: Box::new(|context| {
            let result = context.registers().a ^ parameter.read(context);
            context.registers_mut().a = result;
            context.registers_mut().f = Flags {
                z: result == 0,
                n: false,
                h: false,
                c: false,
            }
        }),
        cycles,
    }
}
