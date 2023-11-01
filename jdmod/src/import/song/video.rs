//! # Video
//! Import the video of the song
use std::{fs::File, io::Write};

use anyhow::{anyhow, Error};
use ubiart_toolkit::cooked;

use super::SongImportState;

/// Imports the video of the song
pub fn import(
    sis: &SongImportState<'_>,
    video_actor: &cooked::isc::Actor<'_>,
) -> Result<&'static str, Error> {
    let pleo = video_actor
        .components
        .first()
        .ok_or_else(|| anyhow!("No components in video actor"))?
        .pleo_component()?;

    let filename = "main_video.webm";
    let to_path = sis.dirs.song().join(filename);
    let mut to = File::create(to_path)?;

    let video_path = pleo.video.as_ref();

    if let Ok(from) = sis.vfs.open(video_path.as_ref()) {
        to.write_all(&from)?;
    } else {
        let index = video_path
            .rfind('.')
            .ok_or_else(|| anyhow!("Malformed video path!"))?;
        let (left, right) = video_path.split_at(index);
        let mut video_path = String::with_capacity(video_path.len() + 8);
        video_path.push_str(left);
        video_path.push_str(".vp9.720");
        video_path.push_str(right);
        to.write_all(&sis.vfs.open(video_path.as_ref())?)?;
    }

    Ok(filename)
}
