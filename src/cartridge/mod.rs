pub mod header;
pub mod mbc;

pub use header::{Header, MbcType};
pub use mbc::Mbc;

use std::result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cartridge {
    rom: Vec<u8>,
    pub header: Header,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    HeaderError(header::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl Cartridge {
    pub fn load(rom: &[u8]) -> Result<Self> {
        Ok(Self {
            rom: rom.into(),
            header: Header::load(rom).map_err(|err| Error::HeaderError(err))?,
        })
    }

    pub fn mbc(self: &Self) -> Option<Box<dyn Mbc>> {
        use MbcType::*;
        match self.header.cartridge_type.mbc_type() {
            Mbc1 => Some(Box::new(mbc::Mbc1::new(self.rom.clone()))),
            _ => None,
        }
    }
}
