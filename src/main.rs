mod cartridge;
mod cpu;
mod memory;
mod util;

use cartridge::Cartridge;
use cpu::Cpu;
use memory::Memory;

use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[clap(long)]
    info: bool,
    #[clap(long)]
    logo: bool,
}

fn print_cartridge_info(header: &cartridge::Header) {
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
    println!("Header Checksum: {:02X}", header.header_checksum);
    println!("Global Checksum: {}", header.global_checksum);
}

fn boot(cartridge: Cartridge) {
    let mut memory = Memory::new(cartridge);
    let mut cpu = Cpu::default();
    cpu.executeNext(&mut memory);
}

fn main() {
    let arg = Args::parse();
    let mut file = File::open(&arg.file).unwrap_or_else(|_err| {
        eprintln!("Could not open the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap_or_else(|_err| {
        eprintln!("Could not read the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let cartridge = Cartridge::new(&rom);
    let header = cartridge.header().unwrap_or_else(|err| {
        eprintln!("Could not load cartridge data from the file: {:?}", err);
        std::process::exit(1);
    });
    if arg.info {
        print_cartridge_info(&header);
        return;
    }
    if arg.logo {
        println!("{}", header.logo.to_ascii_art());
        return;
    }
    boot(cartridge)
}
