//! # Autodance Building
//! Build the autodance and preview audio file
use std::{borrow::Cow, fs::File, path::PathBuf};

use anyhow::Error;
use ubiart_toolkit::{cooked, json_types, utils::SplitPath};

use crate::{build::BuildFiles, types::song::Autodance};

use super::SongExportState;

/// Build the autodance and preview audio file
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let map_path = ses.map_path;
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let autodance_cache_dir = format!("{cache_map_path}/autodance");
    let autodance_dir = format!("{map_path}/autodance");

    let autodance: Autodance =
        serde_json::from_reader(File::open(ses.dirs.song().join("autodance.json"))?)?;

    // autodance actor
    let autodance_actor_vec = autodance_actor(ses)?;

    // autodance template
    let autodance_template_vec = autodance_template(ses, &autodance)?;

    // Cine scene
    let autodance_scene = autodance_scene(ses);
    let autodance_scene_vec = cooked::isc::create_vec_with_capacity_hint(&autodance_scene, 900)?;

    bf.generated_files.add_file(
        format!("{autodance_cache_dir}/{lower_map_name}_autodance.act.ckd"),
        autodance_actor_vec,
    );

    bf.generated_files.add_file(
        format!("{autodance_cache_dir}/{lower_map_name}_autodance.tpl.ckd"),
        autodance_template_vec,
    );

    bf.generated_files.add_file(
        format!("{autodance_cache_dir}/{lower_map_name}_autodance.isc.ckd"),
        autodance_scene_vec,
    );

    // preview audio file
    let from = ses.dirs.audio().join("autodance.ogg");
    let to = PathBuf::from(format!("{autodance_dir}/{lower_map_name}.ogg"));
    if bf.static_files.add_file(from, to).is_err() {
        println!("Warning! Missing autodance.ogg for {lower_map_name}!");
    }

    Ok(cooked::isc::WrappedScene {
        scene: autodance_scene.scene,
    })
}

/// Build the autodance actor
fn autodance_actor(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let actor = cooked::act::Actor {
        tpl: SplitPath {
            path: Cow::Owned(format!("world/maps/{lower_map_name}/autodance/")),
            filename: Cow::Owned(format!("{lower_map_name}_autodance.tpl")),
        },
        unk1: 0,
        unk2: 0x3f80_0000,
        unk2_5: 0x3f80_0000,
        templates: vec![cooked::act::Template {
            the_type: cooked::act::TemplateType::Autodance,
            data: cooked::act::TemplateData::None,
        }],
    };

    cooked::act::create_vec(&actor)
}

/// Build the autodance scene
fn autodance_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let lower_map_name = ses.lower_map_name;
    let map_name = ses.song.map_name.as_ref();
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            actors: vec![cooked::isc::WrappedActors::Actor(
                cooked::isc::WrappedActor {
                    actor: cooked::isc::Actor {
                        userfriendly: Cow::Owned(format!("{map_name}_autodance")),
                        pos2d: (-0.006_150, -0.003_075),
                        lua: Cow::Owned(format!(
                            "world/maps/{lower_map_name}/autodance/{lower_map_name}_autodance.tpl"
                        )),
                        components: vec![cooked::isc::WrappedComponent::Autodance],
                        ..Default::default()
                    },
                },
            )],
            ..Default::default()
        },
    }
}

/// Build the autodance template
fn autodance_template(ses: &SongExportState<'_>, autodance: &Autodance) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        class: None,
        wip: 0,
        lowupdate: 0,
        update_layer: 0,
        procedural: 0,
        startpaused: 0,
        forceisenvironment: 0,
        components: vec![json_types::v22::Template22::AutodanceComponent(
            json_types::AutodanceComponent {
                class: None,
                song: ses.song.map_name.clone(),
                autodance_data: json_types::AutodanceData {
                    class: Some(json_types::AutodanceData::CLASS),
                    recording_structure: json_types::AutodanceRecordingStructure {
                        class: Some(json_types::AutodanceRecordingStructure::CLASS),
                        records: autodance.record.iter().map(Into::into).collect(),
                    },
                    video_structure: json_types::AutodanceVideoStructure {
                        class: Some(json_types::AutodanceVideoStructure::CLASS),
                        song_start_position: autodance.song_start_position,
                        duration: autodance.duration,
                        thumbnail_time: 0,
                        fade_out_duration: 3.0,
                        ground_plane_path: Cow::Borrowed("invalid "),
                        first_layer_triple_background_path: Cow::Borrowed("invalid "),
                        second_layer_triple_background_path: Cow::Borrowed("invalid "),
                        third_layer_triple_background_path: Cow::Borrowed("invalid "),
                        playback_events: autodance
                            .playback_events
                            .as_slice()
                            .iter()
                            .map(Into::into)
                            .collect(),
                        background_effect: Box::<json_types::AutoDanceFxDesc>::default(),
                        player_effect: Box::<json_types::AutoDanceFxDesc>::default(),
                        background_effect_events: Vec::new(),
                        player_effect_events: Vec::new(),
                        prop_events: Vec::new(),
                        props: Vec::new(),
                        props_players_config: Vec::new(),
                    },
                    autodance_sound_path: Cow::Owned(format!(
                        "world/maps/{lower_map_name}/autodance/{lower_map_name}.ogg"
                    )),
                },
            },
        )],
    });

    cooked::json::create_vec(&template)
}
