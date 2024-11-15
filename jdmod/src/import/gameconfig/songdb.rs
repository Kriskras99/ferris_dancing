//! # Song Database
//! Imports all songs in the song database using [`song::import`]
use anyhow::Error;
use crossbeam::channel::TryRecvError;
use ubiart_toolkit::{
    cooked::{self, isc::WrappedComponent},
    utils::{Game, Platform},
};

use crate::{import::song, types::ImportState, utils::cook_path};

/// Imports all songs in the song database using [`song::import`]
///
/// # Panics
/// Will panic if one of the import threads panic. The import threads will panic
/// if the song is wrong and lax is disabled.
/// It will also panic if the channel is poisoned
pub fn import(is: &ImportState<'_>, songdb_scene: &str) -> Result<(), Error> {
    println!("Importing songs...");
    let songdb_scene_file = is.vfs.open(cook_path(songdb_scene, is.ugi)?.as_ref())?;
    let songdb_scene = cooked::isc::parse(&songdb_scene_file, is.ugi)?;

    let n_threads = if let Some(n_threads) = is.n_threads {
        usize::from(n_threads)
    } else {
        usize::from(std::thread::available_parallelism()?)
    };

    // Only start as many threads as there are cpus (excluding the main thread, which will be waiting and doing nothing the entire time)
    std::thread::scope(|s| {
        let (tx_job, rx_job) = crossbeam::channel::unbounded::<(&ImportState, &str)>();

        for actors in &songdb_scene.scene.actors {
            let actor = actors.actor().unwrap();
            if actor
                .components
                .iter()
                .any(|c| matches!(c, WrappedComponent::SongDesc(_)))
            {
                tx_job.send((is, &actor.lua)).unwrap();
            }
        }

        if is.ugi.game == Game::JustDance2019
            && is.ugi.platform != Platform::Wii
            && is.ugi.platform != Platform::X360
        {
            // Nice For What by Drake was removed from the songdb on 8th gen consoles
            tx_job
                .send((is, "world/maps/niceforwhat/songdesc.tpl"))
                .unwrap();
        }

        for i in 0..n_threads {
            let rx_job = rx_job.clone();
            std::thread::Builder::new()
                .name(format!("Songs ({i})"))
                .spawn_scoped(s, move || loop {
                    match rx_job.try_recv() {
                        Ok((is, lua)) => song::import(is, lua).unwrap(),
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => {
                            break;
                        }
                    }
                })
                .unwrap();
        }
    });

    Ok(())
}
