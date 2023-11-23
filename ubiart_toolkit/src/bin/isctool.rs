use std::{fs::File, path::PathBuf};

use clap::Parser;

use ubiart_toolkit::{cooked, utils::bytes::read_to_vec};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.source;

    let data = read_to_vec(&path).unwrap();
    let isc = match cooked::isc::parse(&data) {
        Ok(isc) => isc,
        Err(e) => panic!("{path:?}: {e:?}"),
    };

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &isc).unwrap();
    }
}
