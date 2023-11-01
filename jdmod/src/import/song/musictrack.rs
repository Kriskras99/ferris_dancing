//! # Musictrack
//! Imports the main sequence and audio file
use std::{fs::File, io::Write};

use anyhow::Error;
use ubiart_toolkit::cooked;

use crate::{types::song::MusicTrack, utils::cook_path};

use super::SongImportState;

/// Imports the main sequence and audio file
pub fn import(sis: &SongImportState<'_>, musictrack_path: &str) -> Result<&'static str, Error> {
    let mainsequence_file = sis
        .vfs
        .open(cook_path(musictrack_path, sis.platform)?.as_ref())?;
    let template = cooked::json::parse_v22(&mainsequence_file)?;
    let mut actor = template.actor()?;
    assert!(
        actor.components.len() == 1,
        "More than one component in musictrack"
    );
    let track_data = actor
        .components
        .swap_remove(0)
        .musictrack_component()?
        .track_data;

    // TODO: Decook WAV!
    let audio_filename = "musictrack.wav.ckd";
    let from = sis
        .vfs
        .open(cook_path(&track_data.path, sis.platform)?.as_ref())?;
    let mut to = File::create(sis.dirs.audio().join(audio_filename))?;
    to.write_all(&from)?;

    let structure = track_data.structure;

    let musictrack = MusicTrack {
        markers: structure.markers,
        signatures: structure.signatures.into_iter().map(Into::into).collect(),
        sections: structure.sections.into_iter().map(Into::into).collect(),
        start_beat: structure.start_beat,
        end_beat: structure.end_beat,
        video_start_time: structure.video_start_time,
        preview_entry: structure.preview_entry,
        preview_loop_start: structure.preview_loop_start,
        preview_loop_end: structure.preview_loop_end,
    };

    let musictrack_path = sis.dirs.song().join("musictrack.json");

    let musictrack_file = File::create(musictrack_path)?;
    serde_json::to_writer_pretty(musictrack_file, &musictrack)?;

    Ok(audio_filename)
}
