use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MbcType {
    Unknown,
    RomOnly,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Mmm01,
    Huc1,
    Huc3,
    PocketCamera,
    BandaiTama5,
}

impl fmt::Display for MbcType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MbcType::*;
        write!(
            f,
            "{}",
            match self {
                Unknown => "Unknown",
                RomOnly => "ROM ONLY",
                Mbc1 => "MBC1",
                Mbc2 => "MBC2",
                Mbc3 => "MBC3",
                Mbc5 => "MBC5",
                Mbc6 => "MBC6",
                Mbc7 => "MBC7",
                Mmm01 => "MMM01",
                Huc1 => "Hudson HuC-1",
                Huc3 => "Hudson HuC-3",
                PocketCamera => "Pocket Camera",
                BandaiTama5 => "Bandai TAMA5",
            }
        )
    }
}
