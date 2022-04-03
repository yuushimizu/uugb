const POSITION: usize = 0x014C;

pub fn load(rom_bytes: &[u8]) -> u8 {
    rom_bytes[POSITION]
}
