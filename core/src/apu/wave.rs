use super::length::Length;

const RAM_SIZE: usize = 16;

const FREQUENCY_UNIT_CYCLES: u64 = 64;

const MAX_FREQUENCY: u16 = 2048;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wave {
    is_started: bool,
    length: Length,
    level: u8,
    pattern: [u8; RAM_SIZE],
}
