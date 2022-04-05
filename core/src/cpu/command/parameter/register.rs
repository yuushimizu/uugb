use super::{Destination16, Destination8, Source16, Source8, Writer16, Writer8};
use crate::cpu::{Context, Registers};

macro_rules! register {
    ($name: ident, $field: ident) => {
        mod $field {
            use super::{Context, Destination8, Source8, Writer8};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Source8 for $name {
                fn read(&self, context: &mut dyn Context) -> u8 {
                    context.registers().$field
                }
            }

            impl Destination8 for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer8 {
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
            use super::{Context, Destination16, Source16, Writer16};

            #[derive(Debug, Clone)]
            pub struct $name;

            impl Source16 for $name {
                fn read(&self, context: &mut dyn Context) -> u16 {
                    context.registers().$field()
                }
            }

            impl Destination16 for $name {
                fn writer(&self, _context: &mut dyn Context) -> Writer16 {
                    Box::new(|context, value| context.registers_mut().$setter(value))
                }
            }
        }

        pub const $name: &$field::$name = &$field::$name;
    };
}

register_pair!(BC, bc, set_bc);
register_pair!(DE, de, set_de);
register_pair!(HL, hl, set_hl);

mod sp {
    use super::{Context, Destination16, Source16, Writer16};

    #[derive(Debug, Clone)]
    pub struct SP;

    impl Source16 for SP {
        fn read(&self, context: &mut dyn Context) -> u16 {
            context.registers().sp
        }
    }

    impl Destination16 for SP {
        fn writer(&self, _context: &mut dyn Context) -> Writer16 {
            Box::new(|context, value| context.registers_mut().sp = value)
        }
    }
}

pub const SP: &sp::SP = &sp::SP;
