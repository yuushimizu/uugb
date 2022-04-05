use super::{Destination, Source, Writer};
use crate::cpu::Context;

macro_rules! register {
    ($name: ident, $field: ident) => {
        mod $field {
            use super::{Context, Destination, Source, Writer};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Source<u8> for $name {
                fn read(&self, context: &mut dyn Context) -> u8 {
                    context.registers().$field
                }
            }

            impl Destination<u8> for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer<u8> {
                    Box::new(|context, value| context.registers_mut().$field = value)
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
            use super::{Context, Destination, Source, Writer};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Source<u16> for $name {
                fn read(&self, context: &mut dyn Context) -> u16 {
                    context.registers().$field()
                }
            }

            impl Destination<u16> for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer<u16> {
                    Box::new(|context, value| context.registers_mut().$setter(value))
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
    use super::{Context, Destination, Source, Writer};

    #[derive(Debug, Clone)]
    pub struct SP;

    impl Source<u16> for SP {
        fn read(&self, context: &mut dyn Context) -> u16 {
            context.registers().sp
        }
    }

    impl Destination<u16> for SP {
        fn writer(&self, _context: &mut dyn Context) -> Writer<u16> {
            Box::new(|context, value| context.registers_mut().sp = value)
        }
    }
}

pub const SP: &sp::SP = &sp::SP;
