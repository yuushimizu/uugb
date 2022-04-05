use super::Operator;
use crate::cpu::command::operand::{ReadRef, WriteRef};

pub fn push(source: ReadRef<u16>) -> Operator {
    Operator {
        mnemonic: "PUSH",
        execute: Box::new(|context| {
            let value = source.read(context);
            let address = context.registers().sp;
            context.write16(address, value);
            context.registers_mut().sp = address.wrapping_sub(2);
        }),
    }
}

pub fn pop(destination: WriteRef<u16>) -> Operator {
    Operator {
        mnemonic: "POP",
        execute: Box::new(|context| {
            let writer = destination.writer(context);
            let address = context.registers().sp;
            let value = context.read16(address);
            writer(context, value);
            context.registers_mut().sp = address.wrapping_add(2);
        }),
    }
}
