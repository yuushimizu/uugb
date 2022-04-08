use super::Operator;
use crate::cpu::{
    instruction::operand::{register, Read, Value, Write},
    CpuContext,
};

fn load<T: Value, D: Write<T>, S: Read<T>>(
    context: &mut dyn CpuContext,
    destination: D,
    source: S,
) {
    let writer = destination.prepare(context);
    let value = source.read(context);
    writer.write(context, value);
}

fn ld_generic<T: Value, D: Write<T>, S: Read<T>>(destination: D, source: S) -> Operator {
    Operator::new(format!("LD {}, {}", destination, source), move |context| {
        load(context, destination, source);
    })
}

pub fn ld<D: Write<u8>, S: Read<u8>>(destination: D, source: S) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16<D: Write<u16>, S: Read<u16>>(destination: D, source: S) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16_sp_hl() -> Operator {
    Operator::new("LD SP, HL".into(), |context| {
        load(context, register::SP, register::HL);
        context.wait();
    })
}
