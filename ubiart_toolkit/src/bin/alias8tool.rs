use std::path::PathBuf;

use clap::Parser;

use ubiart_toolkit::alias8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let alias8 = alias8::open(cli.source).unwrap();

    for alias in alias8.aliases() {
        println!(
            "{:04x} {} {} {} {}",
            alias.unk3, alias.first_alias, alias.second_alias, alias.filename, alias.path
        );
    }
}
