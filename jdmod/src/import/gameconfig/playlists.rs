//! # Playlists
//! Import all playlists
use std::{collections::HashMap, fs::File};

use anyhow::{anyhow, Context, Error};
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use hipstr::HipStr;
use test_eq::test_eq;
use ubiart_toolkit::cooked::{self, act::Component};

use crate::{
    types::{gameconfig::playlists::Playlist, ImportState},
    utils::{cook_path, decode_texture},
};

/// Import all playlists
pub fn import_v19v22(is: &ImportState<'_>, playlist_path: &str) -> Result<(), Error> {
    println!("Importing playlists...");

    let dir_playlists = is.dirs.playlists();
    let path_playlists_config = is.dirs.playlists().join("playlists.json");

    let playlists_file = is.vfs.open(cook_path(playlist_path, is.ugi)?.as_ref())?;
    let playlist_database =
        cooked::json::parse_v22(&playlists_file, is.lax)?.into_playlists_database()?;

    let playlists_file = std::fs::read(&path_playlists_config).unwrap_or_else(|_| vec![b'{', b'}']);
    let mut playlists: HashMap<HipStr, Playlist> = serde_json::from_slice(&playlists_file)?;

    for (name, playlist) in &playlist_database.playlists {
        // Get the playlist information
        let new_playlist = Playlist::from_offline_playlist(is, playlist)?;

        // Find the playlist cover location
        let act_file = is
            .vfs
            .open(cook_path(&playlist.cover_path, is.ugi)?.as_ref())?;
        let actor = cooked::act::Actor::deserialize_with(&act_file, is.ugi)?;
        let template = actor
            .components
            .iter()
            .find(|t| matches!(t, Component::MaterialGraphicComponent(_)))
            .ok_or_else(|| anyhow!("No MaterialGraphicComponent in actor!"))?;
        let tga_path = template.material_graphic_component()?.files[0].to_string();
        test_eq!(tga_path.is_empty(), false)?;

        // Open the cover and save it to the mod directory
        let cooked_tga_path = cook_path(&tga_path, is.ugi)?;
        let decooked_image = decode_texture(&is.vfs.open(cooked_tga_path.as_ref())?, is.ugi)
            .with_context(|| format!("Failure decoding texture {cooked_tga_path}!"))?;
        let new_cover_path = dir_playlists.join(new_playlist.cover.as_str());
        decooked_image.save(&new_cover_path)?;

        playlists.insert(HipStr::borrowed(name), new_playlist);
    }

    let file = File::create(path_playlists_config)?;
    serde_json::to_writer_pretty(file, &playlists)?;

    Ok(())
}
