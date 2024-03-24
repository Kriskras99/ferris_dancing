use std::{fs::File, path::PathBuf, rc::Rc};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::loc8::Loc8;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    let file = Rc::new(File::open(cli.source).unwrap());
    let loc8 = Loc8::deserialize(&file).unwrap();

    if cli.verbose {
        for (locale_id, string) in &loc8.strings {
            println!("{locale_id}: {string}");
        }
    }

    println!("Strings: {}", loc8.strings.len());
    println!("Language: {:?}", loc8.language);
}
