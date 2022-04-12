mod info;

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
    #[clap(long)]
    dump: Option<String>,
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

fn boot(cartridge: Cartridge, dump: Option<String>) {
    let mut renderer = DummyRenderer::default();
    let mut serial_connection = DummySerialConnection::default();
    let mut game_boy = GameBoy::boot(cartridge);
    for _ in 0..120 {
        for _ in 0..(4194304) {
            game_boy.tick(&mut renderer, &mut serial_connection);
        }
    }
    if let Some(dump_filepath) = dump {
        use std::io::*;
        let mut file = File::create(dump_filepath).unwrap();
        file.write_all(&game_boy.dump()).unwrap();
    }
}

fn main() {
    /*
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
    */
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
        info::print_cartridge_info(&load_header(&rom));
        return;
    }
    if arg.logo {
        println!("{}", load_header(&rom).logo.to_ascii_art());
        return;
    }
    boot(create_cartridge(rom), arg.dump);
}

use std::io::prelude::*;

#[derive(Debug, Default)]
struct DummyRenderer {
    buffer: String,
}

impl core::Renderer for DummyRenderer {
    fn render(&mut self, position: core::Coordinate, color: u8) {
        //return;
        if position.x == 0 {
            if position.y == 0 {
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", self.buffer);
                self.buffer.clear();
            } else {
                self.buffer.push('\n');
            }
        }
        self.buffer.push_str(match color {
            0b00 => "  ",
            0b01 => "__",
            0b10 => "::",
            0b11 => "██",
            _ => "",
        });
    }
}

#[derive(Debug)]
struct DummySerialConnection {
    file: File,
    bits: Vec<bool>,
}

impl Default for DummySerialConnection {
    fn default() -> Self {
        Self {
            file: File::create("./log/serial").unwrap(),
            bits: vec![],
        }
    }
}

impl core::SerialConnection for DummySerialConnection {
    fn send(&mut self, bit: bool) {
        self.bits.push(bit);
        if self.bits.len() >= 8 {
            let byte = self
                .bits
                .iter()
                .fold(0x00, |acc, &bit| acc << 1 | (bit as u8));
            match self.file.write(&[byte]) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Serial output error: {:?}", err);
                }
            }
            self.bits.clear();
        }
    }

    fn receive(&self) -> bool {
        true
    }
}
