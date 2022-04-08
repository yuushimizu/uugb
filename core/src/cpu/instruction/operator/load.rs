use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, Value, Write},
    CpuContext,
};

fn load<T: Value>(context: &mut CpuContext, destination: impl Write<T>, source: impl Read<T>) {
    let writer = destination.prepare(context);
    let value = source.read(context);
    writer.write(context, value);
}

fn ld_generic<T: Value>(destination: impl Write<T>, source: impl Read<T>) -> Operator {
    Operator::new(format!("LD {}, {}", destination, source), move |context| {
        load(context, destination, source);
    })
}

pub fn ld(destination: impl Write<u8>, source: impl Read<u8>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16(destination: impl Write<u16>, source: impl Read<u16>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16_sp_hl() -> Operator {
    Operator::new("LD SP, HL".into(), |context| {
        load(context, register::SP, register::HL);
        context.wait();
    })
}
