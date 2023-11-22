//! # Playlists
//! Import all playlists
use std::{borrow::Cow, collections::HashMap, fs::File};

use anyhow::{anyhow, Context, Error};
use ubiart_toolkit::cooked::{self, act::TemplateType};

use crate::{
    types::{gameconfig::playlists::Playlist, ImportState},
    utils::{cook_path, decode_texture},
};

/// Import all playlists
pub fn import_v19v22(is: &ImportState<'_>, playlist_path: &str) -> Result<(), Error> {
    println!("Importing playlists...");

    let dir_playlists = is.dirs.playlists();
    let path_playlists_config = is.dirs.playlists().join("playlists.json");

    let mut playlists: HashMap<Cow<'_, str>, Playlist> =
        if let Ok(file) = File::open(&path_playlists_config) {
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

    let playlists_file = is
        .vfs
        .open(cook_path(playlist_path, is.platform)?.as_ref())?;
    let parsed_json = cooked::json::parse_v22(&playlists_file, is.lax)?;
    let playlist_database = parsed_json.playlists_database()?;

    for (name, playlist) in &playlist_database.playlists {
        // Get the playlist information
        let new_playlist = Playlist::from_offline_playlist(is, playlist)?;

        // Find the playlist cover location
        let act_file = is
            .vfs
            .open(cook_path(&playlist.cover_path, is.platform)?.as_ref())?;
        let actor = cooked::act::parse(&act_file, is.game)?;
        let template = actor
            .templates
            .iter()
            .find(|t| t.the_type == TemplateType::MaterialGraphicComponent)
            .ok_or_else(|| anyhow!("No MaterialGraphicComponent in actor!"))?;
        let tga_path = template.data.material_graphics_component()?.files[0]
            .as_ref()
            .ok_or_else(|| anyhow!("No TGA path at index 0!"))?
            .to_string();

        // Open the cover and save it to the mod directory
        let cooked_tga_path = cook_path(&tga_path, is.platform)?;
        let decooked_image = decode_texture(&is.vfs.open(cooked_tga_path.as_ref())?)
            .with_context(|| format!("Failure decoding texture {cooked_tga_path}!"))?;
        let new_cover_path = dir_playlists.join(new_playlist.cover.as_ref());
        decooked_image.save(&new_cover_path)?;

        playlists.insert(Cow::Borrowed(name), new_playlist);
    }

    let file = File::create(path_playlists_config)?;
    serde_json::to_writer_pretty(file, &playlists)?;

    Ok(())
}
