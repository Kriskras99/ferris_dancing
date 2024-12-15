//! # Karaoke timeline
//! Imports the karaoke timeline
use std::{fs::File, io::ErrorKind};

use anyhow::{anyhow, Error};
use test_eq::test_eq;
use tracing::{trace, warn};
use ubiart_toolkit::cooked;

use super::SongImportState;
use crate::{types::song::Timeline, utils::cook_path};

/// Imports the karaoke timeline
pub fn import(sis: &SongImportState<'_>, karaoke_timeline_path: &str) -> Result<(), Error> {
    let karaoke_timeline_file = match (
        sis.vfs
            .open(cook_path(karaoke_timeline_path, sis.ugi)?.as_ref()),
        sis.lax,
    ) {
        (Ok(file), _) => file,
        (Err(err), true) if err.kind() == ErrorKind::NotFound => {
            warn!("Failed to import lyrics, file not found");
            trace!("{err}");
            return Ok(());
        }
        (Err(err), _) => return Err(err.into()),
    };
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

        let tape_file = match (sis.vfs.open(karaoke_tml_path.as_ref()), sis.lax) {
            (Ok(file), _) => file,
            (Err(err), true) if err.kind() == std::io::ErrorKind::NotFound => {
                warn!("Failed to import lyrics, file not found");
                trace!("{err}");
                let karaoke_timeline_path = sis.dirs.song().join("karaoke_timeline.json");
                let timeline_file = File::create(karaoke_timeline_path)?;
                serde_json::to_writer_pretty(timeline_file, &Timeline::default())?;
                return Ok(());
            }
            (err, _) => err?,
        };
        let tape = cooked::tape::parse(&tape_file, sis.ugi, sis.lax)?;

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
