use super::Coordinate;
use std::ops::RangeInclusive;

const TILE_DATA_SIZE: usize = 16;

const TILE_MAP_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileData<'vram> {
    data: &'vram [u8],
}

impl<'vram> TileData<'vram> {
    fn new(data: &'vram [u8]) -> Self {
        Self { data }
    }

    pub fn pixel(&self, position: Coordinate) -> u8 {
        let index = position.y as usize * 2 % 8;
        [1, 0]
            .iter()
            .map(|offset| self.data[index + offset])
            .map(|byte| byte >> (7 - position.x as usize % 8))
            .fold(0b00, |acc, bit| acc << 1 | bit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileDataArea {
    Shifted,
    Origin,
}

impl TileDataArea {
    fn address(&self, id: u8) -> usize {
        use TileDataArea::*;
        match self {
            Shifted => 0x0800 + id.wrapping_sub(128) as usize,
            Origin => id as usize * TILE_DATA_SIZE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileMapArea {
    First,
    Second,
}

impl TileMapArea {
    fn range(&self) -> RangeInclusive<usize> {
        use TileMapArea::*;
        match self {
            First => 0x1800..=0x1BFF,
            Second => 0x1C00..=0x1FFF,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct VramData {
    data: Vec<u8>,
}

impl Default for VramData {
    fn default() -> Self {
        Self {
            data: vec![0x00u8; 0x2000],
        }
    }
}

impl VramData {
    fn tile_map_data(&self, area: TileMapArea) -> &[u8] {
        &self.data[area.range()]
    }

    fn tile_data(&self, area: TileDataArea, id: u8) -> TileData {
        let address = area.address(id);
        TileData::new(&self.data[address..(address + TILE_DATA_SIZE)])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileMap<'vram> {
    data: &'vram VramData,
    map_area: TileMapArea,
    data_area: TileDataArea,
}

impl<'vram> TileMap<'vram> {
    pub fn tile_data(&self, position: Coordinate) -> TileData {
        let id = self.data.tile_map_data(self.map_area)
            [position.y as usize * TILE_MAP_SIZE + position.x as usize];
        self.data.tile_data(self.data_area, id)
    }

    pub fn pixel(&self, position: Coordinate) -> u8 {
        self.tile_data(Coordinate {
            x: position.x / 8,
            y: position.y / 8,
        })
        .pixel(position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Vram {
    data: VramData,
}

impl Vram {
    pub fn tile_map(&self, map_area: TileMapArea, data_area: TileDataArea) -> TileMap {
        TileMap {
            data: &self.data,
            map_area,
            data_area,
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        *self.data.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("VRAM: Attempt to read from out of bounds: {:04X}", address);
            &0x00
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("VRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }
}
