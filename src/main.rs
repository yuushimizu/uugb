use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod rom;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() {
    let arg = Args::parse();
    let mut file = File::open(&arg.file).unwrap_or_else(|_err| {
        eprintln!("Could not open the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap_or_else(|_err| {
        eprintln!("Could not read from the file: {}", arg.file.display());
        std::process::exit(1);
    });
    let rom = rom::Rom::load(&bytes);
    println!("{:?}", rom);
}
