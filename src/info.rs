use core::cartridge;

pub fn print_cartridge_info(header: &cartridge::Header) {
    println!("Title: {}", header.title);
    println!("Entry Point: {}", header.entry_point);
    println!("CGB Flag: {}", header.cgb_flag);
    println!("Licensee: {}", header.licensee);
    println!("SGB Flag: {}", header.sgb_flag);
    println!("Cartridge Type: {}", header.cartridge_type);
    println!("ROM Size: {}", header.rom_size);
    println!("RAM Size: {}", header.ram_size);
    println!("Destination: {}", header.destination);
    println!("Version: {:02X}", header.version);
    println!(
        "Header Checksum: {} = {:02X} {}",
        header.header_checksum,
        header.header_checksum.calculated_value(),
        if header.header_checksum.is_matched() {
            "OK"
        } else {
            "NG"
        }
    );
    println!(
        "Global Checksum: {} = {:04X} {}",
        header.global_checksum,
        header.global_checksum.calculated_value(),
        if header.global_checksum.is_matched() {
            "OK"
        } else {
            "NG"
        }
    );
}
