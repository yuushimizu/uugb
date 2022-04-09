pub mod header;
mod mbc;

pub use header::*;
use mbc::{Mbc, MbcContext};

use std::{rc::Rc, result};

#[derive(Debug)]
struct State {
    rom: Rc<Vec<u8>>,
    ram: Vec<u8>,
}

impl MbcContext for State {
    fn rom(&self) -> &[u8] {
        &self.rom
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }
}

#[derive(Debug)]
pub struct Cartridge {
    header: Header,
    state: State,
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
        Mbc1 => Ok(Box::new(mbc::Mbc1::default())),
        _ => Err(Error::MbcNotImplemented(cartridge_type.clone())),
    }
}

impl Cartridge {
    pub fn new(rom: Rc<Vec<u8>>) -> Result<Self> {
        let header = Header::load(&rom).map_err(Error::HeaderError)?;
        let state = State {
            rom,
            ram: vec![0x00u8; header.ram_size.amount()],
        };
        let mbc = create_mbc(&header)?;
        Ok(Self { header, state, mbc })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mbc.read(&self.state, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.mbc.write(&mut self.state, address, value)
    }
}
