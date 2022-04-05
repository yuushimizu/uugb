use crate::util::bits::Bits;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Logo {
    bytes: Vec<u8>,
}

const RANGE: RangeInclusive<usize> = 0x0104..=0x0133;

impl Logo {
    pub fn load(rom: &[u8]) -> Self {
        Logo {
            bytes: rom[RANGE].into(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn decode(&self) -> Vec<Vec<bool>> {
        const COLUMNS: usize = 12;
        const ROWS: usize = 2;
        const BLOCK_SIZE: usize = COLUMNS * ROWS;
        const BLOCKS: usize = 2;
        (0..BLOCKS)
            .flat_map(|block| {
                (0..ROWS).flat_map(move |row| {
                    [4, 0].map(|bit_offset| {
                        (0..COLUMNS)
                            .flat_map(|column| {
                                (0..4).rev().map(move |bit| {
                                    self.bytes[block * BLOCK_SIZE + row + column * ROWS]
                                        .bit(bit_offset + bit)
                                })
                            })
                            .collect()
                    })
                })
            })
            .collect()
    }

    pub fn to_ascii_art(&self) -> String {
        self.decode()
            .iter()
            .map(|line| line.iter().map(|&on| if on { '█' } else { ' ' }).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
