//! # Timeline Building
//! Builds the karaoke and dance timelines, and pictos
use std::borrow::Cow;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{VirtualFileSystem, VirtualPathBuf};
use ubiart_toolkit::{cooked, json_types, utils::SplitPath};

use super::SongExportState;
use crate::{
    build::BuildFiles,
    types::song::{Clip, MotionClip, Timeline},
    utils::{cook_path, encode_texture},
};

/// Convenience enum for functions that can build both karaoke and dance files
#[derive(Clone, Copy)]
enum KorD {
    /// Build a karaoke file, the boolean indicates if there are lyrics
    Karaoke(bool),
    /// Build a dance file
    Dance,
}

impl KorD {
    /// Convert this to the type name
    const fn to_str(self) -> &'static str {
        match self {
            Self::Karaoke(_) => "karaoke",
            Self::Dance => "dance",
        }
    }
    /// Convert this to the label name
    const fn to_label(self) -> &'static str {
        match self {
            Self::Karaoke(_) => "karaoke",
            Self::Dance => "motion",
        }
    }

    /// Convert this to the end of the filename
    const fn to_tape_end(self) -> &'static str {
        match self {
            Self::Karaoke(_) => "karaoke.ktape",
            Self::Dance => "dance.dtape",
        }
    }
}

/// Build the dance and karaoke timelines
pub fn build(
    ses: &SongExportState<'_>,
    bf: &mut BuildFiles,
) -> Result<cooked::isc::WrappedScene<'static>, Error> {
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let timeline_cache_dir = cache_map_path.join("timeline");

    // tml scene
    let tml_scene = tml_scene(ses);
    let tml_scene_vec = cooked::isc::create_vec_with_capacity_hint(&tml_scene, 1300)?;

    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml.isc.ckd")),
        tml_scene_vec,
    )?;

    // Build the dance timeline and related files
    build_dance(ses, bf)?;

    // Build the karaoke timeline and related files
    build_karaoke(ses, bf)?;

    Ok(cooked::isc::WrappedScene {
        scene: tml_scene.scene,
    })
}

/// Build the dance timeline and pictos
fn build_dance(ses: &SongExportState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let timeline_cache_dir = cache_map_path.join("timeline");

    let timeline_file = ses
        .native_vfs
        .open(&ses.dirs.song().join("dance_timeline.json"))?;
    let timeline: Timeline = serde_json::from_slice(&timeline_file)?;

    let dance_act_vec = tml_actor(ses, KorD::Dance)?;
    let dance_tpl_vec = tml_template(ses, KorD::Dance)?;

    let mut clips = Vec::with_capacity(timeline.timeline.len());

    for orig_clip in timeline.timeline {
        let some = match orig_clip {
            Clip::GoldEffect(orig_clip) => Some(json_types::tape::Clip::GoldEffect(
                json_types::tape::GoldEffectClip::from(orig_clip),
            )),
            Clip::Motion(orig_clip) => {
                let new_clip = orig_clip.to_tape(&ses.song);

                // TODO: Check the msm file
                let from = ses
                    .dirs
                    .moves()
                    .join(orig_clip.classifier_filename.as_ref());
                // Classifier path does not include platform specifier
                let to = MotionClip::fix_classifier_path(&new_clip.classifier_path, ses.platform)?;

                if ses.native_vfs.exists(&from) {
                    bf.static_files.add_file(from, VirtualPathBuf::from(to))?;
                } else {
                    println!(
                        "Warning! Missing {} for {lower_map_name}!",
                        orig_clip.classifier_filename
                    );
                }

                Some(json_types::tape::Clip::Motion(new_clip))
            }
            Clip::Pictogram(orig_clip) => {
                let new_clip = orig_clip.to_tape(&ses.song);
                let to = cook_path(&new_clip.picto_path, ses.platform)?;

                // A picto will be used multiple times, so only create it once
                if !bf.generated_files.exists(to.as_ref()) {
                    let from = ses.dirs.pictos().join(orig_clip.picto_filename.as_ref());
                    if ses.native_vfs.exists(&from) {
                        let encoded = encode_texture(ses.native_vfs, &from)?;
                        let encoded_vec = cooked::png::create_vec(encoded)?;

                        bf.generated_files.add_file(to.into(), encoded_vec)?;
                    } else {
                        println!(
                            "Warning! Missing {} for {lower_map_name}!",
                            orig_clip.picto_filename
                        );
                    }
                }

                Some(json_types::tape::Clip::Pictogram(new_clip))
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
        map_name: ses.song.map_name.clone(),
        soundwich_event: Some(Cow::Borrowed("")),
        actor_paths: Vec::new(),
    });

    let dance_dtape_vec = cooked::json::create_vec(&template)?;

    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_dance.act.ckd")),
        dance_act_vec,
    )?;

    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_dance.tpl.ckd")),
        dance_tpl_vec,
    )?;

    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_dance.dtape.ckd")),
        dance_dtape_vec,
    )?;

    Ok(())
}

/// Build the karaoke timeline
fn build_karaoke(ses: &SongExportState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    let cache_map_path = ses.cache_map_path;
    let lower_map_name = ses.lower_map_name;
    let timeline_cache_dir = cache_map_path.join("timeline");

    let timeline_file = ses
        .native_vfs
        .open(&ses.dirs.song().join("karaoke_timeline.json"))?;
    let timeline: Timeline = serde_json::from_slice(&timeline_file)?;
    let is_empty = timeline.timeline.is_empty();
    let k = KorD::Karaoke(is_empty);

    let tml_actor_vec = tml_actor(ses, k)?;
    let tml_template_vec = tml_template(ses, k)?;

    let template = json_types::v22::Template22::Tape(json_types::tape::Tape {
        class: None,
        clips: timeline
            .timeline
            .into_iter()
            .filter(|clip| {
                if let Clip::Karaoke(_) = clip {
                    true
                } else {
                    println!("Warning! Found non-karaoke clip in karaoke_timeline, ignoring!");
                    false
                }
            })
            .map(|c| c.into_tape(&ses.song))
            .collect::<Result<_, _>>()?,
        tape_clock: 0,
        tape_bar_count: 1,
        free_resources_after_play: 0,
        map_name: ses.song.map_name.clone(),
        soundwich_event: Some(Cow::Borrowed("")),
        actor_paths: Vec::new(),
    });

    let ktape_vec = cooked::json::create_vec(&template)?;

    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_karaoke.act.ckd")),
        tml_actor_vec,
    )?;
    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_karaoke.tpl.ckd")),
        tml_template_vec,
    )?;
    bf.generated_files.add_file(
        timeline_cache_dir.join(format!("{lower_map_name}_tml_karaoke.ktape.ckd")),
        ktape_vec,
    )?;
    Ok(())
}

/// Build tml scene
fn tml_scene(ses: &SongExportState<'_>) -> cooked::isc::Root<'static> {
    let map_path = ses.map_path;
    let map_name = ses.song.map_name.as_ref();
    let lower_map_name = ses.lower_map_name;
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
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: 0.000_001,
                        userfriendly: Cow::Owned(format!("{map_name}_tml_dance")),
                        pos2d: (-1.157_74, 0.006_158),
                        lua: Cow::Owned(
                            map_path
                                .join(format!("timeline/{lower_map_name}_tml_dance.tpl"))
                                .into_string(),
                        ),
                        components: vec![cooked::isc::WrappedComponent::TapeCase],
                        ..Default::default()
                    }),
                }),
                cooked::isc::WrappedActors::Actor(cooked::isc::WrappedActor {
                    actor: Box::new(cooked::isc::Actor {
                        relativez: 0.000_001,
                        userfriendly: Cow::Owned(format!("{map_name}_tml_karaoke")),
                        pos2d: (-1.157_74, 0.006_158),
                        lua: Cow::Owned(
                            map_path
                                .join(format!("timeline/{lower_map_name}_tml_karaoke.tpl"))
                                .into_string(),
                        ),
                        components: vec![cooked::isc::WrappedComponent::TapeCase],
                        ..Default::default()
                    }),
                }),
            ],
            scene_configs: cooked::isc::WrappedSceneConfigs {
                scene_configs: cooked::isc::SceneConfigs {
                    active_scene_config: 0,
                    jd_scene_config: Vec::new(),
                },
            },
        },
    }
}

/// Build a tml actor
fn tml_actor(ses: &SongExportState<'_>, k_or_d: KorD) -> Result<Vec<u8>, Error> {
    let map_path = ses.map_path;
    let lower_map_name = ses.lower_map_name;
    let k_or_d = k_or_d.to_str();
    let actor = cooked::act::Actor {
        tpl: SplitPath::new(
            Cow::Owned(map_path.join("timeline/").into_string()),
            Cow::Owned(format!("{lower_map_name}_tml_{k_or_d}.tpl")),
        )?,
        unk1: 0,
        unk2: 0x3F80_0000,
        unk2_5: 0x3F80_0000,
        components: vec![cooked::act::Component::TapeCaseComponent],
    };

    Ok(cooked::act::create_vec(actor)?)
}

/// Build a tml template
fn tml_template(ses: &SongExportState<'_>, k_or_d: KorD) -> Result<Vec<u8>, Error> {
    // Only add a tape rack to the template if it's Dance or if it's a non-empty karaoke
    let tapes_rack = if matches!(k_or_d, KorD::Karaoke(true)) {
        Vec::new()
    } else {
        let map_path = ses.map_path;
        let lower_map_name = ses.lower_map_name;
        let label = k_or_d.to_label();
        let k_or_d = k_or_d.to_tape_end();
        vec![json_types::tpl::TapeGroup {
            class: Some(json_types::tpl::TapeGroup::CLASS),
            entries: vec![json_types::tpl::TapeEntry {
                class: Some(json_types::tpl::TapeEntry::CLASS),
                label: Cow::Owned(format!("tml_{label}")),
                path: Cow::Owned(
                    map_path
                        .join(format!("timeline/{lower_map_name}_tml_{k_or_d}"))
                        .into_string(),
                ),
            }],
        }]
    };
    let template = json_types::v22::Template22::Actor(json_types::v22::Actor22 {
        class: None,
        wip: 0,
        lowupdate: 0,
        update_layer: 0,
        procedural: 0,
        startpaused: 0,
        forceisenvironment: 0,
        components: vec![json_types::v22::Template22::TapeCase(
            json_types::tpl::MasterTape {
                class: None,
                tapes_rack,
            },
        )],
    });

    Ok(cooked::json::create_vec(&template)?)
}
