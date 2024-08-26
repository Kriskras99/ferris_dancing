//! # Song building
//! All the logic for building a song
use std::borrow::Cow;

use anyhow::Error;
use dotstar_toolkit_utils::{
    path,
    vfs::{native::NativeFs, VirtualFileSystem, VirtualPath, VirtualPathBuf},
};
use ubiart_toolkit::{cooked, utils::Platform};

use super::{BuildFiles, BuildState};
use crate::types::song::{RelativeSongDirectoryTree, Song};

mod audio;
mod autodance;
mod mainsequence;
mod menuart;
mod songdescription;
mod timeline;
mod videoscoach;

/// State used a lot during song building
pub struct SongExportState<'a> {
    /// Lowercase version of the map codename
    pub lower_map_name: &'a str,
    /// The map path in the game
    pub map_path: &'a VirtualPath,
    /// Cache version of the map path
    pub cache_map_path: &'a VirtualPath,
    /// Song directory tree
    pub dirs: RelativeSongDirectoryTree,
    /// Export platform
    pub platform: Platform,
    /// Export Engine version
    pub engine_version: u32,
    /// The song metadata
    pub song: Song<'a>,
    /// Vfs with mod directory as the root
    pub native_vfs: &'a NativeFs,
}

/// Build the song at `dirs`
pub fn build(
    bs: &BuildState<'_>,
    bf: &mut BuildFiles,
    dirs: RelativeSongDirectoryTree,
) -> Result<String, Error> {
    let song_file = bs.native_vfs.open(&dirs.song().join("song.json"))?;
    let song: Song = serde_json::from_slice(&song_file)?;
    let map_name = song.map_name.as_ref();
    let lower_map_name = map_name.to_lowercase();
    let cache_map_path = path!("cache/itf_cooked/nx/world/maps/{lower_map_name}");
    let map_path = path!("world/maps/{lower_map_name}");
    println!("Building song '{map_name}'...");

    let ses = SongExportState {
        lower_map_name: &lower_map_name,
        dirs,
        platform: bs.platform,
        song,
        cache_map_path: &cache_map_path,
        map_path: &map_path,
        engine_version: bs.engine_version,
        native_vfs: bs.native_vfs,
    };

    // Build the songdescription
    songdescription::build(&ses, bf)?;

    // Build the main scene sgs
    let main_scene_sgs = cooked::sgs::MapSceneConfig::default();
    let scene_settings = cooked::sgs::create_vec(&cooked::sgs::Sgs::SceneSettings(
        cooked::sgs::SceneSettings {
            settings: cooked::sgs::Settings::MapSceneConfig(main_scene_sgs),
        },
    ))?;

    // Build all scene files
    // Builds the (empty) graph scene
    let graph_scene = graph_scene(&ses, bf)?;

    // Builds video scene and adds the video file for copying
    let video_scene = videoscoach::build(&ses, bf)?;

    // Builds tml scene and the karaoke/dance timelines and related files
    let tml_scene = timeline::build(&ses, bf)?;

    // Builds menuart scene and all related textures
    let menuart_scene = menuart::build(&ses, bf)?;

    // Builds main sequence scene and adds the audio file for copying
    let mainsequence_scene = mainsequence::build(&ses, bf)?;

    // Builds autodance scene and adds the preview audio file for copying
    let autodance_scene = autodance::build(&ses, bf)?;

    // Builds audio scene and musictrack/sequence/tape for audio
    let audio_scene = audio::build(&ses, bf)?;

    let main_scene = main_scene(
        &ses,
        audio_scene,
        mainsequence_scene,
        graph_scene,
        tml_scene,
        video_scene,
        menuart_scene,
        autodance_scene,
    );

    // Save the main scene
    let main_scene_vec = cooked::isc::create_vec_with_capacity_hint(&main_scene, 35_000)?;

    bf.generated_files.add_file(
        cache_map_path.join(format!("{lower_map_name}_main_scene.sgs.ckd")),
        scene_settings,
    )?;
    bf.generated_files.add_file(
        cache_map_path.join(format!("{lower_map_name}_main_scene.isc.ckd")),
        main_scene_vec,
    )?;

    Ok(ses.song.map_name.to_string())
}

/// Build the graph scene
fn graph_scene(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let root = cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            gridunit: 0.5,
            depth_separator: 0,
            near_separator: [
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ],
            far_separator: [
                (1.0, 0.0, 0.0, 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ],
            view_family: false,
            is_popup: false,
            platform_filters: Vec::new(),
            actors: vec![cooked::isc::WrappedActors::Actor(
                cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: 10.0,
                        userfriendly: Cow::Borrowed("Camera_JD_Dummy"),
                        lua: Cow::Borrowed("enginedata/actortemplates/tpl_emptyactor.tpl"),
                        ..Default::default()
                    }),
                },
            )],
            scene_configs: cooked::isc::WrappedSceneConfigs {
                scene_configs: cooked::isc::SceneConfigs {
                    active_scene_config: 0,
                    jd_scene_config: Vec::new(),
                },
            },
        },
    };

    let graph_scene_vec = cooked::isc::create_vec_with_capacity_hint(&root, 791)?;

    bf.generated_files.add_file(
        cache_map_path.join(format!("{lower_map_name}_graph.isc.ckd")),
        graph_scene_vec,
    )?;

    Ok(cooked::isc::WrappedScene { scene: root.scene })
}

/// Build the main scene from all the subscenes
#[allow(clippy::too_many_arguments, reason = "It's just easier this way")]
fn main_scene<'a>(
    ses: &SongExportState<'a>,
    audio_scene: cooked::isc::WrappedScene<'a>,
    mainsequence_scene: cooked::isc::WrappedScene<'a>,
    graph_scene: cooked::isc::WrappedScene<'a>,
    tml_scene: cooked::isc::WrappedScene<'a>,
    video_scene: cooked::isc::WrappedScene<'a>,
    menuart_scene: cooked::isc::WrappedScene<'a>,
    autodance_scene: cooked::isc::WrappedScene<'a>,
) -> cooked::isc::Root<'a> {
    let lower_map_name = ses.lower_map_name;
    let map_name = ses.song.map_name.as_ref();

    let mut actors = Vec::with_capacity(8);
    for (scene, userfriendly, directory, file, view_type, pos2d) in [
        (audio_scene, "AUDIO", "audio", "audio", 2, (0.0, 0.0)),
        (
            mainsequence_scene,
            "CINE",
            "cinematics",
            "cine",
            2,
            (0.0, 0.0),
        ),
        (graph_scene, "GRAPH", "graph", "graph", 2, (0.0, 0.0)),
        (tml_scene, "TML", "tml", "timeline", 2, (0.0, 0.0)),
        (video_scene, "VIDEO", "videoscoach", "video", 2, (0.0, 0.0)),
        (
            menuart_scene,
            "menuart",
            "menuart",
            "menuart",
            3,
            (0.0, 0.0),
        ),
        (
            autodance_scene,
            "AUTODANCE",
            "autodance",
            "autodance",
            2,
            (0.0, -0.033_823),
        ),
    ] {
        actors.push(cooked::isc::WrappedActors::SubSceneActor(
            cooked::isc::WrappedSubSceneActor {
                sub_scene_actor: Box::new(cooked::isc::SubSceneActor {
                    userfriendly: Cow::Owned(format!("{map_name}_{userfriendly}")),
                    pos2d,
                    lua: Cow::Borrowed("enginedata/actortemplates/subscene.tpl"),
                    relativepath: Cow::Owned(format!(
                        "world/maps/{lower_map_name}/{directory}/{lower_map_name}_{file}.isc"
                    )),
                    enums: vec![cooked::isc::Enum {
                        name: Cow::Borrowed("viewType"),
                        selection: view_type,
                    }],
                    wrapped_scene: scene,
                    ..Default::default()
                }),
            },
        ));
    }

    actors.push(cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor { actor: Box::new(cooked::isc::Actor {
        // This is not an oversight, the JDVer, ID, Type, Flags, NbCoach, and Difficulty do not change per map
        userfriendly: Cow::Owned(format!("{map_name} : Template Artist - Template Title&#10;JDVer = 5, ID = 842776738, Type = 1 (Flags 0x00000000), NbCoach = 2, Difficulty = 2")),
        pos2d: (-3.531_976, -1.485_322),
        lua: Cow::Owned(format!("world/maps/{lower_map_name}/songdesc.tpl")),
        components: vec![cooked::isc::WrappedComponent::SongDesc],
        ..Default::default()
    })}));

    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            gridunit: 2.0,
            depth_separator: 0,
            actors,
            scene_configs: cooked::isc::WrappedSceneConfigs {
                scene_configs: cooked::isc::SceneConfigs {
                    active_scene_config: 0,
                    jd_scene_config: vec![cooked::isc::WrappedJdSceneConfig::Map(
                        cooked::isc::WrappedMapSceneConfig {
                            map_scene_config: cooked::isc::MapSceneConfig {
                                name: Cow::Borrowed(""),
                                sound_context: Cow::Borrowed(""),
                                hud: 0,
                                enums: vec![
                                    cooked::isc::Enum {
                                        name: Cow::Borrowed("Pause_Level"),
                                        selection: 6,
                                    },
                                    cooked::isc::Enum {
                                        name: Cow::Borrowed("type"),
                                        selection: 1,
                                    },
                                    cooked::isc::Enum {
                                        name: Cow::Borrowed("musicscore"),
                                        selection: 2,
                                    },
                                ],
                                phone_title_loc_id: None,
                                phone_image: None,
                            },
                        },
                    )],
                },
            },
            ..Default::default()
        },
    }
}
