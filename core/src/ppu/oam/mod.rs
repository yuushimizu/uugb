pub mod object;

pub use object::Object;

use super::Vec2;
use crate::memory::Dma;

const OBJECT_COUNT: usize = 40;

const TOTAL_SIZE: usize = object::DATA_SIZE * OBJECT_COUNT;

const DMA_DESTINATION: u16 = 0xFE00;

const OBJECTS_PER_LINE: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Oam {
    dma_source_address_upper: u8,
    data: Vec<u8>,
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            dma_source_address_upper: 0x00,
            data: vec![0x00u8; TOTAL_SIZE],
        }
    }
}

impl Oam {
    pub fn dma_source_address_upper(&self) -> u8 {
        self.dma_source_address_upper
    }

    pub fn request_dma(&mut self, source_address_upper: u8, dma: &mut Dma) {
        self.dma_source_address_upper = source_address_upper;
        dma.request(
            (source_address_upper as u16) << 8,
            DMA_DESTINATION,
            TOTAL_SIZE as u16,
        )
    }

    pub fn read(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("OAM: Attempt to read from out of bounds: {:04X}", address);
            &0xFF
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("OAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }

    fn objects(&self) -> impl std::iter::Iterator<Item = Object> + '_ {
        (0..OBJECT_COUNT).map(|i| {
            let start = i * object::DATA_SIZE;
            Object::from(&self.data[start..start + object::DATA_SIZE])
        })
    }

    fn objects_in_line(
        &self,
        y: u8,
        large_object: bool,
    ) -> impl std::iter::Iterator<Item = Object> + '_ {
        self.objects()
            .filter(move |object| object.contains_y(y, large_object))
            .take(OBJECTS_PER_LINE)
    }

    pub fn objects_at_position(
        &self,
        position: Vec2,
        large_object: bool,
    ) -> impl std::iter::Iterator<Item = Object> + '_ {
        self.objects_in_line(position.y, large_object)
            .filter(move |object| object.contains_x(position.x))
    }
}
