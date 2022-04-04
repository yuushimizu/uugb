pub struct U8 {
    read: fn() -> u8,
}

pub const B: U8 = U8 { read: { || 0 } };
