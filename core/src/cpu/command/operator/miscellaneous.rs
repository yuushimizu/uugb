use super::Operator;
use crate::cpu::{command::operand::ReadWriteRef, registers::Flags};

pub fn swap(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new("SWAP", |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = current.rotate_left(4);
        writer(context, result);
        context.registers_mut().f = Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        }
    })
}
