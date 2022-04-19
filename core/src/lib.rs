pub mod cartridge;
pub mod cpu;
pub mod game_boy;
pub mod interrupt;
pub mod joypad;
pub mod memory;
pub mod ppu;
pub mod serial;
pub mod timer;

mod util;

pub use cartridge::Cartridge;
pub use game_boy::GameBoy;
pub use ppu::{display_size, Color, Renderer, Vec2};
pub use serial::SerialConnection;

pub const CLOCK_CYCLE: u64 = 4194304;

pub const M_CYCLES: u64 = CLOCK_CYCLE / 4;

pub const M_CYCLES_PER_FRAME: u64 = 17556;

pub const FRAME_RATE: f64 = M_CYCLES as f64 / M_CYCLES_PER_FRAME as f64;
