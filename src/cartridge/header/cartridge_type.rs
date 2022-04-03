#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartridgeType {
    pub code: u8,
}

impl From<u8> for CartridgeType {
    fn from(code: u8) -> Self {
        Self { code: code }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MbcType {
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

const POSITION: usize = 0x0147;

impl CartridgeType {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    fn decode(self: &Self) -> (MbcType, Vec<CartridgeOption>) {
        use CartridgeOption::*;
        use MbcType::*;
        match self.code {
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
        }
    }

    pub fn mbc_type(self: &Self) -> MbcType {
        self.decode().0
    }

    pub fn options(self: &Self) -> Vec<CartridgeOption> {
        self.decode().1
    }
}
