use super::super::super::Registers;
use super::{Context, U8Destination, U8Source, U8Writer};
use std::fmt;

#[derive(Clone)]
pub struct Register {
    name: &'static str,
    read: fn(&Registers) -> u8,
    write: fn(&mut Registers, value: u8),
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .field("name", &self.name)
            .finish()
    }
}

impl U8Source for Register {
    fn read(&self, context: &mut dyn Context) -> u8 {
        (self.read)(context.registers())
    }
}

impl U8Destination for Register {
    fn writer(&self, _context: &mut dyn Context) -> U8Writer {
        let write = self.write;
        Box::new(move |context, value| (write)(context.registers_mut(), value))
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: &Register = &Register {
            name: stringify!($name),
            read: |registers| registers.$field,
            write: |registers, value| registers.$field = value,
        };
    };
}

register!(A, a);
register!(B, b);
register!(C, c);
register!(D, d);
register!(E, e);
register!(H, h);
register!(L, l);
