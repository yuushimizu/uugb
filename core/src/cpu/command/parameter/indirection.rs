use super::{Destination8, Source8, Writer8};
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

impl Source8 for Indirection {
    fn read(&self, context: &mut dyn Context) -> u8 {
        let address = (self.address)(context);
        context.memory().read(address)
    }
}

impl Destination8 for Indirection {
    fn writer(&self, context: &mut dyn Context) -> Writer8 {
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

pub const LITERAL_8: &Indirection = &Indirection {
    name: "(n)",
    address: |context| 0xFF00 | context.pop_from_pc() as u16,
};

pub const C: &Indirection = &Indirection {
    name: "(C)",
    address: |context| 0xFF00 | context.registers().c as u16,
};

pub const HLD: &Indirection = &Indirection {
    name: "(HLD)",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_sub(1));
        address
    },
};

pub const HLI: &Indirection = &Indirection {
    name: "(HLI)",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_add(1));
        address
    },
};
