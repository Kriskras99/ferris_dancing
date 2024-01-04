use std::path::PathBuf;

use byteorder::BigEndian;
use clap::Parser;
use dotstar_toolkit_utils::{bytes::read_to_vec, bytes_new::read::BinaryDeserialize};
use ubiart_toolkit::alias8::Alias8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = read_to_vec(cli.source).unwrap();
    let alias8 = Alias8::deserialize::<BigEndian>(&data.as_slice()).unwrap();

    for alias in alias8.aliases {
        println!(
            "{:04x} {} {} {}",
            alias.unk3, alias.first_alias, alias.second_alias, alias.path
        );
    }
}
