pub mod header;

pub use header::Header;

#[derive(Debug)]
pub struct Rom {
    pub header: Header,
}

impl Rom {
    pub fn load(bytes: &[u8]) -> Self {
        Self {
            header: Header::load(bytes),
        }
    }
}
