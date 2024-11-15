//! # Musictrack
//! Imports the main sequence and audio file
use std::{fs::File, io::Write};

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::vfs::VirtualPath;
use test_eq::test_eq;
use tracing::debug;
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
        .open(cook_path(musictrack_path, sis.ugi)?.as_ref())?;
    let mut actor = cooked::tpl::parse(&mainsequence_file, sis.ugi, sis.lax)?;
    test_eq!(actor.components.len(), 1)?;
    let track_data = actor
        .components
        .remove(0)
        .into_musictrack_component()?
        .track_data;

    let path = track_data.path.as_str();

    let audio_filename = if sis.vfs.exists(path.as_ref()) {
        let audio_filename = VirtualPath::new(path)
            .file_name()
            .ok_or_else(|| anyhow!("Can't find filename! {path:?}"))?
            .to_string();
        let from = sis.vfs.open(path.as_ref())?;
        let mut to = File::create(sis.dirs.audio().join(&audio_filename))?;
        to.write_all(&from)?;
        audio_filename
    } else if path.ends_with(".ogg") {
        if sis.lax {
            let alt_path = format!(
                "world/maps/{}/audio/{}.ogg",
                sis.lower_map_name, sis.lower_map_name
            );
            debug!("{path} not found, trying {alt_path}");
            let Ok(from) = sis.vfs.open(alt_path.as_ref()) else {
                return Err(anyhow!("{path} does not exist!"));
            };
            let audio_filename = format!("{}.ogg", sis.lower_map_name);
            let mut to = File::create(sis.dirs.audio().join(&audio_filename))?;
            to.write_all(&from)?;
            audio_filename
        } else {
            return Err(anyhow!("{path} does not exist!"));
        }
    } else {
        let cooked_path = cook_path(path, sis.ugi)?;
        let mut audio_filename = VirtualPath::new(&cooked_path)
            .file_name()
            .ok_or_else(|| anyhow!("Can't find filename! {cooked_path:?}"))?
            .to_string();
        test_eq!(
            audio_filename.ends_with(".wav.ckd"),
            true,
            "audio filename does not end in wav.ckd?"
        )?;
        if audio_filename.ends_with(".ckd") {
            audio_filename.truncate(audio_filename.len() - 4);
        }
        let from = sis.vfs.open(cooked_path.as_ref())?;
        let filename = sis.dirs.audio().join(&audio_filename);
        let mut to = File::create(&filename)?;
        let is_opus = utils::decode_audio(&from, &mut to, sis.lax)?;
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
