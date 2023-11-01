use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;
use ubiart_toolkit::loc8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let source = cli.source;
    let loc8 = loc8::open(source)?;

    if cli.verbose {
        for string in loc8.strings() {
            println!("{}: {}", string.0, string.1);
        }
    }

    println!("Strings: {}", loc8.strings().len());
    println!("Language: {:?}", loc8.language());
    Ok(())
}
