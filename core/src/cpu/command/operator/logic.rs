use super::Operator;
use crate::cpu::{
    command::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn and_u8(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    Operator::new("AND", |context| {
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

pub fn and(rhs: Read<u8>) -> Operator {
    and_u8(register::A, rhs)
}

fn or_u8(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    Operator::new("OR", |context| {
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

pub fn or(rhs: Read<u8>) -> Operator {
    or_u8(register::A, rhs)
}

fn xor_u8(lhs: ReadWrite<u8>, rhs: Read<u8>) -> Operator {
    Operator::new("XOR", |context| {
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

pub fn xor(rhs: Read<u8>) -> Operator {
    xor_u8(register::A, rhs)
}
