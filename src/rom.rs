pub mod cgb_flag;
mod title;

pub use cgb_flag::CGBFlag;

#[derive(Debug)]
pub struct Rom {
    pub entry_point: [u8; 4],
    pub cgb_flag: CGBFlag,
    pub title: String,
}

impl Rom {
    pub fn load(bytes: &[u8]) -> Self {
        Rom {
            entry_point: bytes[0x0100..=0x0103].try_into().unwrap(),
            cgb_flag: CGBFlag::load_from(bytes),
            title: title::load_from(bytes),
        }
    }
}
