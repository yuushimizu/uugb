#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod info;

use clap::Parser;
use simplelog::*;
use std::{fs::File, io::Read, path::Path, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: Option<PathBuf>,
    #[clap(long)]
    info: bool,
    #[clap(long)]
    logo: bool,
    #[clap(long)]
    debug: bool,
}

fn read_rom(filepath: &Path) -> Vec<u8> {
    let mut file = File::open(filepath).unwrap_or_else(|_err| {
        eprintln!("Could not open the file: {}", filepath.display());
        std::process::exit(1);
    });
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap_or_else(|_err| {
        eprintln!("Could not read the file: {}", filepath.display());
        std::process::exit(1);
    });
    rom
}

fn load_header(rom: &[u8]) -> core::Header {
    core::Header::load(rom).unwrap_or_else(|err| {
        eprintln!("Could not load cartridge header: {:?}", err);
        std::process::exit(1);
    })
}

fn main() {
    let args = Args::parse();
    if args.debug {
        CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }
    if args.info {
        if let Some(filepath) = args.file {
            let rom = read_rom(&filepath);
            info::print_cartridge_info(&load_header(&rom));
        }
        return;
    }
    if args.logo {
        if let Some(filepath) = args.file {
            let rom = read_rom(&filepath);
            println!("{}", load_header(&rom).logo.to_ascii_art());
        }
        return;
    }
    gui::start_native(args.file);
}
