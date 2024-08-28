//! # Audio Building
//! Build the audio and musictrack
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::{cooked, json_types};

use super::SongExportState;
use crate::{
    build::BuildFiles,
    types::song::MusicTrack,
    utils::{self},
};

/// Build the audio and musictrack
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let map_path = &ses.map_path;
    let cache_map_path = &ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let audio_cache_dir = cache_map_path.join("audio");
    let audio_dir = map_path.join("audio");

    // audio file
    let audio_file_path = {
        let source_file_path = ses.dirs.audio().join(ses.song.audiofile.as_ref());
        let extension = source_file_path
            .extension()
            .ok_or_else(|| anyhow!("Invalid or missing extension! {source_file_path}"))?;
        match extension {
            "wav" | "opus" => {
                let muxed = utils::encode_audio(ses.native_vfs, &source_file_path)?;
                let cache_audio_file_path =
                    audio_cache_dir.join(format!("{lower_map_name}.wav.ckd"));
                let audio_file_path = audio_dir.join(format!("{lower_map_name}.wav"));
                bf.generated_files.add_file(cache_audio_file_path, muxed)?;
                audio_file_path
            }
            "ogg" => {
                let audio_file_path = audio_dir.join(format!("{lower_map_name}.ogg"));
                bf.static_files
                    .add_file(source_file_path, audio_file_path.clone())?;
                audio_file_path
            }
            _ => return Err(anyhow!("Unknown file extension: {extension}")),
        }
    };

    // musictrack template
    let musictrack_template_vec = musictrack_template(ses, audio_file_path.into_string())?;

    // sequence template
    let sequence_template_vec = sequence_template()?;

    // tape template
    let sequence_tape_vec = sequence_tape(ses)?;

    // Audio scene
    let audio_scene = audio_scene(ses);
    let audio_scene_vec = cooked::isc::create_vec_with_capacity_hint(&audio_scene, 1200)?;

    bf.generated_files.add_file(
        audio_cache_dir.join(format!("{lower_map_name}_musictrack.tpl.ckd")),
        musictrack_template_vec,
    )?;

    bf.generated_files.add_file(
        audio_cache_dir.join(format!("{lower_map_name}_sequence.tpl.ckd")),
        sequence_template_vec,
    )?;

    bf.generated_files.add_file(
        audio_cache_dir.join(format!("{lower_map_name}.stape.ckd")),
        sequence_tape_vec,
    )?;

    bf.generated_files.add_file(
        audio_cache_dir.join(format!("{lower_map_name}_audio.isc.ckd")),
        audio_scene_vec,
    )?;

    Ok(cooked::isc::WrappedScene {
        scene: audio_scene.scene,
    })
}

/// Build the audio scene
fn audio_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let map_path = &ses.map_path;
    let lower_map_name = ses.lower_map_name;
    let map_name = ses.song.map_name.as_ref();
    cooked::isc::Root {
        scene: cooked::isc::Scene {
            engine_version: ses.engine_version,
            actors: vec![
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        userfriendly: Cow::Borrowed("MusicTrack"),
                        pos2d: (1.125_962, -0.418_641),
                        lua: Cow::Owned(
                            map_path
                                .join(format!("audio/{lower_map_name}_musictrack.tpl"))
                                .into_string(),
                        ),
                        components: vec![cooked::isc::WrappedComponent::MusicTrack(
                            Default::default(),
                        )],
                        ..Default::default()
                    }),
                }),
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: 0.000_001,
                        userfriendly: Cow::Owned(format!("{map_name}_sequence")),
                        pos2d: (-0.006_158, -0.006_158),
                        lua: Cow::Owned(
                            map_path
                                .join(format!("audio/{lower_map_name}_sequence.tpl"))
                                .into_string(),
                        ),
                        components: vec![cooked::isc::WrappedComponent::TapeCase(
                            Default::default(),
                        )],
                        ..Default::default()
                    }),
                }),
            ],
            ..Default::default()
        },
    }
}

/// Build the musictrack template
///
/// `audio_file_path` is the uncooked path to the file, even if the file is cooked.
fn musictrack_template(
    ses: &SongExportState<'_>,
    audio_file_path: String,
) -> Result<Vec<u8>, Error> {
    let map_name = ses.song.map_name.as_ref();

    let musictrack_file = ses
        .native_vfs
        .open(&ses.dirs.song().join("musictrack.json"))?;
    let musictrack: MusicTrack = serde_json::from_slice(&musictrack_file)?;

    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        class: None,
        wip: 0,
        lowupdate: 0,
        update_layer: 0,
        procedural: 0,
        startpaused: 0,
        forceisenvironment: 0,
        components: vec![json_types::v22::Template22::MusicTrackComponent(
            json_types::tpl::MusicTrackComponent {
                class: None,
                track_data: json_types::tpl::MusicTrackData {
                    class: Some(json_types::tpl::MusicTrackData::CLASS),
                    structure: json_types::tpl::MusicTrackStructure {
                        class: Some(json_types::tpl::MusicTrackStructure::CLASS),
                        markers: musictrack.markers.clone(),
                        signatures: musictrack.signatures.into_iter().map(Into::into).collect(),
                        sections: musictrack.sections.into_iter().map(Into::into).collect(),
                        start_beat: musictrack.start_beat,
                        end_beat: musictrack.end_beat,
                        fade_start_beat: 0,
                        use_fade_start_beat: false,
                        fade_end_beat: 0,
                        use_fade_end_beat: false,
                        video_start_time: musictrack.video_start_time,
                        preview_entry: musictrack.preview_entry,
                        preview_loop_start: musictrack.preview_loop_start,
                        preview_loop_end: musictrack.preview_loop_end,
                        volume: 0.0,
                        fade_in_duration: 0,
                        fade_in_type: 0,
                        fade_out_duration: 0,
                        fade_out_type: 0,
                        entry_points: Vec::new(),
                    },
                    path: Cow::Owned(audio_file_path),
                    url: Cow::Owned(format!("jmcs://jd-contents/{map_name}/{map_name}.ogg")),
                },
            },
        )],
    });

    Ok(cooked::json::create_vec_with_capacity_hint(
        &template, 8000,
    )?)
}

/// Build the sequence template
fn sequence_template() -> Result<Vec<u8>, Error> {
    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        components: vec![json_types::v22::Template22::MasterTape(
            json_types::tpl::MasterTape::default(),
        )],
        ..Default::default()
    });

    Ok(cooked::json::create_vec_with_capacity_hint(&template, 500)?)
}

/// Build the sequence tape
fn sequence_tape(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
    let template = json_types::v22::Template22::Tape(json_types::tape::Tape {
        class: None,
        clips: Vec::new(),
        tape_clock: 0,
        tape_bar_count: 1,
        free_resources_after_play: 0,
        map_name: ses.song.map_name.clone(),
        soundwich_event: Some(Cow::Borrowed("")),
        actor_paths: Vec::new(),
    });

    Ok(cooked::json::create_vec(&template)?)
}
