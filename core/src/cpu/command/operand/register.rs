use super::{Read, ReadWrite, Write, Writer};
use crate::cpu::Context;

macro_rules! register {
    ($name: ident, $field: ident) => {
        mod $field {
            use super::{Context, Read, ReadWrite, Write, Writer};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Read<u8> for $name {
                fn read(&self, context: &mut dyn Context) -> u8 {
                    context.registers().$field
                }
            }

            impl Write<u8> for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer<u8> {
                    Box::new(|context, value| context.registers_mut().$field = value)
                }
            }

            impl ReadWrite<u8> for $name {
                fn read_write(&self, context: &mut dyn Context) -> (u8, Writer<u8>) {
                    (self.read(context), self.writer(context))
                }
            }
        }

        pub const $name: &$field::$name = &$field::$name;
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
        mod $field {
            use super::{Context, Read, ReadWrite, Write, Writer};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Read<u16> for $name {
                fn read(&self, context: &mut dyn Context) -> u16 {
                    context.registers().$field()
                }
            }

            impl Write<u16> for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer<u16> {
                    Box::new(|context, value| context.registers_mut().$setter(value))
                }
            }

            impl ReadWrite<u16> for $name {
                fn read_write(&self, context: &mut dyn Context) -> (u16, Writer<u16>) {
                    (self.read(context), self.writer(context))
                }
            }
        }

        pub const $name: &$field::$name = &$field::$name;
    };
}

register_pair!(AF, af, set_af);
register_pair!(BC, bc, set_bc);
register_pair!(DE, de, set_de);
register_pair!(HL, hl, set_hl);

mod sp {
    use super::{Context, Read, ReadWrite, Write, Writer};

    #[derive(Debug, Clone)]
    pub struct SP;

    impl Read<u16> for SP {
        fn read(&self, context: &mut dyn Context) -> u16 {
            context.registers().sp
        }
    }

    impl Write<u16> for SP {
        fn writer(&self, _context: &mut dyn Context) -> Writer<u16> {
            Box::new(|context, value| context.registers_mut().sp = value)
        }
    }

    impl ReadWrite<u16> for SP {
        fn read_write(&self, context: &mut dyn Context) -> (u16, Writer<u16>) {
            (self.read(context), self.writer(context))
        }
    }
}

pub const SP: &sp::SP = &sp::SP;
