use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::alias8::Alias8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.source).unwrap();
    let alias8 = Alias8::deserialize(&file).unwrap();

    for alias in alias8.aliases() {
        println!("{:04b} {} {}", alias.unk3 >> 12, alias.alias, alias.path);
    }
}
