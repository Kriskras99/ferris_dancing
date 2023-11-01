use std::{fs::File, path::PathBuf};

use clap::Parser;
use ubiart_toolkit::msm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let moves = msm::open(&cli.source).unwrap();

    let file = File::create(cli.source.with_extension("json")).unwrap();
    serde_json::to_writer_pretty(file, &moves.msm()).unwrap();
}
