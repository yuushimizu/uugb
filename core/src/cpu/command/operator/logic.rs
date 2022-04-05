use super::Operator;
use crate::cpu::{command::operand::ReadRef, registers::Flags};

pub fn and(operand: ReadRef<u8>) -> Operator {
    Operator {
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
    }
}

pub fn or(operand: ReadRef<u8>) -> Operator {
    Operator {
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
    }
}

pub fn xor(operand: ReadRef<u8>) -> Operator {
    Operator {
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
    }
}
