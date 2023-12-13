use clap::Parser;
use ubiart_toolkit::utils::string_id;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    string: String,
}

fn main() {
    let cli = Cli::parse();

    println!("{}: 0x{:08x}", cli.string, string_id(&cli.string));
}
