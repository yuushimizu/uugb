use super::{Operand, Read, Write};
use crate::cpu::instruction::Context;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Indirection {
    name: &'static str,
    address: fn(&mut Context) -> u16,
    debug_address: fn(&Context) -> u16,
}

impl fmt::Debug for Indirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Indirection")
            .field("name", &self.name)
            .finish()
    }
}

impl Operand for Indirection {}

impl Read<u8> for Indirection {
    fn read(&self, context: &mut Context) -> u8 {
        let address = (self.address)(context);
        context.read(address)
    }

    fn debug(&self, context: &Context) -> String {
        let address = (self.debug_address)(context);
        format!(
            "({}:{:04X}):{:02X}",
            self.name,
            address,
            context.debug_u8(address)
        )
    }
}

impl Write<u8> for Indirection {
    fn write(&self, context: &mut Context, value: u8) {
        let address = (self.address)(context);
        context.write(address, value);
    }

    fn debug(&self, context: &Context) -> String {
        let address = (self.debug_address)(context);
        format!("({}:{:04X})", self.name, address,)
    }
}

impl Read<u16> for Indirection {
    fn read(&self, context: &mut Context) -> u16 {
        let address = (self.address)(context);
        context.read16(address)
    }

    fn debug(&self, context: &Context) -> String {
        let address = (self.debug_address)(context);
        format!(
            "({}:{:04X}):{:04X}",
            self.name,
            address,
            context.debug_u16(address)
        )
    }
}

impl Write<u16> for Indirection {
    fn write(&self, context: &mut Context, value: u16) {
        let address = (self.address)(context);
        context.write16(address, value);
    }

    fn debug(&self, context: &Context) -> String {
        let address = (self.debug_address)(context);
        format!("({}:{:04X})", self.name, address,)
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: Indirection = Indirection {
            name: stringify!($name),
            address: |context| context.registers().$field(),
            debug_address: |context| context.registers().$field(),
        };
    };
}

register!(BC, bc);

register!(DE, de);

register!(HL, hl);

pub const LITERAL: Indirection = Indirection {
    name: "$",
    address: |context| context.fetch16(),
    debug_address: |context| context.debug_u16(context.registers().pc),
};

pub const LITERAL_8: Indirection = Indirection {
    name: "$FF00+$",
    address: |context| 0xFF00 | context.fetch() as u16,
    debug_address: |context| 0xFF00 | context.debug_u8(context.registers().pc) as u16,
};

pub const C: Indirection = Indirection {
    name: "C",
    address: |context| 0xFF00 | context.registers().c as u16,
    debug_address: |context| 0xFF00 | context.registers().c as u16,
};

pub const HLD: Indirection = Indirection {
    name: "HLD",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_sub(1));
        address
    },
    debug_address: |context| context.registers().hl(),
};

pub const HLI: Indirection = Indirection {
    name: "HLI",
    address: |context| {
        let address = context.registers().hl();
        context.registers_mut().set_hl(address.wrapping_add(1));
        address
    },
    debug_address: |context| context.registers().hl(),
};
