use std::path::PathBuf;

use clap::Parser;

use ubiart_toolkit::{cooked, utils::Game};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    game: String,
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let game = match cli.game.to_lowercase().as_str() {
        "2022" => Game::JustDance2022,
        "2021" => Game::JustDance2021,
        "2020" => Game::JustDance2020,
        "china" => Game::JustDanceChina,
        "2019" => Game::JustDance2019,
        "2018" => Game::JustDance2018,
        "2017" => Game::JustDance2017,
        _ => panic!("Unrecognized game version: {}", cli.game),
    };

    let path = cli.source;

    let _template = match cooked::json::open(&path, game) {
        Ok(template) => template,
        Err(e) => panic!("{path:?}: {e:?}"),
    };
}
