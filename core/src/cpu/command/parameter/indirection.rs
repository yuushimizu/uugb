use super::{U8Destination, U8Source, U8Writer};
use crate::cpu::Context;
use std::fmt;

#[derive(Clone)]
pub struct Indirection {
    name: &'static str,
    address: fn(&mut dyn Context) -> u16,
}

impl fmt::Debug for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Indirection")
            .field("name", &self.name)
            .finish()
    }
}

impl U8Source for Indirection {
    fn read(&self, context: &mut dyn Context) -> u8 {
        let address = (self.address)(context);
        context.memory().read(address)
    }
}

impl U8Destination for Indirection {
    fn writer(&self, context: &mut dyn Context) -> U8Writer {
        let address = (self.address)(context);
        Box::new(move |context, value| context.memory_mut().write(address, value))
    }
}

pub const HL: &Indirection = &Indirection {
    name: "HL",
    address: |context| context.registers().hl(),
};
