use super::{Operand, Read, ReadWrite, Value, Write, Writer};
use crate::cpu::{Context, Registers};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Register<T: Value> {
    name: &'static str,
    read: fn(&Registers) -> T,
    write: fn(&mut Registers, T),
}

impl<T: Value> fmt::Display for Register<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: Value> Operand for Register<T> {}

impl<T: Value> fmt::Debug for Register<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .field("name", &self.name)
            .finish()
    }
}

impl<T: Value> Read<T> for Register<T> {
    fn read(self, context: &mut dyn Context) -> T {
        (self.read)(context.registers())
    }
}

impl<T: Value> Write<T> for Register<T> {
    fn writer(self, _context: &mut dyn Context) -> Writer<T> {
        let write = self.write;
        Box::new(move |context, value| write(context.registers_mut(), value))
    }
}

impl<T: Value> ReadWrite<T> for Register<T> {
    fn read_write(self, context: &mut dyn Context) -> (T, Writer<T>) {
        (self.read(context), self.writer(context))
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: Register<u8> = Register {
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

macro_rules! register_pair {
    ($name: ident, $field: ident, $setter: ident) => {
        pub const $name: Register<u16> = Register {
            name: stringify!($name),
            read: |registers| registers.$field(),
            write: |registers, value| registers.$setter(value),
        };
    };
}

register_pair!(AF, af, set_af);
register_pair!(BC, bc, set_bc);
register_pair!(DE, de, set_de);
register_pair!(HL, hl, set_hl);

pub const SP: Register<u16> = Register {
    name: "SP",
    read: |registers| registers.sp,
    write: |registers, value| registers.sp = value,
};