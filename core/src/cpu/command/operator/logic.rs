use crate::cpu::{
    command::{operand::ReadRef, Content},
    registers::Flags,
};

pub fn and(operand: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "AND",
        execute: Box::new(|context| {
            let result = context.registers().a & operand.read(context);
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

pub fn or(operand: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "OR",
        execute: Box::new(|context| {
            let result = context.registers().a | operand.read(context);
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

pub fn xor(operand: ReadRef<u8>, cycles: u64) -> Content {
    Content {
        mnemonic: "XOR",
        execute: Box::new(|context| {
            let result = context.registers().a ^ operand.read(context);
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
