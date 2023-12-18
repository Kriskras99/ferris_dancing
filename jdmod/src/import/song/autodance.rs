//! # Autodance
//! Imports the autodance settings and preview audio file
use std::{borrow::Cow, fs::File, io::Write};

use anyhow::Error;
use dotstar_toolkit_utils::testing::test;
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::Autodance, utils::cook_path};

/// Imports the autodance settings and preview audio file
pub fn import(sis: &SongImportState<'_>, autodance_path: &str) -> Result<(), Error> {
    let autodance_file = sis
        .vfs
        .open(cook_path(autodance_path, sis.platform)?.as_ref())?;
    let mut actor = cooked::json::parse_v22(&autodance_file, sis.lax)?.actor()?;
    test(&actor.components.len(), &1).context("More than one component in actor!")?;
    let autodance = actor.components.swap_remove(0).autodance_component()?;

    let data = &autodance.autodance_data;
    let audiofile_rel_path = "autodance.ogg";
    let audiofile_path = sis.dirs.audio().join(audiofile_rel_path);
    if let Ok(audiofile) = sis.vfs.open(data.autodance_sound_path.as_ref().as_ref()) {
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
        autodance_sound: Cow::Borrowed(audiofile_rel_path),
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
