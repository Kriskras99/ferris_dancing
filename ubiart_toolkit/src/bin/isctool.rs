use std::{fs::File, path::PathBuf};

use clap::Parser;

use ubiart_toolkit::cooked;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.source;

    let isc = match cooked::isc::open(&path) {
        Ok(isc) => isc,
        Err(e) => panic!("{path:?}: {e:?}"),
    };

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &isc.root()).unwrap();
    }
}
