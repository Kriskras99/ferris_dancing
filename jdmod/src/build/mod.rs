//! # Build
//! Logic for building the mod
use std::borrow::Cow;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{symlinkfs::SymlinkFs, vecfs::VecFs, VirtualFileSystem};
use ubiart_toolkit::{
    cooked::{self, isc, sgs},
    utils::{Game, Platform},
};

use crate::{types::DirectoryTree, utils::cook_path};

pub mod gameconfig;
pub mod localisation;
pub mod song;

/// State that is used a lot during the build
pub struct BuildState<'a> {
    /// Filesystem containing the files in bundle_nx.ipk and patch_nx.ipk
    pub patched_base_vfs: &'a dyn VirtualFileSystem,
    /// Mod directory tree
    pub dirs: &'a DirectoryTree,
    /// Export platform
    pub platform: Platform,
    /// Export game
    pub game: Game,
    /// Export Engine version
    pub engine_version: u32,
}

/// Files collected during the build
pub struct BuildFiles<'fs> {
    /// Files generated during the build
    pub generated_files: VecFs,
    /// Files that can be copied verbatim from the mod directory
    pub static_files: SymlinkFs<'fs>,
}

impl BuildFiles<'_> {
    /// Merge two collections of build files
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        self.generated_files.merge(other.generated_files);
        self.static_files.merge(other.static_files)?;
        Ok(())
    }

    /// Get the size of a build file
    pub fn size(&self) -> Result<u64, Error> {
        Ok(self.generated_files.size() + self.static_files.size()?)
    }
}

/// Builds the song database
pub fn song_database(
    bs: &BuildState,
    bf: &mut BuildFiles,
    song_names: &[String],
) -> Result<(), Error> {
    println!("Building song database...");
    let sgscontainer_path = cook_path("sgscontainer", bs.platform)?;
    let sgscontainer_file = bs.patched_base_vfs.open(sgscontainer_path.as_ref())?;
    let mut sgscontainer = sgs::parse_sgscontainer(&sgscontainer_file)?.clone();

    // Remove all maps and map lists from sgscontainer
    sgscontainer.sgs_map.keys.retain(|k, _| {
        !k.starts_with("world/maps") || k.starts_with("world/skuscenes/skuscene_maps_")
    });

    // We need to store the song name in 3 places:
    // The song database (as the path to the main scene)
    let mut sgscontainer_skuscene_nx = sgs::SongDatabaseSceneConfig::default();
    // The cover database (as the actor for the cover for generic and online)
    let mut isc_coverflow_sku_songs = Vec::new();
    let mut actors = vec![cooked::isc::WrappedActors::Actor(
        cooked::isc::WrappedActor {
            actor: cooked::isc::Actor {
                userfriendly: Cow::Borrowed("skuscene_db"),
                lua: Cow::Borrowed("world/skuscenes/skuscene_base.tpl"),
                components: vec![cooked::isc::WrappedComponent::SongDatabase],
                ..Default::default()
            },
        },
    )];

    for song_name in song_names {
        let lower_song_name = song_name.to_lowercase();
        let song_name = Cow::Borrowed(song_name.as_str());

        sgscontainer.sgs_map.keys.insert(
            Cow::Owned(format!(
                "world/maps/{lower_song_name}/{lower_song_name}_main_scene.isc"
            )),
            sgs::Settings::MapSceneConfig(sgs::MapSceneConfig::default()),
        );

        sgscontainer_skuscene_nx
            .coverflow_sku_songs
            .push(sgs::CoverflowSong {
                class: Some(sgs::CoverflowSong::CLASS),
                name: song_name.clone(),
                cover_path: Cow::Owned(format!(
                "world/maps/{lower_song_name}/menuart/actors/{lower_song_name}_cover_generic.act"
            )),
            });

        sgscontainer_skuscene_nx
            .coverflow_sku_songs
            .push(sgs::CoverflowSong {
                class: Some(sgs::CoverflowSong::CLASS),
                name: song_name.clone(),
                cover_path: Cow::Owned(format!(
                "world/maps/{lower_song_name}/menuart/actors/{lower_song_name}_cover_online.act"
            )),
            });

        isc_coverflow_sku_songs.push(isc::CoverflowSkuSongs {
            coverflow_song: isc::CoverflowSong {
                name: song_name.clone(),
                cover_path: Cow::Owned(format!("world/maps/{lower_song_name}/menuart/actors/{lower_song_name}_cover_generic.act")),
            },
        });

        isc_coverflow_sku_songs.push(isc::CoverflowSkuSongs {
            coverflow_song: isc::CoverflowSong {
                name: song_name.clone(),
                cover_path: Cow::Owned(format!("world/maps/{lower_song_name}/menuart/actors/{lower_song_name}_cover_online.act")),
            },
        });

        actors.push(isc::WrappedActors::Actor(isc::WrappedActor {
            actor: isc::Actor {
                userfriendly: song_name,
                lua: Cow::Owned(format!("world/maps/{lower_song_name}/songdesc.tpl")),
                components: vec![isc::WrappedComponent::SongDesc],
                ..Default::default()
            },
        }));
    }

    let sgscontainer_skuscene_pc = sgscontainer_skuscene_nx.clone();
    let sgs_skuscene = sgs::SceneSettings {
        settings: sgs::Settings::SongDatabaseSceneConfig(sgscontainer_skuscene_nx.clone()),
    };
    sgscontainer.sgs_map.keys.insert(
        Cow::Borrowed("world/skuscenes/skuscene_maps_pc_all.isc"),
        sgs::Settings::SongDatabaseSceneConfig(sgscontainer_skuscene_pc),
    );
    sgscontainer.sgs_map.keys.insert(
        Cow::Borrowed("world/skuscenes/skuscene_maps_nx_all.isc"),
        sgs::Settings::SongDatabaseSceneConfig(sgscontainer_skuscene_nx),
    );

    // Create sgscontainer
    let sgscontainer_vec = sgs::create_sgscontainer_vec(&sgscontainer)?;
    // Create world/skuscenes/skuscene_maps_{nx,pc}_all.sgs
    // Same file content, so just have to generate once
    let skuscene_maps_sgs_vec = sgs::create_sgs_vec(&sgs_skuscene)?;

    bf.generated_files
        .add_file(sgscontainer_path, sgscontainer_vec);

    bf.generated_files.add_file(
        cook_path("world/skuscenes/skuscene_maps_nx_all.sgs", bs.platform)?,
        skuscene_maps_sgs_vec.clone(),
    );
    bf.generated_files.add_file(
        cook_path("world/skuscenes/skuscene_maps_pc_all.sgs", bs.platform)?,
        skuscene_maps_sgs_vec,
    );

    // Create world/skuscenes/skuscene_maps_{nx,pc}_all.isc
    let isc_skuscene_nx = isc::Root {
        scene: isc::Scene {
            engine_version: bs.engine_version,
            actors,
            scene_configs: isc::WrappedSceneConfigs {
                scene_configs: isc::SceneConfigs {
                    active_scene_config: 0,
                    jd_scene_config: vec![isc::WrappedJdSceneConfig::SongDatabase(
                        isc::WrappedSongDatabaseSceneConfig {
                            song_database_scene_config: isc::SongDatabaseSceneConfig {
                                sku: Cow::Borrowed("jd2022-nx-all"),
                                rating_ui: Cow::Borrowed(
                                    "world/ui/screens/boot_warning/boot_warning_esrb.isc",
                                ),
                                coverflow_sku_songs: isc_coverflow_sku_songs,
                                ..Default::default()
                            },
                        },
                    )],
                },
            },
            ..Default::default()
        },
    };

    let skuscene_maps_isc_vec = isc::create_vec_with_capacity_hint(&isc_skuscene_nx, 230_000)?;

    bf.generated_files.add_file(
        cook_path("world/skuscenes/skuscene_maps_nx_all.isc", bs.platform)?,
        skuscene_maps_isc_vec.clone(),
    );
    bf.generated_files.add_file(
        cook_path("world/skuscenes/skuscene_maps_pc_all.isc", bs.platform)?,
        skuscene_maps_isc_vec,
    );

    Ok(())
}
