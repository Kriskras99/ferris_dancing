#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::{cooked, utils::UniqueGameId};

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
    let tpl = match cooked::tpl::parse(&data, UniqueGameId::WIIU2015, false) {
        Ok(tpl) => tpl,
        Err(e) => panic!("{path:?}: {e:?}"),
    };

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &tpl).unwrap();
    }
}
