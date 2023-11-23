use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use ubiart_toolkit::{cooked, utils::bytes::read_to_vec};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.source;
    let data = read_to_vec(&path).unwrap();
    let _sgs = cooked::sgs::parse(&data)
        .with_context(|| format!("{path:?}"))
        .unwrap();
}
