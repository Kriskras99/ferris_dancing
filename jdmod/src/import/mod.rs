//! # Import
//! The main code for importing games and songs
use std::{
    cmp::Ordering,
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Error};
use clap::Args;
use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserializeExt as _,
    vfs::{
        case_insensitive::CaseInsensitiveFs, native::NativeFs, VirtualFileSystem, VirtualPath,
        VirtualPathBuf,
    },
};
use hipstr::HipStr;
use tracing::trace;
use ubiart_toolkit::{
    alias8::Alias8,
    cooked,
    cooked::isg::{
        GameManagerConfigV16, GameManagerConfigV17, GameManagerConfigV18, GameManagerConfigV19,
        GameManagerConfigV20, GameManagerConfigV20C, GameManagerConfigV21, GameManagerConfigV22,
    },
    secure_fat::vfs::SfatFilesystem,
    utils::{Game, Platform, UniqueGameId},
};

use crate::{
    types::{localisation::LocaleIdMap, DirectoryTree, ImportState},
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
    /// Path of the game to import
    ///
    /// Supported files are: secure_fat.gf, dlcdescriptor.ckd
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
    /// Use n threads for importing songs
    #[arg(long)]
    threads: Option<NonZeroUsize>,
}

/// Wrapper around [`import`]
pub fn main(cli: &Import) -> Result<(), Error> {
    import(
        &cli.game_path,
        &cli.mod_path,
        cli.lax,
        cli.songs,
        cli.game,
        cli.threads,
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
    n_threads: Option<NonZeroUsize>,
) -> Result<(), Error> {
    // Check the directory structure
    let dir_tree = DirectoryTree::new(dir_root);
    if !dir_tree.exists() {
        return Err(anyhow!(
            "Mod directory does not exist or is missing vital subdirectories!"
        ));
    }

    if game_path.ends_with("secure_fat.gf") {
        import_sfat(game_path, dir_tree, lax, songs_only, n_threads)?;
    } else if game_path.is_dir() {
        let mut sources = Vec::new();
        find_sources(game_path, &mut sources)?;
        // TODO: Do game/platform detection for every source and sort based on that
        sources.sort();

        if sources.is_empty() {
            bail!("Did not find any sources!");
        }

        trace!("Sources: {sources:#?}");

        for source in sources {
            match source {
                Source::Dlc(path) => import_dlcdescriptor(&path, dir_tree.clone(), lax)?,
                Source::Sfat(path) => {
                    import_sfat(&path, dir_tree.clone(), lax, songs_only, n_threads)?;
                }
                Source::GameConfig(path) => {
                    import_gameconfig(&path, dir_tree.clone(), game, songs_only, n_threads)?;
                }
                Source::SongDesc(path) => import_songdesc(&path, dir_tree.clone())?,
            }
        }
    } else if game_path.ends_with("dlcdescriptor.ckd") {
        import_dlcdescriptor(game_path, dir_tree, lax)?;
    } else if game_path.ends_with("songdesc.tpl.ckd") {
        import_songdesc(game_path, dir_tree)?;
    } else {
        return Err(anyhow!("Cannot import {game_path:?}! Input not recognized, currently only secure_fat.gf, JD Now .json files, and raw import are supported!"));
    }

    Ok(())
}

/// Import a game from a secure_fat.gf
fn import_sfat(
    game_path: &Path,
    dir_tree: DirectoryTree,
    lax: bool,
    songs_only: bool,
    n_threads: Option<NonZeroUsize>,
) -> Result<(), Error> {
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
    import_full_game_vfs(
        &sfat_vfs,
        dir_tree,
        unique_game_id,
        lax,
        songs_only,
        n_threads,
    )
}

/// Import a song from a dlcdescriptor.ckd
fn import_dlcdescriptor(game_path: &Path, dir_tree: DirectoryTree, lax: bool) -> Result<(), Error> {
    // Init the native filesystem and load the securefat as a virtual filesystem
    let native_vfs = NativeFs::new(
        game_path
            .parent()
            .ok_or_else(|| anyhow!("No parent directory for secure_fat.gf!"))?,
    )?;
    let sfat_vfs = SfatFilesystem::new(&native_vfs, &VirtualPathBuf::from("secure_fat.gf"))?;

    // TODO: Check engine version and warn user they're missing an update

    let unique_game_id = sfat_vfs.unique_game_id();

    // Collect common required items in a convenient place
    let is = ImportState {
        vfs: &sfat_vfs,
        dirs: dir_tree,
        ugi: unique_game_id,
        locale_id_map: LocaleIdMap::default(),
        aliases: Alias8::default(),
        lax,
        n_threads: NonZeroUsize::new(1),
    };

    let dlcdescriptor_file = native_vfs.open(VirtualPath::new("dlcdescriptor.ckd"))?;
    let dlcdescriptor = cooked::dlcdescriptor::DlcDescriptor::deserialize(&dlcdescriptor_file)?;
    let mapname = dlcdescriptor.name;

    song::import(&is, &format!("world/jd2015/{mapname}/songdesc.tpl"))?;
    Ok(())
}

/// Import a song from songdesc.tpl.ckd
fn import_songdesc(game_path: &Path, dir_tree: DirectoryTree) -> Result<(), Error> {
    let mapname = game_path
        .parent()
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?; // mapname
    let maps_folder = mapname
        .parent()
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?; // maps
    let platform = maps_folder.parent() // world
        .and_then(Path::parent) // nx
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?;
    let root = platform.parent() // itf_cooked
        .and_then(Path::parent) // cache
        .and_then(Path::parent) // root
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?;

    let mapname = mapname
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid map name in path: {}", game_path.display()))?;
    let maps_folder = maps_folder
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid maps folder in path: {}", game_path.display()))?;
    let platform = match platform
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid platform name in path: {}", game_path.display()))?
    {
        "nx" => Platform::Nx,
        "wiiu" => Platform::WiiU,
        platform => bail!("Unsupported platform {platform}"),
    };

    // Init the native filesystem
    let native_vfs = NativeFs::new(root)?;
    let case_insenstive_vfs = CaseInsensitiveFs::new(&native_vfs)?;

    trace!("Root: {root:?}, Game path: {game_path:?}");

    let new_path = format!("world/{maps_folder}/{mapname}/songdesc.tpl");

    trace!("New path: {new_path}");

    let unique_game_id = UniqueGameId {
        game: Game::JustDance2022,
        platform,
        id: 0,
    };

    // Collect common required items in a convenient place
    let is = ImportState {
        vfs: &case_insenstive_vfs,
        dirs: dir_tree,
        ugi: unique_game_id,
        locale_id_map: LocaleIdMap::default(),
        aliases: Alias8::default(),
        lax: true,
        n_threads: NonZeroUsize::new(1),
    };

    song::import(&is, &new_path)?;
    Ok(())
}

/// Import a game from gameconfig.isg.ckd
fn import_gameconfig(
    game_path: &Path,
    dir_tree: DirectoryTree,
    game: Option<Game>,
    songs_only: bool,
    n_threads: Option<NonZeroUsize>,
) -> Result<(), Error> {
    let platform = game_path.parent() // gameconfig
        .and_then(Path::parent) // enginedata
        .and_then(Path::parent) // nx
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?;
    let root = platform.parent() // itf_cooked
        .and_then(Path::parent) // cache
        .and_then(Path::parent) // root
        .ok_or_else(|| anyhow!("No root found for {}!", game_path.display()))?;

    let platform = match platform
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("Invalid platform name in path: {}", game_path.display()))?
    {
        "nx" => Platform::Nx,
        "wiiu" => Platform::WiiU,
        platform => bail!("Unsupported platform {platform}"),
    };

    // Init the native filesystem
    let native_vfs = NativeFs::new(root)?;
    let case_insenstive_vfs = CaseInsensitiveFs::new(&native_vfs)?;

    trace!("Root: {root:?}, Game path: {game_path:?}");

    let game = if let Some(game) = game {
        game
    } else {
        let new_path = cook_path(
            "enginedata/gameconfig/enginedata.isg",
            UniqueGameId {
                platform,
                game: Game::JustDance2022,
                id: 0,
            },
        )?;
        let gameconfig_file = case_insenstive_vfs.open(VirtualPath::new(&new_path))?;
        if cooked::isg::parse::<GameManagerConfigV22>(&gameconfig_file, true).is_ok() {
            Game::JustDance2022
        } else if cooked::isg::parse::<GameManagerConfigV21>(&gameconfig_file, true).is_ok() {
            Game::JustDance2021
        } else if cooked::isg::parse::<GameManagerConfigV20>(&gameconfig_file, true).is_ok() {
            Game::JustDance2020
        } else if cooked::isg::parse::<GameManagerConfigV20C>(&gameconfig_file, true).is_ok() {
            Game::JustDanceChina
        } else if cooked::isg::parse::<GameManagerConfigV19>(&gameconfig_file, true).is_ok() {
            Game::JustDance2019
        } else if cooked::isg::parse::<GameManagerConfigV18>(&gameconfig_file, true).is_ok() {
            Game::JustDance2018
        } else if cooked::isg::parse::<GameManagerConfigV17>(&gameconfig_file, true).is_ok() {
            Game::JustDance2017
        } else if cooked::isg::parse::<GameManagerConfigV16>(&gameconfig_file, true).is_ok() {
            Game::JustDance2016
        } else {
            bail!("Invalid game config file! {}", game_path.display());
        }
    };

    let unique_game_id = UniqueGameId {
        game,
        platform,
        id: 0,
    };

    import_full_game_vfs(
        &case_insenstive_vfs,
        dir_tree,
        unique_game_id,
        true,
        songs_only,
        n_threads,
    )
}

/// Import a game represented as a virtual filesystem
pub fn import_full_game_vfs(
    vfs: &dyn VirtualFileSystem,
    dirs: DirectoryTree,
    ugi: UniqueGameId,
    lax: bool,
    songs_only: bool,
    n_threads: Option<NonZeroUsize>,
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
        n_threads,
    };

    if is.ugi.game <= Game::JustDance2015 {
        println!("Warning! Only importing songs. Avatars and other extras are not supported.");
    }

    if songs_only || is.ugi.game <= Game::JustDance2015 {
        // Get the gameconfig path
        let gameconfig_path = cook_path(
            &is.aliases
                .get_path_for_alias("gameconfig")
                .ok_or_else(|| anyhow!("common.alias8 does not contain gameconfig path!"))?,
            is.ugi,
        )?;
        let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;

        let songdb_scene = match ugi.game {
            Game::JustDance2015 => HipStr::borrowed("world/skuscenes/skuscene_maps_pc_all.isc"),
            Game::JustDance2016 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV16>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2017 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV17>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2018 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV18>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2019 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV19>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2020 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV20>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDanceChina => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV20C>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2021 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV21>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            Game::JustDance2022 => {
                let parsed_json =
                    cooked::isg::parse::<GameManagerConfigV22>(&gameconfig_file, true)?;
                parsed_json.songdb_scene
            }
            _ => {
                if lax {
                    println!("Unknown game, assuming JustDance2022");
                    let parsed_json =
                        cooked::isg::parse::<GameManagerConfigV22>(&gameconfig_file, true)?;
                    parsed_json.songdb_scene
                } else {
                    return Err(anyhow!("Unknown game"));
                }
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

/// A source for the import
#[derive(Debug, Clone, PartialEq, Eq)]
enum Source {
    /// dlcdescriptor.ckd file
    Dlc(PathBuf),
    /// secure_fat.gf file
    Sfat(PathBuf),
    /// cook_path(enginedata/gameconfig/gameconfig.isg) file
    GameConfig(PathBuf),
    /// cook_path(world/maps/../songdesc.tpl) file
    SongDesc(PathBuf),
}

impl Ord for Source {
    #[allow(clippy::match_same_arms, reason = "Clearer this way")]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Sfat(left), Self::Sfat(right)) => right.cmp(left),
            (Self::Sfat(_), _) => Ordering::Less,
            (_, Self::Sfat(_)) => Ordering::Greater,
            (Self::Dlc(left), Self::Dlc(right)) => right.cmp(left),
            (Self::Dlc(_), _) => Ordering::Less,
            (_, Self::Dlc(_)) => Ordering::Greater,
            (Self::SongDesc(left), Self::SongDesc(right)) => right.cmp(left),
            (Self::SongDesc(_), _) => Ordering::Less,
            (_, Self::SongDesc(_)) => Ordering::Greater,
            (Self::GameConfig(left), Self::GameConfig(right)) => right.cmp(left),
        }
    }
}

impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Find everything that looks like we can import it
fn find_sources(path: &Path, sources: &mut Vec<Source>) -> Result<(), Error> {
    trace!("Checking path: {}", path.display());
    if path.join("dlcdescriptor.ckd").exists() {
        sources.push(Source::Dlc(path.join("dlcdescriptor.ckd")));
    } else if path.join("secure_fat.gf").exists() {
        sources.push(Source::Sfat(path.join("secure_fat.gf")));
    } else if path
        .join("cache/itf_cooked/nx/enginedata/gameconfig/gameconfig.isg.ckd")
        .exists()
    {
        sources.push(Source::GameConfig(path.join(
            "cache/itf_cooked/nx/enginedata/gameconfig/gameconfig.isg.ckd",
        )));
    } else if path
        .join("cache/itf_cooked/wiiu/enginedata/gameconfig/gameconfig.isg.ckd")
        .exists()
    {
        sources.push(Source::GameConfig(path.join(
            "cache/itf_cooked/wiiu/enginedata/gameconfig/gameconfig.isg.ckd",
        )));
    } else if path.join("cache/itf_cooked/nx/world/maps").exists() {
        let maps = path.join("cache/itf_cooked/nx/world/maps");
        trace!("Looking for maps: {}", maps.display());
        for dir in maps.read_dir()? {
            let path = dir?.path().join("songdesc.tpl.ckd");
            if path.exists() {
                sources.push(Source::SongDesc(path));
            }
        }
    } else if path.join("cache/itf_cooked/wiiu/world/maps").exists() {
        let maps = path.join("cache/itf_cooked/wiiu/world/maps");
        trace!("Looking for maps: {}", maps.display());
        for dir in maps.read_dir()? {
            let path = dir?.path().join("songdesc.tpl.ckd");
            if path.exists() {
                sources.push(Source::SongDesc(path));
            }
        }
    } else {
        for dir in path.read_dir()? {
            let dir = dir?;
            if dir.file_type()?.is_dir() {
                find_sources(&dir.path(), sources)?;
            }
        }
    }
    Ok(())
}
