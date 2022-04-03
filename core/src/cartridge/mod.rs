pub mod header;
pub mod mbc;

mod context;

pub use header::*;
pub use mbc::Mbc;

use context::Context;
use std::result;

#[derive(Debug)]
pub struct Cartridge {
    context: Context,
    header: Header,
    mbc: Box<dyn Mbc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HeaderError(header::Error),
    MbcNotImplemented(CartridgeType),
}

pub type Result<T> = result::Result<T, Error>;

pub fn create_mbc(header: &Header) -> Result<Box<dyn Mbc>> {
    use MbcType::*;
    let cartridge_type = &header.cartridge_type;
    match cartridge_type.mbc_type() {
        Mbc1 => Ok(Box::new(mbc::Mbc1::new())),
        _ => Err(Error::MbcNotImplemented(cartridge_type.clone())),
    }
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self> {
        let header = Header::load(&rom).map_err(|err| Error::HeaderError(err))?;
        let context = Context {
            rom,
            ram: vec![0u8; header.ram_size.amount()],
        };
        let mbc = create_mbc(&header)?;
        Ok(Self {
            context,
            header,
            mbc,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mbc.read(&self.context, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mbc.write(&mut self.context, address, value)
    }
}
