//! # Autodance Building
//! Build the autodance and preview audio file

use anyhow::Error;
use hipstr::HipStr;
use ubiart_toolkit::{cooked, shared_json_types, utils::SplitPath};

use super::SongExportState;
use crate::build::BuildFiles;

/// Build the autodance and preview audio file
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let cache_map_path = &ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let autodance_cache_dir = cache_map_path.join("autodance");

    // autodance actor
    let autodance_actor_vec = autodance_actor(ses)?;

    // autodance template
    let autodance_template_vec = autodance_template(ses)?;

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
fn autodance_template(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
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
                        records: Vec::new(),
                    },
                    video_structure: shared_json_types::AutodanceVideoStructure {
                        class: Some(shared_json_types::AutodanceVideoStructure::CLASS),
                        song_start_position: 0.0,
                        duration: 30.0,
                        thumbnail_time: 0,
                        fade_out_duration: 3.0,
                        ground_plane_path: HipStr::borrowed("invalid "),
                        first_layer_triple_background_path: HipStr::borrowed("invalid "),
                        second_layer_triple_background_path: HipStr::borrowed("invalid "),
                        third_layer_triple_background_path: HipStr::borrowed("invalid "),
                        playback_events: Vec::new(),
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
