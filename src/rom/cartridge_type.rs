use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MBCType {
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

fn from_value(value: u8) -> Option<CartridgeType> {
    use CartridgeOption::*;
    use MBCType::*;
    match value {
        0x00 => Some((NoMBC, vec![])),
        0x01 => Some((Mbc1, vec![])),
        0x02 => Some((Mbc1, vec![Ram])),
        0x03 => Some((Mbc1, vec![Ram, Battery])),
        0x05 => Some((Mbc2, vec![])),
        0x06 => Some((Mbc2, vec![Battery])),
        0x08 => Some((NoMBC, vec![Ram])),
        0x09 => Some((NoMBC, vec![Ram, Battery])),
        0x0B => Some((Mmm01, vec![])),
        0x0C => Some((Mmm01, vec![Ram])),
        0x0D => Some((Mmm01, vec![Ram, Battery])),
        0x0F => Some((Mbc3, vec![Battery, Timer])),
        0x10 => Some((Mbc3, vec![Ram, Battery, Timer])),
        0x11 => Some((Mbc3, vec![])),
        0x12 => Some((Mbc3, vec![Ram])),
        0x13 => Some((Mbc3, vec![Ram, Battery])),
        0x19 => Some((Mbc5, vec![])),
        0x1A => Some((Mbc5, vec![Ram])),
        0x1B => Some((Mbc5, vec![Ram, Battery])),
        0x1C => Some((Mbc5, vec![Rumble])),
        0x1D => Some((Mbc5, vec![Ram, Rumble])),
        0x1E => Some((Mbc5, vec![Ram, Battery, Rumble])),
        0x20 => Some((Mbc6, vec![])),
        0x22 => Some((Mbc7, vec![Ram, Battery, Rumble, Accelerometer])),
        0xFC => Some((GameBoyCamera, vec![])),
        0xFD => Some((BandaiTama5, vec![])),
        0xFE => Some((HuC3, vec![])),
        0xFF => Some((HuC1, vec![Ram, Battery])),
        _ => None,
    }
    .map(|(mbc_type, options)| CartridgeType {
        mbc_type,
        options: HashSet::from_iter(options),
    })
}

const POSITION: usize = 0x0147;

impl CartridgeType {
    pub fn load(rom_bytes: &[u8]) -> Option<Self> {
        from_value(rom_bytes[POSITION])
    }
}
