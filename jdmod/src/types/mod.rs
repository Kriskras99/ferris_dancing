//! # Types
//! This module contains all the types that are shared between the various portions of this application
use std::path::{Path, PathBuf};

use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::{
    alias8::Alias8,
    utils::{Game, GamePlatform, Platform},
};

use self::localisation::LocaleIdMap;

pub mod gameconfig;
pub mod localisation;
pub mod song;

/// Values needed when exporting
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    /// Game platform version
    pub game_platform: GamePlatform,
    /// Engine version
    pub engine_version: u32,
    /// Unk4 of the IPK archives
    pub ipk_unk4: u32,
}

/// Common state when importing
pub struct ImportState<'a> {
    /// Filesystem with the import files
    pub vfs: &'a dyn VirtualFileSystem,
    /// Mod directory tree
    pub dirs: DirectoryTree,
    /// Import game
    pub game: Game,
    /// Import platform
    pub platform: Platform,
    /// Mapping of game locale id to mod locale id
    pub locale_id_map: LocaleIdMap,
    /// See [`Alias8`]
    pub aliases: Alias8<'a>,
    /// Should we be lax with parsing
    pub lax: bool,
}

/// The directory tree of a mod
pub struct DirectoryTree {
    /// The root of the mod
    dir_root: PathBuf,
    /// The .mod directory, used for non-user editable config files
    dir_root_mod: PathBuf,
    /// The .mod/base directory, used for storing base_nx.ipk and patch_nx.ipk
    dir_root_mod_base: PathBuf,
    /// The songs directory
    dir_root_songs: PathBuf,
    /// The config directory, used for user editable config files
    dir_root_config: PathBuf,
    /// The translations directory
    dir_root_translations: PathBuf,
    /// The playlists directory
    dir_root_playlists: PathBuf,
    /// The avatars directory
    dir_root_avatars: PathBuf,
    /// The portraitborders directory
    dir_root_portraitborders: PathBuf,
}

impl DirectoryTree {
    /// Create a new directory tree from root.
    ///
    /// This does not create directories or check if they exists!
    pub fn new(dir_root: &Path) -> Self {
        let dir_root = dir_root.clean();
        let dir_root_mod = dir_root.join(".mod");
        let dir_root_mod_base = dir_root_mod.join("base");
        let dir_root_songs = dir_root.join("songs");
        let dir_root_config = dir_root.join("config");
        let dir_root_translations = dir_root.join("translations");
        let dir_root_playlists = dir_root.join("playlists");
        let dir_root_avatars = dir_root.join("avatars");
        let dir_root_portraitborders = dir_root.join("portraitborders");
        Self {
            dir_root,
            dir_root_mod,
            dir_root_mod_base,
            dir_root_songs,
            dir_root_config,
            dir_root_translations,
            dir_root_playlists,
            dir_root_avatars,
            dir_root_portraitborders,
        }
    }

    /// Create the directory tree.
    pub fn create_all(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.dir_root)
            .and_then(|()| std::fs::create_dir(&self.dir_root_mod))
            .and_then(|()| std::fs::create_dir(&self.dir_root_mod_base))
            .and_then(|()| std::fs::create_dir(&self.dir_root_songs))
            .and_then(|()| std::fs::create_dir(&self.dir_root_config))
            .and_then(|()| std::fs::create_dir(&self.dir_root_translations))
            .and_then(|()| std::fs::create_dir(&self.dir_root_playlists))
            .and_then(|()| std::fs::create_dir(&self.dir_root_avatars))
            .and_then(|()| std::fs::create_dir(&self.dir_root_portraitborders))
    }

    /// Check if the directory tree exists.
    pub fn exists(&self) -> bool {
        self.dir_root.exists()
            && self.dir_root_mod.exists()
            && self.dir_root_mod_base.exists()
            && self.dir_root_songs.exists()
            && self.dir_root_config.exists()
            && self.dir_root_translations.exists()
            && self.dir_root_playlists.exists()
            && self.dir_root_avatars.exists()
            && self.dir_root_portraitborders.exists()
    }

    // /// The root of the mod directory.
    // pub fn root(&self) -> &Path {
    //     &self.dir_root
    // }

    /// .mod directory, used for storing data the user is not expected to change.
    pub fn dot_mod(&self) -> &Path {
        &self.dir_root_mod
    }

    /// Used to store (some of) the .ipk files of the base game.
    pub fn base(&self) -> &Path {
        &self.dir_root_mod_base
    }

    /// Used to store all the parsed songs.
    pub fn songs(&self) -> &Path {
        &self.dir_root_songs
    }

    /// Used to store the translations.
    pub fn translations(&self) -> &Path {
        &self.dir_root_translations
    }

    /// Used to store everything that doesn't need it owns directory but does need to be user changeable.
    pub fn config(&self) -> &Path {
        &self.dir_root_config
    }

    /// Used to store the playlists and their covers
    pub fn playlists(&self) -> &Path {
        &self.dir_root_playlists
    }

    /// Used to store the avatars
    pub fn avatars(&self) -> &Path {
        &self.dir_root_avatars
    }

    /// Used to store the portraitborders
    pub fn portraitborders(&self) -> &Path {
        &self.dir_root_portraitborders
    }
}
