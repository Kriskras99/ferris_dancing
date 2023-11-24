use std::path::PathBuf;

use clap::Parser;

use ubiart_toolkit::{alias8, utils::bytes::read_to_vec};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = read_to_vec(cli.source).unwrap();
    let alias8 = alias8::parse(&data).unwrap();

    for alias in alias8.aliases {
        println!(
            "{:04x} {} {} {}",
            alias.unk3, alias.first_alias, alias.second_alias, alias.path
        );
    }
}
