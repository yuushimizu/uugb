use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MBCType {
    Unknown,
    NoMBC,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Mmm01,
    HuC1,
    HuC3,
    GameBoyCamera,
    BandaiTama5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CartridgeOption {
    Ram,
    Battery,
    Timer,
    Rumble,
    Accelerometer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartridgeType {
    pub mbc_type: MBCType,
    pub options: HashSet<CartridgeOption>,
}

impl From<u8> for CartridgeType {
    fn from(value: u8) -> Self {
        use CartridgeOption::*;
        use MBCType::*;
        let (mbc_type, options) = match value {
            0x00 => (NoMBC, vec![]),
            0x01 => (Mbc1, vec![]),
            0x02 => (Mbc1, vec![Ram]),
            0x03 => (Mbc1, vec![Ram, Battery]),
            0x05 => (Mbc2, vec![]),
            0x06 => (Mbc2, vec![Battery]),
            0x08 => (NoMBC, vec![Ram]),
            0x09 => (NoMBC, vec![Ram, Battery]),
            0x0B => (Mmm01, vec![]),
            0x0C => (Mmm01, vec![Ram]),
            0x0D => (Mmm01, vec![Ram, Battery]),
            0x0F => (Mbc3, vec![Battery, Timer]),
            0x10 => (Mbc3, vec![Ram, Battery, Timer]),
            0x11 => (Mbc3, vec![]),
            0x12 => (Mbc3, vec![Ram]),
            0x13 => (Mbc3, vec![Ram, Battery]),
            0x19 => (Mbc5, vec![]),
            0x1A => (Mbc5, vec![Ram]),
            0x1B => (Mbc5, vec![Ram, Battery]),
            0x1C => (Mbc5, vec![Rumble]),
            0x1D => (Mbc5, vec![Ram, Rumble]),
            0x1E => (Mbc5, vec![Ram, Battery, Rumble]),
            0x20 => (Mbc6, vec![]),
            0x22 => (Mbc7, vec![Ram, Battery, Rumble, Accelerometer]),
            0xFC => (GameBoyCamera, vec![]),
            0xFD => (BandaiTama5, vec![]),
            0xFE => (HuC3, vec![]),
            0xFF => (HuC1, vec![Ram, Battery]),
            _ => (Unknown, vec![]),
        };
        Self {
            mbc_type,
            options: HashSet::from_iter(options),
        }
    }
}

const POSITION: usize = 0x0147;

impl CartridgeType {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}
