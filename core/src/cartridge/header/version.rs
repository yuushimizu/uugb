const ADDRESS: usize = 0x014C;

pub fn load(rom: &[u8]) -> u8 {
    rom[ADDRESS]
}
