//! # Karaoke timeline
//! Imports the karaoke timeline
use std::fs::File;

use anyhow::{anyhow, Error};
use test_eq::test_eq;
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::Timeline, utils::cook_path};

/// Imports the karaoke timeline
pub fn import(sis: &SongImportState<'_>, karaoke_timeline_path: &str) -> Result<(), Error> {
    let karaoke_timeline_file = sis
        .vfs
        .open(cook_path(karaoke_timeline_path, sis.ugi)?.as_ref())?;
    let mut actor = cooked::tpl::parse(&karaoke_timeline_file, sis.ugi, sis.lax)?;
    test_eq!(actor.components.len(), 1)?;
    let tape_case = actor.components.remove(0).into_tape_case_component()?;

    let tape_group = &tape_case.tapes_rack.first();

    // No tape group means no lyrics
    if let Some(tape_group) = tape_group {
        let karaoke_tml_path = &tape_group
            .entries
            .first()
            .ok_or_else(|| anyhow!("Karaoke Timeline Template has no Tape Entries"))?
            .path;

        let karaoke_tml_path = cook_path(karaoke_tml_path, sis.ugi)?;

        let tape_file = sis.vfs.open(karaoke_tml_path.as_ref())?;
        let tape = cooked::tape::parse(&tape_file, sis.ugi)?;

        let timeline = Timeline {
            timeline: tape
                .clips
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        };

        let karaoke_timeline_path = sis.dirs.song().join("karaoke_timeline.json");
        let timeline_file = File::create(karaoke_timeline_path)?;
        serde_json::to_writer_pretty(timeline_file, &timeline)?;
    } else {
        let karaoke_timeline_path = sis.dirs.song().join("karaoke_timeline.json");
        let timeline_file = File::create(karaoke_timeline_path)?;
        serde_json::to_writer_pretty(timeline_file, &Timeline::default())?;
    };

    Ok(())
}
