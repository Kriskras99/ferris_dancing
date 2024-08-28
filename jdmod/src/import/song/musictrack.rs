//! # Musictrack
//! Imports the main sequence and audio file
use std::{fs::File, io::Write};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::{test_eq, vfs::VirtualPath};
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{
    types::song::MusicTrack,
    utils::{self, cook_path},
};

/// Imports the main sequence and audio file
pub fn import(sis: &SongImportState<'_>, musictrack_path: &str) -> Result<String, Error> {
    let mainsequence_file = sis
        .vfs
        .open(cook_path(musictrack_path, sis.ugi.platform)?.as_ref())?;
    let template = cooked::json::parse_v22(&mainsequence_file, sis.lax)?;
    let mut actor = template.into_actor()?;
    test_eq!(actor.components.len(), 1)?;
    let track_data = actor
        .components
        .swap_remove(0)
        .into_musictrack_component()?
        .track_data;

    let path = track_data.path.as_ref();

    // TODO: Decook WAV!
    let audio_filename = if sis.vfs.exists(path.as_ref()) {
        let audio_filename = VirtualPath::new(path)
            .file_name()
            .ok_or_else(|| anyhow!("Can't find filename! {path:?}"))?
            .to_string();
        let from = sis.vfs.open(path.as_ref())?;
        let mut to = File::create(sis.dirs.audio().join(&audio_filename))?;
        to.write_all(&from)?;
        audio_filename
    } else {
        let cooked_path = cook_path(path, sis.ugi.platform)?;
        let mut audio_filename = VirtualPath::new(&cooked_path)
            .file_name()
            .ok_or_else(|| anyhow!("Can't find filename! {cooked_path:?}"))?
            .to_string();
        assert!(
            audio_filename.ends_with(".wav.ckd"),
            "audio filename does not end in wav.ckd?"
        );
        if audio_filename.ends_with(".ckd") {
            audio_filename.truncate(audio_filename.len() - 4);
        }
        let from = sis.vfs.open(cooked_path.as_ref())?;
        let filename = sis.dirs.audio().join(&audio_filename);
        let mut to = File::create(&filename)?;
        let is_opus = utils::decode_audio(&from, &mut to)?;
        if is_opus {
            std::fs::rename(&filename, filename.with_extension("opus"))?;
            audio_filename.truncate(audio_filename.len() - 4);
            audio_filename.push_str(".opus");
        }
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
