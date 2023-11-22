use std::path::PathBuf;

use anyhow::Context;
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

    match game {
        Game::JustDance2017 => {
            cooked::json::open_v17(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDance2018 => {
            cooked::json::open_v18(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDance2019 => {
            cooked::json::open_v19(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDance2020 => {
            cooked::json::open_v20(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDanceChina => {
            cooked::json::open_v20c(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDance2021 => {
            cooked::json::open_v21(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        Game::JustDance2022 => {
            cooked::json::open_v22(&path, false)
                .with_context(|| format!("{path:?}"))
                .unwrap();
        }
        _ => panic!("Unsupported game version: {game}"),
    }
}
