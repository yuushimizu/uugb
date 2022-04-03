mod cartridge_type;
mod cgb_flag;
mod destination;
mod licensee;
mod ram_size;
mod rom_size;
mod sgb_flag;
mod title;

pub use cartridge_type::{CartridgeType, MBCType};
pub use cgb_flag::CGBFlag;
pub use destination::Destination;
pub use licensee::Licensee;
pub use ram_size::RamSize;
pub use rom_size::RomSize;
pub use sgb_flag::SGBFlag;
pub use title::Title;

#[derive(Debug)]
pub struct Header {
    pub entry_point: [u8; 4],
    pub cgb_flag: CGBFlag,
    pub title: Title,
    pub licensee: Licensee,
    pub sgb_flag: SGBFlag,
    pub cartridge_type: CartridgeType,
    pub rom_size: RomSize,
    pub ram_size: RamSize,
    pub destination: Destination,
}

impl Header {
    pub fn load(bytes: &[u8]) -> Self {
        Self {
            entry_point: bytes[0x0100..=0x0103].try_into().unwrap(),
            cgb_flag: CGBFlag::load(bytes),
            title: Title::load(bytes),
            licensee: licensee::Licensee::load(bytes),
            sgb_flag: SGBFlag::load(bytes),
            cartridge_type: CartridgeType::load(bytes),
            rom_size: RomSize::load(bytes),
            ram_size: RamSize::load(bytes),
            destination: Destination::load(bytes),
        }
    }
}
