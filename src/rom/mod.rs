mod cgb_flag;
mod licensee;
mod title;

pub use cgb_flag::CGBFlag;
pub use licensee::Licensee;

#[derive(Debug)]
pub struct Rom {
    pub entry_point: [u8; 4],
    pub cgb_flag: CGBFlag,
    pub title: String,
    pub licensee: Licensee,
}

impl Rom {
    pub fn load(bytes: &[u8]) -> Self {
        Rom {
            entry_point: bytes[0x0100..=0x0103].try_into().unwrap(),
            cgb_flag: CGBFlag::load(bytes),
            title: title::load(bytes),
            licensee: licensee::Licensee::load(bytes),
        }
    }
}
