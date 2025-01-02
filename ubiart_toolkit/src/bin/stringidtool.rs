use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt;
use ubiart_toolkit::utils::{string_id, InternedString};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Decode from a hex value instead of encoding
    #[arg(short, long)]
    decode: bool,
    /// Can be a string, path to a file, or '-' for stdin
    input: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.decode {
        let without_prefix = cli.input.trim_start_matches("0x");
        let bin = u32::from_str_radix(without_prefix, 16)
            .unwrap()
            .to_be_bytes();
        match InternedString::deserialize(bin.as_slice()) {
            Ok(id) => println!("{id}"),
            Err(_) => println!("Unknown!"),
        }
    } else {
        if Path::new(&cli.input).exists() {
            let file = File::open(&cli.input).unwrap();
            let bufread = BufReader::new(file);
            for line in bufread.lines() {
                let line = line.unwrap();
                println!("0x{:08x}: {}", string_id(&line), line);
            }
        } else if cli.input == "-" {
            let bufread = BufReader::new(stdin());
            for line in bufread.lines() {
                let line = line.unwrap();
                println!("0x{:08x}: {}", string_id(&line), line);
            }
        } else {
            println!("0x{:08x}: {}", string_id(&cli.input), cli.input);
        }
    }
}
