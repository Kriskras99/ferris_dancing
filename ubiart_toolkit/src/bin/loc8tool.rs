use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;
use ubiart_toolkit::{loc8, utils::bytes::read_to_vec};

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
    let data = read_to_vec(source)?;
    let loc8 = loc8::parse(&data)?;

    if cli.verbose {
        for (locale_id, string) in &loc8.strings {
            println!("{locale_id}: {string}");
        }
    }

    println!("Strings: {}", loc8.strings.len());
    println!("Language: {:?}", loc8.language);
    Ok(())
}
