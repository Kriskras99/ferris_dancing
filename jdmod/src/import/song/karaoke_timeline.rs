//! # Karaoke timeline
//! Imports the karaoke timeline
use std::fs::File;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::test_eq;
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::Timeline, utils::cook_path};

/// Imports the karaoke timeline
pub fn import(sis: &SongImportState<'_>, karaoke_timeline_path: &str) -> Result<(), Error> {
    let karaoke_timeline_file = sis
        .vfs
        .open(cook_path(karaoke_timeline_path, sis.ugi.platform)?.as_ref())?;
    let mut actor = cooked::json::parse_v22(&karaoke_timeline_file, sis.lax)?.actor()?;
    test_eq(&actor.components.len(), &1).context("More than one component in actor!")?;
    let tape_case = actor.components.swap_remove(0).tape_case_component()?;

    let tape_group = &tape_case.tapes_rack.first();

    // No tape group means no lyrics
    if let Some(tape_group) = tape_group {
        let karaoke_tml_path = &tape_group
            .entries
            .first()
            .ok_or_else(|| anyhow!("Karaoke Timeline Template has no Tape Entries"))?
            .path;

        let karaoke_tml_path = cook_path(karaoke_tml_path, sis.ugi.platform)?;

        let tape_file = sis.vfs.open(karaoke_tml_path.as_ref())?;
        let template = cooked::json::parse_v22(&tape_file, sis.lax)?;
        let tape = template.tape()?;

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
