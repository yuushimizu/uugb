mod cartridge;
mod util;

use cartridge::Cartridge;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
    #[clap(long)]
    logo: bool,
}

fn main() {
    let arg = Args::parse();
    let mut file = File::open(&arg.file).unwrap_or_else(|_err| {
        eprintln!("Could not open the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap_or_else(|_err| {
        eprintln!("Could not read the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let cartridge = Cartridge::load(&bytes).unwrap_or_else(|err| {
        eprintln!("Could not load cartridge data from the file: {:?}", err);
        std::process::exit(1);
    });
    if arg.logo {
        println!("{}", cartridge.header.logo.to_ascii_art());
    } else {
        println!("{:#?}", cartridge);
    }
}
