use core::{
    cartridge::{self, Cartridge},
    GameBoy,
};

use clap::Parser;
use simplelog::*;
use std::{fs::File, io::Read, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[clap(long)]
    info: bool,
    #[clap(long)]
    logo: bool,
}

fn load_header(rom: &[u8]) -> cartridge::Header {
    cartridge::Header::load(rom).unwrap_or_else(|err| {
        eprintln!("Could not load cartridge header: {:?}", err);
        std::process::exit(1);
    })
}

fn create_cartridge(rom: Vec<u8>) -> Cartridge {
    Cartridge::new(rom.into()).unwrap_or_else(|err| {
        eprintln!("Could not load cartridge: {:?}", err);
        std::process::exit(1);
    })
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

fn boot(cartridge: Cartridge) {
    let mut game_boy = GameBoy::boot(cartridge);
    let mut buffer = String::new();
    for _ in 0..600 {
        for _ in 0..(4194304) {
            game_boy.tick();
            _ = std::io::stdin().read_line(&mut buffer);
        }
    }
    use std::io::*;
    let mut file = File::create("./log/dump").unwrap();
    file.write_all(&game_boy.dump()).unwrap();
}

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
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
    if arg.info {
        print_cartridge_info(&load_header(&rom));
        return;
    }
    if arg.logo {
        println!("{}", load_header(&rom).logo.to_ascii_art());
        return;
    }
    boot(create_cartridge(rom))
}
