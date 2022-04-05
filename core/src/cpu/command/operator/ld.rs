use crate::cpu::command::{
    parameter::{Destination, Source},
    Content,
};

fn ld_generic<T: Copy>(
    destination: &'static dyn Destination<T>,
    source: &'static dyn Source<T>,
    cycles: u64,
) -> Content {
    Content {
        mnemonic: "LD",
        execute: Box::new(|context| {
            let writer = destination.writer(context);
            let value = source.read(context);
            writer(context, value);
        }),
        cycles,
    }
}

pub fn ld(
    destination: &'static dyn Destination<u8>,
    source: &'static dyn Source<u8>,
    cycles: u64,
) -> Content {
    ld_generic(destination, source, cycles)
}

pub fn ld16(
    destination: &'static dyn Destination<u16>,
    source: &'static dyn Source<u16>,
    cycles: u64,
) -> Content {
    ld_generic(destination, source, cycles)
}
