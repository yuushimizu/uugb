use super::super::Context;
use std::fmt;

#[derive(Clone, Copy)]
pub struct U8Destination {
    name: &'static str,
    write: fn(&mut dyn Context, u8),
}

impl U8Destination {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn write(&self, context: &mut dyn Context, value: u8) {
        (self.write)(context, value)
    }
}

impl fmt::Debug for U8Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("U8Destination")
            .field("name", &self.name)
            .finish()
    }
}

macro_rules! register {
    ($name: ident, $field: ident) => {
        pub const $name: U8Destination = U8Destination {
            name: stringify!($name),
            write: |context, value| {
                context.registers_mut().$field = value;
            },
        };
    };
}

register!(B, b);
register!(C, c);
register!(D, d);
register!(E, e);
register!(H, h);
register!(L, l);
