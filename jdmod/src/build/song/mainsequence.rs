//! # Mainsequence Building
//! Build the mainsequence
use std::{borrow::Cow, fs::File};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPathBuf};
use ubiart_toolkit::{cooked, json_types, utils::SplitPath};

use super::SongExportState;
use crate::{
    build::BuildFiles,
    types::song::{Clip, Timeline},
    utils::cook_path,
};

/// Build the mainsequence
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let cache_map_path = &ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let cinematics_cache_dir = cache_map_path.join("cinematics");

    // main sequence actor
    let mainsequence_actor_vec = mainsequence_actor(ses)?;

    // main sequence template
    let mainsequence_template_vec = mainsequence_template(ses)?;

    // Cine scene
    let cine_scene = cine_scene(ses);
    let cine_scene_vec = cooked::isc::create_vec_with_capacity_hint(&cine_scene, 900)?;

    bf.generated_files.add_file(
        cinematics_cache_dir.join(format!("{lower_map_name}_mainsequence.act.ckd")),
        mainsequence_actor_vec,
    )?;

    bf.generated_files.add_file(
        cinematics_cache_dir.join(format!("{lower_map_name}_mainsequence.tpl.ckd")),
        mainsequence_template_vec,
    )?;

    bf.generated_files.add_file(
        cinematics_cache_dir.join(format!("{lower_map_name}_cine.isc.ckd")),
        cine_scene_vec,
    )?;

    // the timeline
    mainsequence_timeline(ses, bf)?;

    Ok(cooked::isc::WrappedScene {
        scene: cine_scene.scene,
    })
}

/// Build the mainsequence actor
fn mainsequence_actor(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let actor = cooked::act::Actor {
        tpl: SplitPath::new(
            Cow::Owned(format!("world/maps/{lower_map_name}/cinematics/")),
            Cow::Owned(format!("{lower_map_name}_mainsequence.tpl")),
        )?,
        unk1: 0,
        unk2: 0x3F80_0000,
        unk2_5: 0x3F80_0000,
        components: vec![cooked::act::Component::MasterTape],
    };

    Ok(cooked::act::create_vec(&actor)?)
}

/// Build the cine scene
fn cine_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let lower_map_name = ses.lower_map_name;
    let map_name = ses.song.map_name.as_ref();
    cooked::isc::Root {
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
            actors: vec![
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor { actor: cooked::isc::Actor {
                    userfriendly: Cow::Owned(format!("{map_name}_MainSequence")),
                    lua: Cow::Owned(format!("world/maps/{lower_map_name}/cinematics/{lower_map_name}_mainsequence.tpl")),
                    components: vec![cooked::isc::WrappedComponent::MasterTape],
                    ..Default::default()
                }}),
            ],
            scene_configs: cooked::isc::WrappedSceneConfigs::default(),
        },
    }
}

/// Build the mainsequence template
fn mainsequence_template(ses: &SongExportState<'_>) -> Result<Vec<u8>, Error> {
    let lower_map_name = ses.lower_map_name;
    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        class: None,
        wip: 0,
        lowupdate: 0,
        update_layer: 0,
        procedural: 0,
        startpaused: 0,
        forceisenvironment: 0,
        components: vec![json_types::v22::Template22::MasterTape(
            json_types::tpl::MasterTape {
                class: None,
                tapes_rack: vec![json_types::tpl::TapeGroup {
                    class: Some(json_types::tpl::TapeGroup::CLASS),
                    entries: vec![json_types::tpl::TapeEntry {
                        class: Some(json_types::tpl::TapeEntry::CLASS),
                        label: Cow::Borrowed("master"),
                        path: Cow::Owned(format!(
                        "world/maps/{lower_map_name}/cinematics/{lower_map_name}_mainsequence.tape"
                    )),
                    }],
                }],
            },
        )],
    });

    Ok(cooked::json::create_vec(&template)?)
}

/// Build the mainsequence timeline
fn mainsequence_timeline(ses: &SongExportState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    let timeline: Timeline =
        serde_json::from_reader(File::open(ses.dirs.song().join("mainsequence.json"))?)?;
    let lower_map_name = ses.lower_map_name;
    let cache_map_path = &ses.cache_map_path;
    let map_path = &ses.map_path;

    let mut clips = Vec::with_capacity(timeline.timeline.len());

    for orig_clip in timeline.timeline {
        let some = match orig_clip {
            Clip::HideUserInterface(_) | Clip::GameplayEvent(_) | Clip::Vibration(_) => {
                Some(orig_clip.into_tape(&ses.song)?)
            }
            Clip::SoundSet(orig_clip) => {
                let name = orig_clip.name.as_ref();
                let filename =
                    Cow::Owned(map_path.join(format!("audio/amb/{name}.wav")).to_string());
                let cooked_filename = cook_path(&filename, ses.platform)?;

                // Add amb clip to copy list
                let from = ses.dirs.audio().join(orig_clip.audio_filename.as_ref());
                let to = VirtualPathBuf::from(cooked_filename);
                let template_path =
                    Cow::Owned(map_path.join(format!("audio/amb/{name}.tpl")).to_string());

                // If the amb clip is already in the list, we skip building the template
                if !bf.static_files.exists(&to) {
                    bf.static_files.add_file(from, to)?;
                    // Create the sound template
                    let sound_descriptor = json_types::tpl::SoundDescriptor {
                        name: Cow::Borrowed(name),
                        files: vec![filename],
                        ..Default::default()
                    };
                    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
                        components: vec![json_types::v22::Template22::SoundComponent(
                            json_types::tpl::SoundComponent {
                                class: None,
                                sound_list: vec![sound_descriptor],
                            },
                        )],
                        ..Default::default()
                    });

                    // Save the template
                    let cooked_template_path = cook_path(&template_path, ses.platform)?;
                    let cooked_template_vec = cooked::json::create_vec(&template)?;
                    bf.generated_files
                        .add_file(cooked_template_path.into(), cooked_template_vec)?;
                }

                // Create the new clip with the proper template path
                let new_clip = orig_clip.to_tape(template_path);

                Some(json_types::tape::Clip::SoundSet(new_clip))
            }
            x => {
                println!("Warning! Found non-dance clip in dance_timeline, ignoring! {x:?}");
                None
            }
        };
        if let Some(clip) = some {
            clips.push(clip);
        }
    }

    let template = json_types::v22::Template22::Tape(json_types::tape::Tape {
        class: None,
        clips,
        tape_clock: 0,
        tape_bar_count: 1,
        free_resources_after_play: 0,
        map_name: Cow::Borrowed(lower_map_name),
        soundwich_event: Some(Cow::Borrowed("")),
        actor_paths: Vec::new(),
    });

    let mainsequence_tape_vec = cooked::json::create_vec(&template)?;
    bf.generated_files.add_file(
        cache_map_path.join(format!("cinematics/{lower_map_name}_mainsequence.tape.ckd")),
        mainsequence_tape_vec,
    )?;

    Ok(())
}
