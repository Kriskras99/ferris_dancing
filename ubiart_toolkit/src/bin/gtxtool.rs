use std::{fs::File, path::PathBuf};

use clap::Parser;
use ubiart_toolkit::{
    cooked,
    utils::{Game, Platform, UniqueGameId},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(&cli.source).unwrap();
    let _ = cooked::png::parse(
        &file,
        UniqueGameId {
            game: Game::JustDance2016,
            platform: Platform::WiiU,
            id: 0,
        },
    )
    .unwrap();
}
