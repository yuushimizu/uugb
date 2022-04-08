use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, ReadWrite},
    registers::Flags,
};

fn and_u8<L: ReadWrite<u8>, R: Read<u8>>(format: String, lhs: L, rhs: R) -> Operator {
    Operator::new(format, move |context| {
        lhs.prepare_and_read(context)
            .then(move |context, (current, writer)| {
                rhs.read(context).then(move |context, n| {
                    let result = current & n;
                    context.set_flags(Flags {
                        z: result == 0,
                        n: false,
                        h: true,
                        c: false,
                    });
                    writer.write(context, result)
                })
            })
    })
}

pub fn and<R: Read<u8>>(rhs: R) -> Operator {
    and_u8(format!("AND {}", rhs), register::A, rhs)
}

fn or_u8<L: ReadWrite<u8>, R: Read<u8>>(format: String, lhs: L, rhs: R) -> Operator {
    Operator::new(format, move |context| {
        lhs.prepare_and_read(context)
            .then(move |context, (current, writer)| {
                rhs.read(context).then(move |context, n| {
                    let result = current | n;
                    context.set_flags(Flags {
                        z: result == 0,
                        n: false,
                        h: false,
                        c: false,
                    });
                    writer.write(context, result)
                })
            })
    })
}

pub fn or<R: Read<u8>>(rhs: R) -> Operator {
    or_u8(format!("OR {}", rhs), register::A, rhs)
}

fn xor_u8<L: ReadWrite<u8>, R: Read<u8>>(format: String, lhs: L, rhs: R) -> Operator {
    Operator::new(format, move |context| {
        lhs.prepare_and_read(context)
            .then(move |context, (current, writer)| {
                rhs.read(context).then(move |context, n| {
                    let result = current ^ n;
                    context.set_flags(Flags {
                        z: result == 0,
                        n: false,
                        h: false,
                        c: false,
                    });
                    writer.write(context, result)
                })
            })
    })
}

pub fn xor<R: Read<u8>>(rhs: R) -> Operator {
    xor_u8(format!("XOR {}", rhs), register::A, rhs)
}
