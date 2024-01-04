//! # Import
//! The main code for importing games and songs
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};
use clap::Args;
use dotstar_toolkit_utils::bytes::BigEndian;
use dotstar_toolkit_utils::bytes_new::read::BinaryDeserialize;
use dotstar_toolkit_utils::testing::test;
use dotstar_toolkit_utils::vfs::{native::Native, VirtualFileSystem};
use ubiart_toolkit::alias8::Alias8;
use ubiart_toolkit::{
    cooked,
    secure_fat::vfs::SfatFilesystem,
    utils::{Game, Platform},
};

use crate::{
    types::{DirectoryTree, ImportState},
    utils::cook_path,
};

mod gameconfig;
mod localisation;
mod song;

/// Import <game_path> into mod at <mod_path>
#[derive(Args, Clone)]
pub struct Import {
    /// Path of the game to import (either secure_fat.gf or raw extracted game)
    game_path: PathBuf,
    /// Mod directory
    mod_path: PathBuf,
    /// Ignore mistakes in the file format (useful for modded files)
    #[arg(long, default_value_t = false)]
    lax: bool,
    /// Only import the songs
    #[arg(long, default_value_t = false)]
    songs: bool,
    /// Overwrite game
    #[arg(long)]
    game: Option<Game>,
    /// Overwrite platform
    #[arg(long)]
    platform: Option<Platform>,
}

/// Wrapper around [`import`]
pub fn main(cli: &Import) -> Result<(), Error> {
    import(
        &cli.game_path,
        &cli.mod_path,
        cli.lax,
        cli.songs,
        cli.game,
        cli.platform,
    )
}

/// Import a game at `game_path` into the mod at `dir_root`
pub fn import(
    game_path: &Path,
    dir_root: &Path,
    lax: bool,
    songs_only: bool,
    game: Option<Game>,
    platform: Option<Platform>,
) -> Result<(), Error> {
    // Check the directory structure
    let dir_tree = DirectoryTree::new(dir_root);
    if !dir_tree.exists() {
        return Err(anyhow!(
            "Mod directory does not exist or is missing vital subdirectories!"
        ));
    }

    if game_path.ends_with("secure_fat.gf") {
        // Init the native filesystem and load the securefat as a virtual filesystem
        let native_vfs = Native::new(
            game_path
                .parent()
                .ok_or_else(|| anyhow!("No parent directory for secure_fat.gf!"))?,
        )?;
        let sfat_vfs = SfatFilesystem::new(&native_vfs, &PathBuf::from("secure_fat.gf"), lax)?;

        // TODO: Check engine version and warn user they're missing an update

        // Make game and platform easily accessible
        let platform = platform.unwrap_or_else(|| sfat_vfs.game_platform().platform);
        let game = game.unwrap_or_else(|| sfat_vfs.game_platform().game);

        // Import songs and other content from the game
        import_vfs(&sfat_vfs, dir_root, game, platform, lax, songs_only)?;
    } else if game_path.is_dir() {
        let native_vfs = Native::new(game_path)?;
        import_vfs(
            &native_vfs,
            dir_root,
            game.unwrap_or(Game::JustDance2022),
            platform.unwrap_or(Platform::Nx),
            lax,
            songs_only,
        )?;
    } else {
        return Err(anyhow!("Cannot import {game_path:?}! Input not recognized, currently only secure_fat.gf and raw import are supported!"));
    }

    Ok(())
}

/// Import a game represented as a virtual filesystem
pub fn import_vfs(
    vfs: &dyn VirtualFileSystem,
    dir_root: &Path,
    game: Game,
    platform: Platform,
    lax: bool,
    songs_only: bool,
) -> Result<(), Error> {
    // Make sure the directory tree is intact
    let dirs = DirectoryTree::new(dir_root);
    test(&dirs.exists(), &true)?;

    // Load localisations
    let locale_id_map = localisation::import(vfs, &dirs)?;

    // Load alias8, which contains the locations of important files
    let alias8_file = vfs.open(String::from("enginedata/common.alias8").as_ref())?;
    let aliases = Alias8::deserialize::<BigEndian>(&alias8_file.as_ref())?;

    // Collect common required items in a convenient place
    let is = ImportState {
        vfs,
        dirs,
        game,
        platform,
        locale_id_map,
        aliases,
        lax,
    };

    if songs_only {
        // Get the gameconfig path
        let gameconfig_path = cook_path(
            &is.aliases
                .get_path_for_alias("gameconfig")
                .ok_or_else(|| anyhow!("common.alias8 does not contain gameconfig path!"))?,
            is.platform,
        )?;
        let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;

        let songdb_scene = match game {
            Game::JustDance2017 => {
                let parsed_json =
                    cooked::json::parse_v17(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2018 => {
                let parsed_json =
                    cooked::json::parse_v18(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2019 => {
                let parsed_json =
                    cooked::json::parse_v19(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2020 => {
                let parsed_json =
                    cooked::json::parse_v20(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDanceChina => {
                let parsed_json =
                    cooked::json::parse_v20c(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2021 => {
                let parsed_json =
                    cooked::json::parse_v21(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2022 => {
                let parsed_json =
                    cooked::json::parse_v22(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            _ => {
                println!("Unknown game, trying JustDance2022");
                let parsed_json =
                    cooked::json::parse_v22(&gameconfig_file, true)?.game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
        };

        // Import only songs
        gameconfig::songdb::import(&is, &songdb_scene)?;
    } else {
        // Import gameconfig (& songs)
        gameconfig::import(&is)?;
    };
    Ok(())
}
