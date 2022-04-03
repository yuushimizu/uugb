mod header;
mod mbc;

pub use header::*;
pub use mbc::Mbc;

use std::result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cartridge {
    rom: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HeaderError(header::Error),
    MbcNotImplemented(CartridgeType),
}

pub type Result<T> = result::Result<T, Error>;

impl Cartridge {
    pub fn new(rom: &[u8]) -> Self {
        Self { rom: rom.into() }
    }

    pub fn header(&self) -> Result<Header> {
        Header::load(&self.rom).map_err(|err| Error::HeaderError(err))
    }

    pub fn mbc(&self) -> Result<Box<dyn Mbc>> {
        use MbcType::*;
        let cartridge_type = self.header()?.cartridge_type;
        match cartridge_type.mbc_type() {
            Mbc1 => Ok(Box::new(mbc::Mbc1::new(self.rom.clone()))),
            _ => Err(Error::MbcNotImplemented(cartridge_type)),
        }
    }
}
