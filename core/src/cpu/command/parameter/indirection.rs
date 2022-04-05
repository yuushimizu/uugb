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

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: &Indirection = &Indirection {
            name: concat!("(", stringify!($name), ")"),
            address: |context| context.registers().$field(),
        };
    };
}

register!(BC, bc);
register!(DE, bc);
register!(HL, bc);

pub const LITERAL: &Indirection = &Indirection {
    name: "(nn)",
    address: |context| context.pop16_from_pc(),
};

pub const C: &Indirection = &Indirection {
    name: "(C)",
    address: |context| 0xFF00 | context.registers().c as u16,
};
