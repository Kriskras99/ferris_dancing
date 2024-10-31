//! Functionality for unlocking songs, avatars, and anything else

use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{bail, Error};
use clap::Args;
use hipstr::HipStr;

use crate::types::{
    gameconfig::{
        aliases::Aliases,
        avatars::{Avatar, UnlockType},
        portraitborders::{LockStatus, PortraitBorder},
    },
    song::{MapStatus, Song},
    DirectoryTree,
};

/// Unlock everything at <mod_path>
#[derive(Args, Clone)]
pub struct Unlock {
    /// Directory of the mod
    mod_path: PathBuf,
}

/// Wrapper around [`unlock`]
pub fn main(args: &Unlock) -> Result<(), Error> {
    unlock(&args.mod_path)
}

/// Unlock everything at the <mod_path>
pub fn unlock(mod_path: &Path) -> Result<(), Error> {
    // Check the directory structure
    let dir_tree = DirectoryTree::new(mod_path);
    if !dir_tree.exists() {
        bail!("Mod directory does not exist or is missing vital subdirectories!");
    }

    // Unlock all the songs
    println!("Unlocking all the songs");
    for song_dir in dir_tree.songs().read_dir()? {
        let song_dir = song_dir?;
        if song_dir.metadata()?.is_dir() {
            let path = song_dir.path().join("song.json");
            let file = std::fs::read(&path)?;
            let mut song = serde_json::from_slice::<Song>(&file)?;
            song.status = MapStatus::Unlocked;
            serde_json::to_writer_pretty(File::create(path)?, &song)?;
        }
    }

    // Unlock all the aliases
    println!("Unlocking all the aliases");
    let aliases_path = dir_tree.config().join("aliases.json");
    let aliases_file = std::fs::read(&aliases_path)?;
    let mut aliases = serde_json::from_slice::<Aliases>(&aliases_file)?;
    for alias in &mut aliases.aliases {
        alias.unlock_objective = None;
        alias.unlocked_by_default = true;
    }
    serde_json::to_writer_pretty(File::create(aliases_path)?, &aliases)?;

    // Unlock all the avatars
    println!("Unlocking all the avatars");
    let avatars_path = dir_tree.avatars().join("avatars.json");
    let avatars_file = std::fs::read(&avatars_path)?;
    let mut avatars = serde_json::from_slice::<HashMap<HipStr, Avatar>>(&avatars_file)?;
    for avatar in avatars.values_mut() {
        avatar.unlock_type = UnlockType::Unlocked;
        avatar.status = 3;
    }
    serde_json::to_writer_pretty(File::create(avatars_path)?, &avatars)?;

    // Unlock all the portraitborders
    println!("Unlocking all the portraitborders");
    let portraitborders_path = dir_tree.portraitborders().join("portraitborders.json");
    let portraitborders_file = std::fs::read(&portraitborders_path)?;
    let mut portraitborders =
        serde_json::from_slice::<HashMap<HipStr, PortraitBorder>>(&portraitborders_file)?;
    for portraitborder in portraitborders.values_mut() {
        portraitborder.lock_status = LockStatus::UnlockedByDefault;
    }
    serde_json::to_writer_pretty(File::create(portraitborders_path)?, &portraitborders)?;

    println!("Everything is unlocked!");
    Ok(())
}
