mod mbc1;

pub use mbc1::Mbc1;

use super::Context;
use std::fmt;

pub trait Mbc
where
    Self: fmt::Debug,
{
    fn read(&self, context: &Context, address: u16) -> u8;

    fn write(&mut self, context: &mut Context, address: u16, value: u8);
}
