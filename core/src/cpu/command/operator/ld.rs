use crate::cpu::command::{
    parameter::{DestinationRef, SourceRef},
    Content,
};

fn ld_generic<T: Copy>(
    destination: DestinationRef<T>,
    source: SourceRef<T>,
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

pub fn ld(destination: DestinationRef<u8>, source: SourceRef<u8>, cycles: u64) -> Content {
    ld_generic(destination, source, cycles)
}

pub fn ld16(destination: DestinationRef<u16>, source: SourceRef<u16>, cycles: u64) -> Content {
    ld_generic(destination, source, cycles)
}
