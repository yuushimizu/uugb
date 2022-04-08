use super::{Continuation, Operand, Read, ReadWrite, Write, Writer};
use crate::cpu::CpuContext;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Indirection {
    name: &'static str,
    address: fn(&mut dyn CpuContext) -> Continuation<u16>,
}

impl fmt::Debug for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Indirection")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}

impl Operand for Indirection {}

impl Read<u8> for Indirection {
    fn read(self, context: &mut dyn CpuContext) -> Continuation<u8> {
        (self.address)(context).then(|context, address| context.read(address))
    }
}

impl Write<u8> for Indirection {
    fn prepare(self, context: &mut dyn CpuContext) -> Continuation<Writer<u8>> {
        (self.address)(context).map(|_context, address| {
            Writer::new(move |context, value| context.write(address, value))
        })
    }
}

impl ReadWrite<u8> for Indirection {
    fn prepare_and_read(self, context: &mut dyn CpuContext) -> Continuation<(u8, Writer<u8>)> {
        (self.address)(context).then(|context, address| {
            context.read(address).map(move |_context, value| {
                (
                    value,
                    Writer::new(move |context, value| context.write(address, value)),
                )
            })
        })
    }
}

impl Read<u16> for Indirection {
    fn read(self, context: &mut dyn CpuContext) -> Continuation<u16> {
        (self.address)(context).then(|context, address| context.read16(address))
    }
}

impl Write<u16> for Indirection {
    fn prepare(self, context: &mut dyn CpuContext) -> Continuation<Writer<u16>> {
        (self.address)(context).map(|_context, address| {
            Writer::new(move |context, value| context.write16(address, value))
        })
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: Indirection = Indirection {
            name: stringify!($name),
            address: |context| Continuation::just(context.registers().$field()),
        };
    };
}

register!(BC, bc);
register!(DE, bc);
register!(HL, bc);

pub const LITERAL: Indirection = Indirection {
    name: "nn",
    address: |context| context.fetch16(),
};

pub const LITERAL_8: Indirection = Indirection {
    name: "$FF00+n",
    address: |context| context.fetch().map(|_context, value| 0xFF00 | value as u16),
};

pub const C: Indirection = Indirection {
    name: "C",
    address: |context| Continuation::just(0xFF00 | context.registers().c as u16),
};

pub const HLD: Indirection = Indirection {
    name: "HLD",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_sub(1));
        Continuation::just(address)
    },
};

pub const HLI: Indirection = Indirection {
    name: "HLI",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_add(1));
        Continuation::just(address)
    },
};
