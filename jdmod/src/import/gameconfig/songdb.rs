//! # Song Database
//! Imports all songs in the song database using [`song::import`]
use anyhow::Error;
use ubiart_toolkit::{
    cooked::{self, isc::WrappedComponent},
    utils::{Game, Platform},
};

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
            match (song::import(is, &actor.lua), is.lax) {
                (Ok(_), _) => {}
                (Err(error), true) => {
                    println!("Warning! Failed to import {}! Error: {error:?}", actor.userfriendly);
                }
                (Err(error), false) => return Err(error),
            }
        }
    }

    if is.game == Game::JustDance2019
        && is.platform != Platform::Wii
        && is.platform != Platform::X360
    {
        // Nice For What by Drake was removed from the songdb on 8th gen consoles
        if let Err(err) = song::import(is, "world/maps/niceforwhat/songdesc.tpl") {
            println!("Warning! Importing Nice For What by Drake failed! {err:?}");
        }
    }

    Ok(())
}
