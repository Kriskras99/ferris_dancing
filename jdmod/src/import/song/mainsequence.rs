//! # Main sequence
//! Imports the mainsequence and files referenced in it
use std::{borrow::Cow, collections::BTreeSet, fs::File};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::test_eq;
use ubiart_toolkit::{cooked, cooked::tape, json_types, utils::Game};

use super::SongImportState;
use crate::{
    types::song::{Clip, SoundSetClip, Timeline},
    utils::{self, cook_path},
};

/// Imports the mainsequence and files referenced in it
pub fn import(sis: &SongImportState<'_>, mainsequence_path: &str) -> Result<(), Error> {
    let mainsequence_file = sis
        .vfs
        .open(cook_path(mainsequence_path, sis.ugi.platform)?.as_ref())?;
    let mut actor = cooked::json::parse_v22(&mainsequence_file, sis.lax)?.into_actor()?;
    test_eq!(actor.components.len(), 1)?;
    let tape_case = actor.components.swap_remove(0).into_master_tape()?;

    let mainsequence_tml_path_option = tape_case
        .tapes_rack
        .first()
        .and_then(|t| t.entries.first())
        .map(|t| t.path.as_ref());
    let lax = sis.lax || sis.ugi.game == Game::JustDance2016;
    let mainsequence_tml_path = match (mainsequence_tml_path_option, lax) {
        (Some(mainsequence_tml_path), _) => Cow::Borrowed(mainsequence_tml_path),
        (None, true) => {
            if sis.ugi.game != Game::JustDance2016 {
                println!("Warning! MainSequence Timeline Template is empty! Guessing name!");
            }
            Cow::Owned(mainsequence_path.replace(".tpl", ".tape"))
        }
        (None, false) => return Err(anyhow!("MainSequence Timeline Template is empty!")),
    };
    let mainsequence_tml_path = cook_path(&mainsequence_tml_path, sis.ugi.platform)?;

    match (sis.vfs.open(mainsequence_tml_path.as_ref()), sis.lax) {
        (Ok(tape_file), _) => {
            let tape = cooked::tape::parse(&tape_file, sis.ugi)?;

            let mut timeline = Timeline {
                timeline: BTreeSet::new(),
            };

            for clip in tape.clips {
                let new_clip = match clip {
                    tape::Clip::HideUserInterface(hui) => Clip::HideUserInterface(hui.into()),
                    tape::Clip::SoundSet(soundset) => {
                        match (parse_soundset(sis, &soundset), sis.lax) {
                            (Ok(clip), _) => clip,
                            (Err(error), true) => {
                                println!("Warning! {error}");
                                continue;
                            }
                            (Err(error), false) => return Err(error),
                        }
                    }
                    tape::Clip::Vibration(vib) => Clip::Vibration(vib.into()),
                    tape::Clip::GoldEffect(goldeffect) => Clip::GoldEffect(goldeffect.into()),
                    tape::Clip::GameplayEvent(gameplay) => Clip::GameplayEvent(gameplay.into()),
                    tape::Clip::TapeReference(reference) => {
                        let ref_file = sis
                            .vfs
                            .open(cook_path(&reference.path, sis.ugi.platform)?.as_ref())?;
                        let ref_tape = cooked::tape::parse(&ref_file, sis.ugi)?;

                        for clip in ref_tape.clips {
                            match clip {
                                tape::Clip::SoundSet(soundset) => {
                                    match (parse_soundset(sis, &soundset), sis.lax) {
                                        (Ok(clip), _) => {
                                            timeline.timeline.insert(clip);
                                        }
                                        (Err(error), true) => {
                                            println!("Warning! {error}");
                                            continue;
                                        }
                                        (Err(error), false) => return Err(error),
                                    }
                                }
                                tape::Clip::Vibration(vib) => {
                                    timeline
                                        .timeline
                                        .insert(Clip::Vibration(vib.to_owned().into()));
                                }
                                _ => {
                                    return Err(anyhow!(
                                        "Unexpected Clip in Mainsequence Subtape ({})! {clip:?}",
                                        reference.path
                                    ));
                                }
                            }
                        }

                        // No clip to return, so just continue to next loop
                        continue;
                    }
                    _ => return Err(anyhow!("Unexpected Clip in Mainsequence Tape! {clip:?}")),
                };
                timeline.timeline.insert(new_clip);
            }
            let mainsequence_path = sis.dirs.song().join("mainsequence.json");

            let mainsequence_file = File::create(mainsequence_path)?;
            serde_json::to_writer_pretty(mainsequence_file, &timeline)?;
        }
        (Err(error), true) => {
            println!("Warning! {error}");
            let timeline = Timeline {
                timeline: BTreeSet::new(),
            };

            let mainsequence_path = sis.dirs.song().join("mainsequence.json");

            let mainsequence_file = File::create(mainsequence_path)?;
            serde_json::to_writer_pretty(mainsequence_file, &timeline)?;
        }
        (Err(error), false) => return Err(error.into()),
    };

    Ok(())
}

/// Parse a `SoundSetClip`
pub fn parse_soundset(
    sis: &SongImportState<'_>,
    soundset: &tape::SoundSetClip,
) -> Result<Clip<'static>, Error> {
    let sound_set_path = cook_path(&soundset.sound_set_path, sis.ugi.platform)?;
    let template_file = sis.vfs.open(sound_set_path.as_ref())?;
    let template = cooked::json::parse_v22(&template_file, sis.lax)?.into_actor()?;
    let descriptor = template
        .components
        .first()
        .and_then(|c| c.into_sound_component().ok())
        .and_then(|c| c.sound_list.first())
        .ok_or_else(|| anyhow!("Template is missing proper SoundDescriptor"))?;

    let name = Cow::Owned(descriptor.name.to_string());
    let filename = descriptor
        .files
        .first()
        .ok_or_else(|| anyhow!("No file path in SoundDescriptor!"))?
        .as_ref();

    let cooked_path = cook_path(filename, sis.ugi.platform)?;
    let from = sis.vfs.open(cooked_path.as_ref())?;
    let mut new_filename = format!("{name}.wav");
    let filename = sis.dirs.audio().join(&new_filename);

    let mut to = File::create(&filename)?;
    let is_opus = utils::decode_audio(&from, &mut to)?;
    if is_opus {
        std::fs::rename(&filename, filename.with_extension("opus"))?;
        new_filename = format!("{name}.opus");
    }

    Ok(Clip::SoundSet(SoundSetClip {
        is_active: soundset.is_active == 1,
        start_time: soundset.start_time,
        duration: soundset.duration,
        audio_filename: Cow::Owned(new_filename),
        name,
    }))
}
