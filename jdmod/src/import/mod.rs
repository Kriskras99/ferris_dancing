//! # Import
//! The main code for importing games and songs
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Error};
use clap::Args;
use dotstar_toolkit_utils::testing::test;
use dotstar_toolkit_utils::vfs::{native::Native, VirtualFileSystem};
use ubiart_toolkit::{
    alias8, cooked,
    secure_fat::vfs::VfsSfatFilesystem,
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
}

/// Wrapper around [`import`]
pub fn cli_import(cli: &Import) -> Result<(), Error> {
    import(&cli.game_path, &cli.mod_path)
}

/// Import a game at `game_path` into the mod at `dir_root`
pub fn import(game_path: &Path, dir_root: &Path) -> Result<(), Error> {
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
        let sfat_vfs = VfsSfatFilesystem::new(&native_vfs, &PathBuf::from("secure_fat.gf"))?;

        // TODO: Check engine version and warn user they're missing an update

        // Import songs and other content from the game
        import_sfat(&sfat_vfs, dir_root)?;
    } else if game_path.is_dir() {
        let native_vfs = Native::new(game_path)?;
        import_raw(&native_vfs, dir_root, Game::JustDance2022, Platform::Nx)?;
    } else {
        return Err(anyhow!("Cannot import {game_path:?}! Input not recognized, currently only secure_fat.gf and raw import are supported!"));
    }

    Ok(())
}

/// Import a secure_fat.gf into the mod at `dir_root`
pub fn import_sfat(sfat: &VfsSfatFilesystem<'_>, dir_root: &Path) -> Result<(), Error> {
    // Make sure the directory tree is intact
    let dirs = DirectoryTree::new(dir_root);
    test(&dirs.exists(), &true)?;

    // Make game and platform easily accessible
    let platform = sfat.game_platform().platform;
    let game = sfat.game_platform().game;

    // Load localisations
    let locale_id_map = localisation::import(sfat, &dirs)?;

    // Load alias8, which contains the locations of important files
    let alias8_file = sfat.open(String::from("enginedata/common.alias8").as_ref())?;
    let aliases = alias8::parse(&alias8_file)?;

    // Collect common required items in a convenient place
    let state = ImportState {
        vfs: sfat,
        dirs,
        game,
        platform,
        locale_id_map,
        aliases,
    };

    // Import gameconfig (& songs)
    gameconfig::import(&state)?;

    Ok(())
}

/// Import a unpacked game into the mod at `dir_root`
pub fn import_raw(
    unpacked_game: &Native,
    dir_root: &Path,
    game: Game,
    platform: Platform,
) -> Result<(), Error> {
    // Make sure the directory tree is intact
    let dirs = DirectoryTree::new(dir_root);
    test(&dirs.exists(), &true)?;

    // Load localisations
    let locale_id_map = localisation::import(unpacked_game, &dirs)?;

    // Load alias8, which contains the locations of important files
    let alias8_file = unpacked_game.open(String::from("enginedata/common.alias8").as_ref())?;
    let aliases = alias8::parse(&alias8_file)?;

    // Collect common required items in a convenient place
    let is = ImportState {
        vfs: unpacked_game,
        dirs,
        game,
        platform,
        locale_id_map,
        aliases,
    };

    // Get the gameconfig path
    let gameconfig_path = cook_path(
        &is.aliases
            .get_path_for_alias("gameconfig")
            .ok_or_else(|| anyhow!("common.alias8 does not contain gameconfig path!"))?,
        is.platform,
    )?;
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v22_lax(&gameconfig_file)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Import only songs
    gameconfig::songdb::import(&is, &gameconfig.songdb_scene)?;

    Ok(())
}
