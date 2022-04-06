pub mod header;
pub mod mbc;

pub use header::*;
pub use mbc::Mbc;

use mbc::MbcContext;
use std::rc::Rc;
use std::result;

#[derive(Debug)]
pub struct Cartridge {
    header: Header,
    mbc: Box<dyn Mbc>,
    mbc_context: MbcContext,
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
    pub fn new(rom: Rc<Vec<u8>>) -> Result<Self> {
        let header = Header::load(&rom).map_err(|err| Error::HeaderError(err))?;
        let mbc = create_mbc(&header)?;
        let mbc_context = MbcContext {
            rom,
            ram: vec![0x00u8; header.ram_size.amount()],
        };
        Ok(Self {
            header,
            mbc,
            mbc_context,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mbc.read(&self.mbc_context, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mbc.write(&mut self.mbc_context, address, value)
    }
}
