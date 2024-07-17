#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use image::ImageFormat;
use ubiart_toolkit::{
    cooked::png::Png,
    utils::{Game, Platform, UniqueGameId},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    info: bool,
    #[arg(short, long, default_value_t = false)]
    xtx_info: bool,
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(&cli.source).unwrap();
    let png = Png::deserialize_with(
        &file,
        UniqueGameId {
            game: Game::JustDance2022,
            platform: Platform::Nx,
            id: 0,
        },
    )
    .unwrap();

    if let Some(savepath) = cli.output {
        let buffer = png.texture;

        let mut filename = PathBuf::from(cli.source.file_name().unwrap());
        filename.set_extension("png");
        let path = savepath.join(filename);
        let mut fout = File::create(path).unwrap();
        buffer.write_to(&mut fout, ImageFormat::Png).unwrap();
    }
}
