#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::path::PathBuf;

use clap::Parser;
use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::cooked;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.source;
    let data = read_to_vec(path).unwrap();
    let _sgs = cooked::sgs::parse(&data).unwrap();
}
