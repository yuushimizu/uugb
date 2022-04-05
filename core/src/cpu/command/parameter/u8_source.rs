use super::super::Context;
use std::fmt;

#[derive(Clone, Copy)]
pub struct U8Source {
    name: &'static str,
    read: fn(&mut dyn Context) -> u8,
}

impl U8Source {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn read(&self, context: &mut dyn Context) -> u8 {
        (self.read)(context)
    }
}

impl fmt::Debug for U8Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("U8Source")
            .field("name", &self.name)
            .finish()
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: U8Source = U8Source {
            name: stringify!($name),
            read: |context| context.registers().$field,
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
