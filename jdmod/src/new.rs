//! # New
//! Contains the logic for creating a new mod
use std::{
    fs::File,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use clap::Args;
use dotstar_toolkit_utils::vfs::native::NativeFs;
use ubiart_toolkit::{
    secure_fat::vfs::SfatFilesystem,
    utils::{Game, Platform},
};

use crate::{
    import,
    types::{Config, DirectoryTree},
};

/// Create a new mod at <mod_path> using <sfat_path> as a base
#[derive(Args, Clone)]
pub struct New {
    /// secure_fat.gf of the game to mod
    sfat_path: PathBuf,
    /// Directory to place the mod
    mod_path: PathBuf,
}

/// Wrapper around [`new`]
pub fn main(cli: &New) -> Result<(), Error> {
    new(&cli.sfat_path, &cli.mod_path)
}

/// Create a new game at `dir_root` with the secure_fat.gf at `game_path`
///
/// # Errors
/// - When `dir_root` exists and is not empty or not a directory
/// - When `game_path` is not a secure_fat.gf
/// - Invalid secure_fat.gf or .ipks or missing .ipks
/// - The secure_fat.gf is not for the Nintendo Switch
pub fn new(game_path: &Path, dir_root: &Path) -> Result<(), Error> {
    // Check that the target directory either doesn't exist yet or is empty
    // We do not want to potentially override existing files and/or directories
    if dir_root.exists() && dir_root.read_dir()?.next().is_some() {
        return Err(anyhow!("{dir_root:?} exists and is not empty!"));
    }

    // TODO: import directly from XCI/NSP?
    if !game_path.ends_with("secure_fat.gf") {
        return Err(anyhow!(
            "Expected path to 'secure_fat.gf', got {game_path:?} instead!"
        ));
    }

    // Init the native filesystem and load the securefat as a virtual filesystem
    let native_vfs = NativeFs::new(
        game_path
            .parent()
            .ok_or_else(|| anyhow!("No parent directory for secure_fat.gf!"))?,
    )?;
    let sfat_vfs = SfatFilesystem::new(&native_vfs, &PathBuf::from("secure_fat.gf"))?;

    // Check that the sfat is from the right game
    let game_platform = sfat_vfs.game_platform();
    if game_platform.game != Game::JustDance2022 {
        return Err(anyhow!(
            "The secure_fat.gf is from {} instead of {}!",
            game_platform.game,
            Game::JustDance2022
        ));
    }
    if game_platform.platform != Platform::Nx {
        return Err(anyhow!(
            "The secure_fat.gf is for {} instead of {}!",
            game_platform.platform,
            Platform::Nx
        ));
    }
    // TODO: Check engine version and warn user they're missing an update

    // Create the directory structure
    let dir_tree = DirectoryTree::new(dir_root);
    dir_tree.create_all()?;

    // Write the config file
    let path_root_mod_config = dir_tree.dot_mod().join("config.json");
    let file_root_mod_config = File::create(path_root_mod_config)?;
    serde_json::to_writer_pretty(
        file_root_mod_config,
        &Config {
            game_platform,
            engine_version: sfat_vfs.engine_version(),
            ipk_unk4: sfat_vfs.ipk_unk4(),
        },
    )?;

    // Copy bundle_nx and patch_nx to .mod/base
    std::fs::copy(
        game_path.with_file_name("bundle_nx.ipk"),
        dir_tree.base().join("bundle_nx.ipk"),
    )?;
    if let Err(e) = std::fs::copy(
        game_path.with_file_name("patch_nx.ipk"),
        dir_tree.base().join("patch_nx.ipk"),
    ) {
        if e.kind() == ErrorKind::NotFound {
            println!("Warning! You're missing patch_nx.ipk. This means you're missing an update!");
        } else {
            return Err(e.into());
        }
    }

    // Make game and platform easily accessible
    let platform = sfat_vfs.game_platform().platform;
    let game = sfat_vfs.game_platform().game;

    // Import songs and other content from the game
    import::import_vfs(&sfat_vfs, dir_root, game, platform, false, false)?;

    Ok(())
}
