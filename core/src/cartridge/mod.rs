pub mod header;
mod mbc;

pub use header::*;
use mbc::{Mbc, MbcContext};

use std::{cmp::max, rc::Rc};

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

impl From<header::Error> for Error {
    fn from(error: header::Error) -> Self {
        Self::HeaderError(error)
    }
}

pub fn create_mbc(header: &Header) -> Result<Box<dyn Mbc>, Error> {
    use MbcType::*;
    let cartridge_type = &header.cartridge_type;
    Ok(match cartridge_type.mbc_type() {
        RomOnly => Box::new(mbc::RomOnly::default()),
        Mbc1 => Box::new(mbc::Mbc1::default()),
        Mbc2 => Box::new(mbc::Mbc2::default()),
        _ => Err(Error::MbcNotImplemented(cartridge_type.clone()))?,
    })
}

impl Cartridge {
    pub fn new(rom: Rc<Vec<u8>>) -> Result<Self, Error> {
        let header = Header::load(&rom)?;
        let mbc = create_mbc(&header)?;
        let state = State {
            rom,
            ram: vec![0xFFu8; max(header.ram_size.amount(), mbc.internal_ram_size())],
        };
        Ok(Self { header, state, mbc })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.mbc.read_rom(&self.state, address)
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_rom(&mut self.state, address, value)
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.mbc.read_ram(&self.state, address)
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.mbc.write_ram(&mut self.state, address, value)
    }
}
