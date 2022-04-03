mod cartridge_option;
mod mbc_type;

pub use cartridge_option::CartridgeOption;
pub use mbc_type::MbcType;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CartridgeType {
    code: u8,
}

impl From<u8> for CartridgeType {
    fn from(code: u8) -> Self {
        Self { code: code }
    }
}

fn decode(code: u8) -> (MbcType, Vec<CartridgeOption>) {
    use CartridgeOption::*;
    use MbcType::*;
    match code {
        0x00 => (RomOnly, vec![]),
        0x01 => (Mbc1, vec![]),
        0x02 => (Mbc1, vec![Ram]),
        0x03 => (Mbc1, vec![Ram, Battery]),
        0x05 => (Mbc2, vec![]),
        0x06 => (Mbc2, vec![Battery]),
        0x08 => (RomOnly, vec![Ram]),
        0x09 => (RomOnly, vec![Ram, Battery]),
        0x0B => (Mmm01, vec![]),
        0x0C => (Mmm01, vec![Ram]),
        0x0D => (Mmm01, vec![Ram, Battery]),
        0x0F => (Mbc3, vec![Battery, Rtc]),
        0x10 => (Mbc3, vec![Ram, Battery, Rtc]),
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
        0x22 => (Mbc7, vec![Ram, Battery, Rumble, Sensor]),
        0xFC => (PocketCamera, vec![]),
        0xFD => (BandaiTama5, vec![]),
        0xFE => (Huc3, vec![]),
        0xFF => (Huc1, vec![Ram, Battery]),
        _ => (Unknown, vec![]),
    }
}

const POSITION: usize = 0x0147;

impl CartridgeType {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn code(&self) -> u8 {
        return self.code;
    }

    pub fn mbc_type(&self) -> MbcType {
        decode(self.code).0
    }

    pub fn options(&self) -> Vec<CartridgeOption> {
        decode(self.code).1
    }
}

impl fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} + [{}] ({:02X})",
            self.mbc_type(),
            self.options()
                .iter()
                .map(|option| format!("{}", option))
                .collect::<Vec<String>>()
                .join(", "),
            self.code
        )
    }
}
