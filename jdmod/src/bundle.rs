//! # Extract
//! Code for extracting a UbiArt archive (ipk or gf)
use std::{path::{PathBuf, Path}, fs::File};

use anyhow::{anyhow, Error};
use clap::Args;
use ubiart_toolkit::utils::GamePlatform;

use crate::{FileConflictStrategy, types::Config};

/// Extract a UbiArt archive
#[derive(Args, Clone)]
pub struct Bundle {
    /// Directory to bundle
    source: PathBuf,
    /// Directory to put the bundled files
    destination: PathBuf,
    /// Config file to use (instead of manually specifying the values)
    #[arg(short, long = "config")]
    config_path: Option<PathBuf>,
    /// The GamePlatform version to use
    #[arg(long)]
    game_platform: Option<u32>,
    /// The engine version to use
    #[arg(long)]
    engine_version: Option<u32>,
    /// The IPK unk4 value to use
    #[arg(long)]
    ipk_unk4: Option<u32>,
}

/// Bundle the files at `source` to `destination`
pub fn main(data: &Bundle) -> Result<(), Error> {
    let config = if let Some(config_path) = &data.config_path {
        let file = File::open(config_path)?;
        let config: Config = serde_json::from_reader(file)?;
        config
    } else {
        let gp = data.game_platform.ok_or_else(|| anyhow!("Missing --game-platform or --config"))?;
        let ev = data.engine_version.ok_or_else(|| anyhow!("Missing --engine-version or --config"))?;
        let iu = data.ipk_unk4.ok_or_else(|| anyhow!("Missing --ipk-unk4 or --config"))?;
        Config {
            game_platform: GamePlatform::try_from(gp)?,
            engine_version: ev,
            ipk_unk4: iu,
        }
    };
    bundle(&data.source, &data.destination, &config)
}

pub fn bundle(source: &Path, destination: &Path, config: &Config) -> Result<(), Error> {
    todo!()
}
