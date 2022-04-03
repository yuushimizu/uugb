const POSITION: usize = 0x014D;

pub fn load(rom: &[u8]) -> u8 {
    rom[POSITION]
}
