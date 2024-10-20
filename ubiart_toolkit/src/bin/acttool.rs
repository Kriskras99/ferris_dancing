use std::{fs::File, path::PathBuf};

use clap::Parser;
use ubiart_toolkit::{
    cooked::act,
    utils::{Game, Platform, UniqueGameId},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    game: String,
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
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

    let gp = UniqueGameId {
        game,
        platform: Platform::Nx,
        id: 0,
    };

    let data = File::open(&cli.source).unwrap();
    let actors = act::parse(&data, &mut 0, gp).unwrap();

    for component in &actors.components {
        println!("{component:#?}");
    }
}
