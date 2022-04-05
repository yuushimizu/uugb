use crate::cpu::command::{
    parameter::{DestinationRef, SourceRef},
    Content,
};

pub fn push(source: SourceRef<u16>, cycles: u64) -> Content {
    Content {
        mnemonic: "PUSH",
        execute: Box::new(|context| {
            let value = source.read(context);
            let address = context.registers().sp;
            context.write16(address, value);
            context.registers_mut().sp = address.wrapping_sub(2);
        }),
        cycles,
    }
}

pub fn pop(destination: DestinationRef<u16>, cycles: u64) -> Content {
    Content {
        mnemonic: "POP",
        execute: Box::new(|context| {
            let writer = destination.writer(context);
            let address = context.registers().sp;
            let value = context.read16(address);
            writer(context, value);
            context.registers_mut().sp = address.wrapping_add(2);
        }),
        cycles,
    }
}
