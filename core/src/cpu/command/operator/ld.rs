use crate::cpu::command::{
    parameter::{ReadRef, WriteRef},
    Content,
};

fn ld_generic<T: Copy>(destination: WriteRef<T>, source: ReadRef<T>, cycles: u64) -> Content {
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

pub fn ld(destination: WriteRef<u8>, source: ReadRef<u8>, cycles: u64) -> Content {
    ld_generic(destination, source, cycles)
}

pub fn ld16(destination: WriteRef<u16>, source: ReadRef<u16>, cycles: u64) -> Content {
    ld_generic(destination, source, cycles)
}
