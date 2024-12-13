//! # Autodance Building
//! Build the autodance and preview audio file

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use ubiart_toolkit::{cooked, shared_json_types, utils::SplitPath};

use super::SongExportState;
use crate::{build::BuildFiles, types::song::Autodance};

/// Build the autodance and preview audio file
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let map_path = &ses.map_path;
    let cache_map_path = &ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let autodance_cache_dir = cache_map_path.join("autodance");
    let autodance_dir = map_path.join("autodance");

    let autodance_file = ses
        .native_vfs
        .open(&ses.dirs.song().join("autodance.json"))?;
    let autodance: Autodance = serde_json::from_slice(&autodance_file)?;

    // autodance actor
    let autodance_actor_vec = autodance_actor(ses)?;

    // autodance template
    let autodance_template_vec = autodance_template(ses, &autodance)?;

    // Cine scene
    let autodance_scene = autodance_scene(ses);
    let autodance_scene_vec = cooked::isc::create_vec_with_capacity_hint(&autodance_scene, 900)?;

    bf.generated_files.add_file(
        autodance_cache_dir.join(format!("{lower_map_name}_autodance.act.ckd")),
        autodance_actor_vec,
    )?;

    bf.generated_files.add_file(
        autodance_cache_dir.join(format!("{lower_map_name}_autodance.tpl.ckd")),
        autodance_template_vec,
    )?;

    bf.generated_files.add_file(
        autodance_cache_dir.join(format!("{lower_map_name}_autodance.isc.ckd")),
        autodance_scene_vec,
    )?;

    // preview audio file
    let from = ses.dirs.audio().join("autodance.ogg");
    let to = autodance_dir.join(format!("{lower_map_name}.ogg"));
    if bf.static_files.add_file(from, to).is_err() {
        println!("Warning! Missing autodance.ogg for {lower_map_name}!");
    }

    Ok(autodance_scene.scene.into())
}

/// Build the autodance actor
fn autodance_actor(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let actor = cooked::act::Actor {
        lua: SplitPath::new(
            HipStr::from(format!("world/maps/{lower_map_name}/autodance/")),
            HipStr::from(format!("{lower_map_name}_autodance.tpl")),
        )?,
        unk1: 0.0,
        unk2: 1.0,
        unk2_5: 1.0,
        unk3_5: 0,
        components: vec![cooked::act::Component::AutodanceComponent],
    };

    Ok(cooked::act::create_vec(actor)?)
}

/// Build the autodance scene
fn autodance_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let lower_map_name = ses.lower_map_name;
    let map_name = ses.song.map_name.as_str();
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            actors: vec![cooked::isc::WrappedActors::Actor(
                cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        userfriendly: HipStr::from(format!("{map_name}_autodance")),
                        pos2d: (-0.006_150, -0.003_075),
                        lua: HipStr::from(format!(
                            "world/maps/{lower_map_name}/autodance/{lower_map_name}_autodance.tpl"
                        )),
                        components: vec![cooked::isc::WrappedComponent::Autodance(
                            cooked::isc::Autodance::default(),
                        )],
                        ..Default::default()
                    }),
                },
            )],
            ..Default::default()
        },
    }
}

/// Build the autodance template
fn autodance_template(ses: &SongExportState<'_>, autodance: &Autodance) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let template = cooked::tpl::types::Actor {
        class: cooked::tpl::types::Actor::CLASS,
        wip: 0,
        lowupdate: 0,
        update_layer: 0,
        procedural: 0,
        startpaused: 0,
        forceisenvironment: 0,
        components: vec![cooked::tpl::types::Template::AutodanceComponent(
            cooked::tpl::types::AutodanceComponent {
                class: None,
                song: ses.song.map_name.clone(),
                autodance_data: cooked::tpl::types::AutodanceData {
                    class: Some(cooked::tpl::types::AutodanceData::CLASS),
                    recording_structure: cooked::tpl::types::AutodanceRecordingStructure {
                        class: Some(cooked::tpl::types::AutodanceRecordingStructure::CLASS),
                        records: autodance.record.iter().map(Into::into).collect(),
                    },
                    video_structure: shared_json_types::AutodanceVideoStructure {
                        class: Some(shared_json_types::AutodanceVideoStructure::CLASS),
                        song_start_position: autodance.song_start_position,
                        duration: autodance.duration,
                        thumbnail_time: 0,
                        fade_out_duration: 3.0,
                        ground_plane_path: HipStr::borrowed("invalid "),
                        first_layer_triple_background_path: HipStr::borrowed("invalid "),
                        second_layer_triple_background_path: HipStr::borrowed("invalid "),
                        third_layer_triple_background_path: HipStr::borrowed("invalid "),
                        playback_events: autodance
                            .playback_events
                            .as_slice()
                            .iter()
                            .map(Into::into)
                            .collect(),
                        background_effect: Box::default(),
                        player_effect: Box::default(),
                        background_effect_events: Vec::new(),
                        player_effect_events: Vec::new(),
                        prop_events: Vec::new(),
                        props: Vec::new(),
                        props_players_config: Vec::new(),
                        game_mode: None,
                        animated_frame_path: None,
                    },
                    autodance_sound_path: HipStr::from(format!(
                        "world/maps/{lower_map_name}/autodance/{lower_map_name}.ogg"
                    )),
                },
            },
        )],
    };

    Ok(cooked::json::create_vec(&template)?)
}
