//! # Import
//! The main code for importing games and songs
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use clap::Args;
use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserializeExt as _,
    vfs::{native::NativeFs, VirtualFileSystem, VirtualPath, VirtualPathBuf},
};
use ubiart_toolkit::{
    alias8::Alias8,
    cooked,
    secure_fat::vfs::SfatFilesystem,
    utils::{Game, Platform, UniqueGameId},
};

use crate::{
    types::{DirectoryTree, ImportState},
    utils::cook_path,
};

mod gameconfig;
#[cfg(feature = "experimental")]
mod jdnow;
mod localisation;
mod song;

/// Import <game_path> into mod at <mod_path>
#[derive(Args, Clone)]
pub struct Import {
    /// Path of the game to import (secure_fat.gf or a JD Now json file)
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
#[tracing::instrument]
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
        let native_vfs = NativeFs::new(
            game_path
                .parent()
                .ok_or_else(|| anyhow!("No parent directory for secure_fat.gf!"))?,
        )?;
        let sfat_vfs = SfatFilesystem::new(&native_vfs, &VirtualPathBuf::from("secure_fat.gf"))?;

        // TODO: Check engine version and warn user they're missing an update

        let unique_game_id = sfat_vfs.unique_game_id();

        // Import songs and other content from the game
        import_vfs(&sfat_vfs, dir_tree, unique_game_id, lax, songs_only)?;
    } else if game_path.is_dir() {
        let native_vfs = NativeFs::new(game_path)?;

        let game = if let Some(game) = game {
            game
        } else {
            println!("No game specified, assuming {}", Game::JustDance2022);
            Game::JustDance2022
        };

        let platform = if let Some(platform) = platform {
            platform
        } else {
            println!("No platform specified, assuming {}", Platform::Nx);
            Platform::Nx
        };

        let unique_game_id = UniqueGameId {
            game,
            platform,
            id: 0,
        };

        import_vfs(&native_vfs, dir_tree, unique_game_id, lax, songs_only)?;
    } else if game_path.extension() == Some(OsStr::new("json")) {
        #[cfg(feature = "experimental")]
        {
            let parent = game_path
                .parent()
                .ok_or_else(|| anyhow!("File has no parent directory!"))?;
            let filename = game_path
                .file_name()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("Filename is invalid!"))?;
            let native_vfs = NativeFs::new(parent)?;
            let path = VirtualPath::new(filename);
            jdnow::import(&native_vfs, path, &dir_tree)?;
        }
        #[cfg(not(feature = "experimental"))]
        {
            panic!("This feature is still in development!");
        }
    } else {
        return Err(anyhow!("Cannot import {game_path:?}! Input not recognized, currently only secure_fat.gf, JD Now .json files, and raw import are supported!"));
    }

    Ok(())
}

/// Import a game represented as a virtual filesystem
pub fn import_vfs(
    vfs: &dyn VirtualFileSystem,
    dirs: DirectoryTree,
    ugi: UniqueGameId,
    lax: bool,
    songs_only: bool,
) -> Result<(), Error> {
    if ugi.id == 0 {
        println!("Importing {} for {}", ugi.game, ugi.platform);
    } else {
        println!(
            "Importing {} for {} (UGI: {:x})",
            ugi.game, ugi.platform, ugi.id
        );
    }

    // Load localisations
    let locale_id_map = localisation::import(vfs, &dirs)?;

    // Load alias8, which contains the locations of important files
    let alias8_file = vfs.open(VirtualPath::new("enginedata/common.alias8"))?;
    let aliases = Alias8::deserialize(&alias8_file)?;

    // Collect common required items in a convenient place
    let is = ImportState {
        vfs,
        dirs,
        ugi,
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
            is.ugi.platform,
        )?;
        let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;

        let songdb_scene = match ugi.game {
            Game::JustDance2017 => {
                let parsed_json =
                    cooked::json::parse_v17(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2018 => {
                let parsed_json =
                    cooked::json::parse_v18(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2019 => {
                let parsed_json =
                    cooked::json::parse_v19(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2020 => {
                let parsed_json =
                    cooked::json::parse_v20(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDanceChina => {
                let parsed_json =
                    cooked::json::parse_v20c(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2021 => {
                let parsed_json =
                    cooked::json::parse_v21(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            Game::JustDance2022 => {
                let parsed_json =
                    cooked::json::parse_v22(&gameconfig_file, true)?.into_game_manager_config()?;
                parsed_json.songdb_scene.into_owned()
            }
            _ => {
                println!("Unknown game, trying JustDance2022");
                let parsed_json =
                    cooked::json::parse_v22(&gameconfig_file, true)?.into_game_manager_config()?;
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
