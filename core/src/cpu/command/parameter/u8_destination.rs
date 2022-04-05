use super::super::Context;
use std::fmt;

pub type Writer = fn(context: &mut dyn Context, value: u8);

#[derive(Clone, Copy)]
pub struct U8Destination {
    name: &'static str,
    writer: fn(context: &mut dyn Context) -> Writer,
}

impl U8Destination {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn writer(&self, context: &mut dyn Context) -> Writer {
        (self.writer)(context)
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
            writer: |_context| |context, value| context.registers_mut().$field = value,
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
