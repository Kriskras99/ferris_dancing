//! # Autodance
//! Imports the autodance settings and preview audio file
use std::{fs::File, io::Write};

use anyhow::Error;
use hipstr::HipStr;
use test_eq::test_eq;
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::Autodance, utils::cook_path};

/// Imports the autodance settings and preview audio file
pub fn import(sis: &SongImportState<'_>, autodance_path: &str) -> Result<(), Error> {
    let autodance_file = sis.vfs.open(cook_path(autodance_path, sis.ugi)?.as_ref())?;
    let mut actor = cooked::tpl::parse(&autodance_file, sis.ugi, sis.lax)?;
    test_eq!(actor.components.len(), 1)?;
    let autodance = actor.components.swap_remove(0).into_autodance_component()?;

    let data = &autodance.autodance_data;
    let audiofile_rel_path = "autodance.ogg";
    let audiofile_path = sis.dirs.audio().join(audiofile_rel_path);
    if let Ok(audiofile) = sis.vfs.open(data.autodance_sound_path.as_str().as_ref()) {
        let mut new_audiofile = File::create(audiofile_path)?;
        new_audiofile.write_all(&audiofile)?;
    } else {
        println!("Warning! {} not found!", data.autodance_sound_path);
    }

    let autodance = Autodance {
        record: data
            .recording_structure
            .records
            .iter()
            .map(Into::into)
            .collect(),
        song_start_position: data.video_structure.song_start_position,
        autodance_sound: HipStr::borrowed(audiofile_rel_path),
        duration: data.video_structure.duration,
        playback_events: data
            .video_structure
            .playback_events
            .iter()
            .map(Into::into)
            .collect(),
    };

    let autodance_path = sis.dirs.song().join("autodance.json");
    let autodance_file = File::create(autodance_path)?;
    serde_json::to_writer_pretty(autodance_file, &autodance)?;

    Ok(())
}
