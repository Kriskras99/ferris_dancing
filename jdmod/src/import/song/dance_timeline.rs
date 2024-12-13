//! # Dance Timeline
//! Imports the dance timeline, pictos, and classifiers
use std::{collections::BTreeSet, fs::File, io::Write};

use anyhow::{anyhow, Error};
use test_eq::test_eq;
use ubiart_toolkit::{cooked, cooked::tape};

use super::{montage, SongImportState};
use crate::{
    types::song::{Clip, MotionClip, PictogramClip, Timeline},
    utils::{cook_path, decode_texture},
};

/// Imports the dance timeline, pictos, and classifiers
pub fn import(sis: &SongImportState<'_>, dance_timeline_path: &str) -> Result<(), Error> {
    let dance_timeline_file = sis
        .vfs
        .open(cook_path(dance_timeline_path, sis.ugi)?.as_ref())?;
    let mut actor = cooked::tpl::parse(&dance_timeline_file, sis.ugi, sis.lax)?;
    test_eq!(actor.components.len(), 1)?;
    let tape_case = actor.components.remove(0).into_tape_case_component()?;
    let tape_case_path = tape_case
        .tapes_rack
        .first()
        .and_then(|t| t.entries.first())
        .ok_or_else(|| anyhow!("Incomplete tapes rack!"))?
        .path
        .as_ref();
    let dance_tml_path = cook_path(tape_case_path, sis.ugi)?;

    let tape_file = sis.vfs.open(dance_tml_path.as_ref())?;
    let tape = tape::parse(&tape_file, sis.ugi)?;

    let mut timeline = Timeline {
        timeline: BTreeSet::new(),
    };

    let montage_path = cook_path(
        &format!(
            "world/maps/{}/timeline/pictos/montage.png",
            sis.lower_map_name
        ),
        sis.ugi,
    )?;

    let mut montage_vec = sis.vfs.exists(montage_path.as_ref()).then(Vec::new);

    for clip in tape.clips {
        let new_clip = match clip {
            tape::Clip::GoldEffect(goldeffect) => Clip::GoldEffect(goldeffect.into()),
            tape::Clip::Motion(motion) => {
                let classifier_path = motion.classifier_path.clone();
                let new_motion: MotionClip = motion.try_into()?;

                // Classifier path does not include platform specifier
                let classifier_path =
                    MotionClip::fix_classifier_path(&classifier_path, sis.ugi.platform)?;

                // Save the classifier
                if let Ok(from) = sis.vfs.open(classifier_path.as_ref()) {
                    let mut to = File::create(
                        sis.dirs
                            .moves()
                            .join(new_motion.classifier_filename.as_str()),
                    )?;
                    to.write_all(&from)?;
                } else {
                    println!("Warning! Missing classifier {classifier_path}!");
                }
                Clip::Motion(new_motion)
            }
            tape::Clip::Pictogram(pictogram) => {
                let picto_path = pictogram.picto_path.clone();
                let new_picto: PictogramClip = pictogram.try_into()?;
                if let Some(ref mut vec) = montage_vec {
                    if !vec.contains(&new_picto.picto_filename) {
                        vec.push(new_picto.picto_filename.clone());
                    }
                } else {
                    let cooked_path = cook_path(&picto_path, sis.ugi)?;
                    match (sis.vfs.open(cooked_path.as_ref()), sis.lax) {
                        (Ok(from), _) => {
                            let decooked_picto = decode_texture(&from, sis.ugi)?;
                            let path = sis.dirs.pictos().join(new_picto.picto_filename.as_str());
                            decooked_picto.save(path)?;
                        }
                        (Err(error), true) => println!("Warning! {error}"),
                        (Err(error), false) => return Err(error.into()),
                    };
                }
                Clip::Pictogram(new_picto)
            }
            _ => return Err(anyhow!("Unexpected Clip in Dance Timeline Tape! {clip:?}")),
        };
        timeline.timeline.insert(new_clip);
    }

    if let Some(mut vec) = montage_vec {
        vec.sort();
        let vec: Vec<_> = vec.iter().map(AsRef::as_ref).collect();
        montage::import(sis, &montage_path, &vec)?;
    }

    let dance_timeline_path = sis.dirs.song().join("dance_timeline.json");

    let timeline_file = File::create(dance_timeline_path)?;
    serde_json::to_writer_pretty(timeline_file, &timeline)?;

    Ok(())
}
