use std::path::PathBuf;

use clap::Parser;
use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::loc8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let source = cli.source;
    let data = read_to_vec(source).unwrap();
    let loc8 = loc8::parse(&data).unwrap();

    if cli.verbose {
        for (locale_id, string) in &loc8.strings {
            println!("{locale_id}: {string}");
        }
    }

    println!("Strings: {}", loc8.strings.len());
    println!("Language: {:?}", loc8.language);
}
