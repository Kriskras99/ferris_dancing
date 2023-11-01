//! # Song Database
//! Imports all songs in the song database using [`song::import`]
use anyhow::Error;
use ubiart_toolkit::cooked::{self, isc::WrappedComponent};

use crate::{import::song, types::ImportState, utils::cook_path};

/// Imports all songs in the song database using [`song::import`]
pub fn import(is: &ImportState<'_>, songdb_scene: &str) -> Result<(), Error> {
    println!("Importing songs...");
    let songdb_scene_file = is
        .vfs
        .open(cook_path(songdb_scene, is.platform)?.as_ref())?;
    let songdb_scene = cooked::isc::parse(&songdb_scene_file)?;

    for actors in songdb_scene.scene.actors {
        let actor = actors.actor()?;
        if actor
            .components
            .iter()
            .any(|c| matches!(c, WrappedComponent::SongDesc))
        {
            song::import(is, &actor.lua)?;
        }
    }

    Ok(())
}
