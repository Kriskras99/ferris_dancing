#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read_to_vec;
use ubiart_toolkit::{cooked, utils::Game};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    game: String,
    source: PathBuf,
    destination: Option<PathBuf>,
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
    let data = read_to_vec(path).unwrap();

    match game {
        Game::JustDance2017 => {
            cooked::json::parse_v17(&data, false).unwrap();
        }
        Game::JustDance2018 => {
            cooked::json::parse_v18(&data, false).unwrap();
        }
        Game::JustDance2019 => {
            cooked::json::parse_v19(&data, false).unwrap();
        }
        Game::JustDance2020 => {
            cooked::json::parse_v20(&data, false).unwrap();
        }
        Game::JustDanceChina => {
            cooked::json::parse_v20c(&data, false).unwrap();
        }
        Game::JustDance2021 => {
            cooked::json::parse_v21(&data, false).unwrap();
        }
        Game::JustDance2022 => {
            let json = cooked::json::parse_v22(&data, false).unwrap();
            if let Some(path) = cli.destination {
                let file = File::create(path).unwrap();
                cooked::json::create(file, &json).unwrap();
            }
        }
        _ => panic!("Unsupported game version: {game}"),
    }
}
