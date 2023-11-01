//! # Main sequence
//! Imports the mainsequence and files referenced in it
use std::{borrow::Cow, collections::BinaryHeap, fs::File, io::Write};

use anyhow::{anyhow, Error};
use ubiart_toolkit::{cooked, json_types};

use crate::{
    types::song::{Clip, SoundSetClip, Timeline},
    utils::cook_path,
};

use super::SongImportState;

/// Imports the mainsequence and files referenced in it
pub fn import(sis: &SongImportState<'_>, mainsequence_path: &str) -> Result<(), Error> {
    let mainsequence_file = sis
        .vfs
        .open(cook_path(mainsequence_path, sis.platform)?.as_ref())?;
    let mut actor = cooked::json::parse_v22(&mainsequence_file)?.actor()?;
    assert!(
        actor.components.len() == 1,
        "More than one component in actor!"
    );
    let tape_case = actor.components.swap_remove(0).master_tape()?;

    let mainsequence_tml_path = &tape_case
        .tapes_rack
        .first()
        .ok_or_else(|| anyhow!("MainSequence Timeline Template has no Tape Groups"))?
        .entries
        .first()
        .ok_or_else(|| anyhow!("MainSequence Timeline Template has no Tape Entries"))?
        .path;
    let mainsequence_tml_path = cook_path(mainsequence_tml_path, sis.platform)?;

    let tape_file = sis.vfs.open(mainsequence_tml_path.as_ref())?;
    let tape = cooked::json::parse_v22(&tape_file)?.tape()?;

    let mut timeline = Timeline {
        timeline: BinaryHeap::with_capacity(tape.clips.len()),
    };

    for clip in tape.clips {
        let new_clip = match clip {
            json_types::Clip::HideUserInterface(hui) => Clip::HideUserInterface(hui.into()),
            json_types::Clip::SoundSet(soundset) => {
                let sound_set_path = cook_path(&soundset.sound_set_path, sis.platform)?;
                let template_file = sis.vfs.open(sound_set_path.as_ref())?;
                let template = cooked::json::parse_v22(&template_file)?.actor()?;
                let descriptor = template
                    .components
                    .first()
                    .and_then(|c| c.sound_component().ok())
                    .and_then(|c| c.sound_list.first())
                    .ok_or_else(|| anyhow!("Template is missing proper SoundDescriptor"))?;

                let name = Cow::Owned(descriptor.name.to_string());
                let filename = descriptor
                    .files
                    .first()
                    .ok_or_else(|| anyhow!("No file path in SoundDescriptor!"))?
                    .as_ref();

                let from = sis.vfs.open(cook_path(filename, sis.platform)?.as_ref())?;
                let new_filename = format!("{name}.wav.ckd");
                // TODO: Decook wav.ckd!
                let mut to = File::create(sis.dirs.audio().join(&new_filename))?;
                to.write_all(&from)?;

                Clip::SoundSet(SoundSetClip {
                    is_active: soundset.is_active == 1,
                    start_time: soundset.start_time,
                    duration: soundset.duration,
                    audio_filename: Cow::Owned(new_filename),
                    name,
                })
            }
            json_types::Clip::Vibration(vib) => Clip::Vibration(vib.into()),
            json_types::Clip::GoldEffect(goldeffect) => Clip::GoldEffect(goldeffect.into()),
            json_types::Clip::GameplayEvent(gameplay) => Clip::GameplayEvent(gameplay.into()),
            json_types::Clip::TapeReference(reference) => {
                let ref_file = sis
                    .vfs
                    .open(cook_path(&reference.path, sis.platform)?.as_ref())?;
                let ref_tape = cooked::json::parse_v22(&ref_file)?.tape()?;

                for clip in ref_tape.clips {
                    if let json_types::Clip::Vibration(mut vib) = clip {
                        vib.class = None;
                        vib.vibration_file_path = vib.vibration_file_path.clone();
                        timeline
                            .timeline
                            .push(Clip::Vibration(vib.to_owned().into()));
                    } else {
                        return Err(anyhow!(
                            "Unexpected Clip in Mainsequence Subtape ({})! {clip:?}",
                            reference.path
                        ));
                    }
                }

                // We need to return a clip, so just unpop the last one
                timeline.timeline.pop().unwrap()
            }
            _ => return Err(anyhow!("Unexpected Clip in Mainsequence Tape! {clip:?}")),
        };
        timeline.timeline.push(new_clip);
    }

    let mainsequence_path = sis.dirs.song().join("mainsequence.json");

    let mainsequence_file = File::create(mainsequence_path)?;
    serde_json::to_writer_pretty(mainsequence_file, &timeline)?;

    Ok(())
}
