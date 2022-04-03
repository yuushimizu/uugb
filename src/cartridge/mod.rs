pub mod header;

pub use header::Header;

use std::result;

#[derive(Debug)]
pub struct Cartridge {
    pub header: Header,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    HeaderError(header::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl Cartridge {
    pub fn load(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            header: Header::load(bytes).map_err(|err| Error::HeaderError(err))?,
        })
    }
}
