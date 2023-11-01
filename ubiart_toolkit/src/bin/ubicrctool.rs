use clap::Parser;

use ubiart_toolkit::utils::ubi_crc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    string: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{}: 0x{:08x}", cli.string, ubi_crc(cli.string.as_bytes()))
}
