use super::Operator;
use crate::cpu::{
    command::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn and_u8<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    Operator::new("AND", move |context| {
        let (current, writer) = lhs.read_write(context);
        let result = current & rhs.read(context);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: true,
            c: false,
        });
    })
}

pub fn and<R: Read<u8>>(rhs: R) -> Operator {
    and_u8(register::A, rhs)
}

fn or_u8<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    Operator::new("OR", move |context| {
        let (current, writer) = lhs.read_write(context);
        let result = current | rhs.read(context);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
    })
}

pub fn or<R: Read<u8>>(rhs: R) -> Operator {
    or_u8(register::A, rhs)
}

fn xor_u8<L: ReadWrite<u8>, R: Read<u8>>(lhs: L, rhs: R) -> Operator {
    Operator::new("XOR", move |context| {
        let (current, writer) = lhs.read_write(context);
        let result = current ^ rhs.read(context);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
    })
}

pub fn xor<R: Read<u8>>(rhs: R) -> Operator {
    xor_u8(register::A, rhs)
}
