use super::Operator;
use crate::cpu::instruction::{
    operand::{register, DebugOperand, Read, Value, Write},
    Context,
};

fn load<T: Value>(context: &mut Context, destination: impl Write<T>, source: impl Read<T>) {
    let value = source.read(context);
    destination.write(context, value);
}

fn ld_generic<T: Value>(destination: impl Write<T>, source: impl Read<T>) -> Operator {
    Operator::new(
        move |context| {
            load(context, destination, source);
        },
        move |context| {
            format!(
                "LD {}, {}",
                destination.debug(context),
                source.debug(context)
            )
        },
    )
}

pub fn ld(destination: impl Write<u8>, source: impl Read<u8>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16(destination: impl Write<u16>, source: impl Read<u16>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16_sp_hl() -> Operator {
    Operator::new(
        |context| {
            load(context, register::Sp, register::Hl);
            context.wait();
        },
        |context| {
            format!(
                "LD {}, {}",
                register::Sp.debug(context),
                register::Hl.debug(context)
            )
        },
    )
}
