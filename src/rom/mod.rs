mod cartridge_type;
mod cgb_flag;
mod licensee;
mod sgb_flag;
mod title;

pub use cartridge_type::{CartridgeType, MBCType};
pub use cgb_flag::CGBFlag;
pub use licensee::Licensee;
pub use sgb_flag::SGBFlag;

#[derive(Debug)]
pub struct Rom {
    pub entry_point: [u8; 4],
    pub cgb_flag: CGBFlag,
    pub title: String,
    pub licensee: Licensee,
    pub sgb_flag: SGBFlag,
    pub cartridge_type: Option<CartridgeType>,
}

impl Rom {
    pub fn load(bytes: &[u8]) -> Self {
        Self {
            entry_point: bytes[0x0100..=0x0103].try_into().unwrap(),
            cgb_flag: CGBFlag::load(bytes),
            title: title::load(bytes),
            licensee: licensee::Licensee::load(bytes),
            sgb_flag: SGBFlag::load(bytes),
            cartridge_type: CartridgeType::load(bytes),
        }
    }
}
