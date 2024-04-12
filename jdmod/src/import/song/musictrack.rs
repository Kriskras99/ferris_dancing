//! # Musictrack
//! Imports the main sequence and audio file
use std::{fs::File, io::Write};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::test_eq;
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::MusicTrack, utils::cook_path};

/// Imports the main sequence and audio file
pub fn import(sis: &SongImportState<'_>, musictrack_path: &str) -> Result<String, Error> {
    let mainsequence_file = sis
        .vfs
        .open(cook_path(musictrack_path, sis.ugi.platform)?.as_ref())?;
    let template = cooked::json::parse_v22(&mainsequence_file, sis.lax)?;
    let mut actor = template.into_actor()?;
    test_eq(&actor.components.len(), &1).context("More than one component in muisctrack")?;
    let track_data = actor
        .components
        .swap_remove(0)
        .into_musictrack_component()?
        .track_data;

    let path = track_data.path.as_ref();

    // TODO: Decook WAV!
    let audio_filename = if sis.vfs.exists(path.as_ref()) {
        let audio_filename = path
            .rsplit_once('/')
            .map(|p| p.1.to_string())
            .ok_or_else(|| anyhow!("Invalid path! {path:?}"))?;
        let from = sis.vfs.open(path.as_ref())?;
        let mut to = File::create(sis.dirs.audio().join(&audio_filename))?;
        to.write_all(&from)?;
        audio_filename
    } else {
        let cooked_path = cook_path(path, sis.ugi.platform)?;
        let audio_filename = cooked_path
            .rsplit_once('/')
            .map(|p| p.1.to_string())
            .ok_or_else(|| anyhow!("Invalid cooked path! {cooked_path:?}"))?;
        let from = sis.vfs.open(cooked_path.as_ref())?;
        let mut to = File::create(sis.dirs.audio().join(&audio_filename))?;
        to.write_all(&from)?;
        audio_filename
    };

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
