//! # Video Building
//! Build the video scenes, actors, and video

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use test_eq::test_eq;
use ubiart_toolkit::{cooked, utils::SplitPath};

use super::SongExportState;
use crate::build::BuildFiles;

/// Build the video scenes, actors, and video
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let map_path = ses.map_path;
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let videoscoach_cache_dir = cache_map_path.join("videoscoach");
    let videoscoach_dir = map_path.join("videoscoach");

    // .mpd.ckd is always the same
    let mpd_vec = vec![
        0x00, 0x00, 0x00, 0x01, 0x00, 0x42, 0x4B, 0xAE, 0x14, 0x3F, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];

    // video player actors
    let video_player_main_act_vec = video_player_actor(ses, false)?;
    let video_player_map_preview_act_vec = video_player_actor(ses, true)?;

    // video player scenes
    let video_scene = video_scene(ses);
    let video_scene_vec = cooked::isc::create_vec_with_capacity_hint(&video_scene, 2700)?;
    let video_map_preview_scene_vec =
        cooked::isc::create_vec_with_capacity_hint(&video_map_preview_scene(ses), 1100)?;

    // the actual video
    let video_path = ses.dirs.song().join(ses.song.videofile.as_str());
    test_eq!(
        video_path.extension(),
        Some("webm"),
        "Video file is not a webm! Transcoding is not supported: {:?}",
        video_path
    )?;
    test_eq!(
        ses.native_vfs.exists(&video_path),
        true,
        "Video file does not exist at {:?}!",
        video_path
    )?;

    bf.generated_files.add_file(
        videoscoach_cache_dir.join(format!("{lower_map_name}.mpd.ckd")),
        mpd_vec,
    )?;
    bf.generated_files.add_file(
        videoscoach_cache_dir.join("video_player_main.act.ckd"),
        video_player_main_act_vec,
    )?;
    bf.generated_files.add_file(
        videoscoach_cache_dir.join("video_player_map_preview.act.ckd"),
        video_player_map_preview_act_vec,
    )?;
    bf.generated_files.add_file(
        videoscoach_cache_dir.join(format!("{lower_map_name}_video.isc.ckd")),
        video_scene_vec,
    )?;
    bf.generated_files.add_file(
        videoscoach_cache_dir.join(format!("{lower_map_name}_video_map_preview.isc.ckd")),
        video_map_preview_scene_vec,
    )?;

    bf.static_files.add_file(
        video_path,
        videoscoach_dir.join(format!("{lower_map_name}.vp9.720.webm")),
    )?;

    Ok(video_scene.scene.into())
}

/// Build the video player actor
///
/// If `map_preview` is true it will build the preview version of the actor
fn video_player_actor(ses: &SongExportState<'_>, map_preview: bool) -> Result<Vec<u8>, Error> {
    let map_path = ses.map_path;
    let lower_map_name = ses.lower_map_name;
    let actor = cooked::act::Actor {
        lua: SplitPath::new(
            HipStr::borrowed("world/_common/videoscreen/"),
            if map_preview {
                HipStr::borrowed("video_player_map_preview.tpl")
            } else {
                HipStr::borrowed("video_player_main.tpl")
            },
        )?,
        unk1: 0.0,
        unk2: 1.0,
        unk2_5: 1.0,
        unk3_5: 0,
        components: vec![cooked::act::Component::PleoComponent(
            cooked::act::PleoComponent {
                video: SplitPath::new(
                    HipStr::from(map_path.join("videoscoach/").into_string()),
                    HipStr::from(format!("{lower_map_name}.webm")),
                )?,
                dash_mpd: SplitPath::new(
                    HipStr::from(map_path.join("videoscoach/").into_string()),
                    HipStr::from(format!("{lower_map_name}.mpd")),
                )?,
                channel_id: map_preview
                    .then(|| ses.song.map_name.clone())
                    .unwrap_or_default(),
            },
        )],
    };

    Ok(cooked::act::create_vec(actor)?)
}

/// Build the video scene
fn video_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let map_path = ses.map_path;
    let lower_map_name = ses.lower_map_name;
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            gridunit: 0.5,
            depth_separator: 0,
            near_separator: [
                ubiart_toolkit::utils::Color { color: (1.0, 0.0, 0.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 1.0, 0.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 0.0, 1.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 0.0, 0.0, 1.0)},
            ],
            far_separator: [
                ubiart_toolkit::utils::Color { color: (1.0, 0.0, 0.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 1.0, 0.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 0.0, 1.0, 0.0)},
                ubiart_toolkit::utils::Color { color: (0.0, 0.0, 0.0, 1.0)},
            ],
            view_family: false,
            is_popup: false,
            platform_filters: Vec::new(),
            actors: vec![
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: -1.0,
                        userfriendly: HipStr::borrowed("VideoScreen"),
                        pos2d: (0.0, -4.5),
                        lua: HipStr::borrowed("world/_common/videoscreen/video_player_main.tpl"),
                        components: vec![cooked::isc::WrappedComponent::Pleo(
                            cooked::isc::PleoComponent {
                                video: HipStr::from(
                                    map_path
                                        .join(format!("videoscoach/{lower_map_name}.webm"))
                                        .into_string(),
                                ),
                                dash_mpd: HipStr::from(
                                    map_path
                                        .join(format!("videoscoach/{lower_map_name}.mpd"))
                                        .into_string(),
                                ),
                                channel_id: HipStr::borrowed(""),
                            }
                            .into(),
                        )],
                        ..Default::default()
                    }),
                }),
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        scale: (3.941_238, 2.22),
                        userfriendly: HipStr::borrowed("VideoOutput"),
                        lua: HipStr::borrowed("world/_common/videoscreen/video_output_main.tpl"),
                        components: vec![cooked::isc::WrappedComponent::PleoTextureGraphic(
                            cooked::isc::PleoTextureGraphicComponent {
                                color_computer_tag_id: 0,
                                render_in_target: false,
                                disable_light: false,
                                disable_shadow: 4_294_967_295,
                                atlas_index: 0,
                                custom_anchor: (0.0, 0.0),
                                sinus_amplitude: (0.0, 0.0, 0.0),
                                sinus_speed: 1.0,
                                angle_x: 0.0,
                                angle_y: 0.0,
                                channel_id: HipStr::borrowed(""),
                                primitive_parameters: cooked::isc::PrimitiveParameters {
                                    gfx_primitive_param: cooked::isc::GFXPrimitiveParam {
                                        color_factor: ubiart_toolkit::utils::Color { color: (1.0, 1.0, 1.0, 1.0) },
                                        enums: vec![cooked::isc::Enum {
                                            name: HipStr::borrowed("gfxOccludeInfo"),
                                            selection: 1,
                                        }],
                                    },
                                },
                                enums: vec![
                                    cooked::isc::Enum {
                                        name: HipStr::borrowed("anchor"),
                                        selection: 1,
                                    },
                                    cooked::isc::Enum {
                                        name: HipStr::borrowed("oldAnchor"),
                                        selection: 1,
                                    },
                                ],
                                material: cooked::isc::Material {
                                    gfx_material_serializable:
                                        cooked::isc::GFXMaterialSerializable {
                                            atl_channel: 0,
                                            atl_path: HipStr::borrowed(""),
                                            shader_path: HipStr::borrowed(
                                                "world/_common/matshader/pleofullscreen.msh",
                                            ),
                                            stencil_test: None,
                                            alpha_test: 4_294_967_295,
                                            alpha_ref: 4_294_967_295,
                                            texture_set: cooked::isc::TextureSet::default(),
                                            material_params: cooked::isc::MaterialParams::default(),
                                            outlined_mask_params: Some(
                                                cooked::isc::OutlinedMaskParams {
                                                    outline_mask_material_params:
                                                        cooked::isc::OutlinedMaskMaterialParams {
                                                            mask_color: ubiart_toolkit::utils::Color::default(),
                                                            outline_color: ubiart_toolkit::utils::Color::default(),
                                                            thickness: 1.0,
                                                        },
                                                },
                                            ),
                                        },
                                },
                            }
                            .into(),
                        )],
                        ..Default::default()
                    }),
                }),
            ],
            scene_configs: cooked::isc::SceneConfigs {
                active_scene_config: 0,
                jd_scene_config: Vec::new(),
            }
            .into(),
        },
    }
}

/// Build the video map preview scene
fn video_map_preview_scene<'a>(ses: &SongExportState<'a>) -> cooked::isc::Root<'a> {
    let map_path = ses.map_path;
    let lower_map_name = ses.lower_map_name;
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: 326_704,
            gridunit: 0.5,
            depth_separator: 0,
            near_separator: [
                ubiart_toolkit::utils::Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            far_separator: [
                ubiart_toolkit::utils::Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                ubiart_toolkit::utils::Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            view_family: false,
            is_popup: false,
            platform_filters: Vec::new(),
            actors: vec![cooked::isc::WrappedActors::Actor(
                cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: -1.0,
                        userfriendly: HipStr::borrowed("VideoScreen"),
                        pos2d: (0.0, -4.5),
                        lua: HipStr::borrowed(
                            "world/_common/videoscreen/video_player_map_preview.tpl",
                        ),
                        components: vec![cooked::isc::WrappedComponent::Pleo(
                            cooked::isc::PleoComponent {
                                video: HipStr::from(
                                    map_path
                                        .join(format!("videoscoach/{lower_map_name}.webm"))
                                        .into_string(),
                                ),
                                dash_mpd: HipStr::from(
                                    map_path
                                        .join(format!("videoscoach/{lower_map_name}.mpd"))
                                        .into_string(),
                                ),
                                channel_id: ses.song.map_name.clone(),
                            }
                            .into(),
                        )],
                        ..Default::default()
                    }),
                },
            )],
            scene_configs: cooked::isc::SceneConfigs {
                active_scene_config: 0,
                jd_scene_config: Vec::new(),
            }
            .into(),
        },
    }
}
