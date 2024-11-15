//! # Playlists
//! Types for playlists
use std::path::Path;

use anyhow::{anyhow, Error};
use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::cooked;

use crate::types::{localisation::LocaleId, ImportState};

/// Describes a playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist<'a> {
    /// Title of the playlist
    pub title: LocaleId,
    /// Description of the playlist
    pub description: LocaleId,
    /// Path to the cover image
    #[serde(borrow)]
    pub cover: HipStr<'a>,
    /// Codenames of the maps in the playlist
    #[serde(borrow)]
    pub maps: Vec<HipStr<'a>>,
}

impl<'a> Playlist<'a> {
    /// Convert from the UbiArt representation
    ///
    /// # Errors
    /// Will error if the parsing of the filename fails or if the game is not supported
    pub fn from_offline_playlist(
        is: &ImportState<'_>,
        offline_playlist: &cooked::json::OfflinePlaylist<'a>,
    ) -> Result<Self, Error> {
        let file_stem = AsRef::<Path>::as_ref(offline_playlist.cover_path.as_str())
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        let cover = if file_stem.starts_with("jd") {
            HipStr::from(format!("{file_stem}.png"))
        } else {
            match is.ugi.game {
                ubiart_toolkit::utils::Game::JustDance2014 => {
                    HipStr::from(format!("jd14_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2015 => {
                    HipStr::from(format!("jd15_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2016 => {
                    HipStr::from(format!("jd16_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2017 => {
                    HipStr::from(format!("jd17_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2018 => {
                    HipStr::from(format!("jd18_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2019 => {
                    HipStr::from(format!("jd19_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2020 => {
                    HipStr::from(format!("jd20_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDanceChina => {
                    HipStr::from(format!("jdc_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2021 => {
                    HipStr::from(format!("jd21_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2022 => {
                    HipStr::from(format!("jd22_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::Unknown => {
                    return Err(anyhow!(
                        "Unknown game! Cannot properly name playlist cover!"
                    ))
                }
            }
        };
        Ok(Self {
            title: is
                .locale_id_map
                .get(offline_playlist.title_id)
                .unwrap_or_default(),
            description: is
                .locale_id_map
                .get(offline_playlist.description_id)
                .unwrap_or_default(),
            cover,
            maps: offline_playlist.maps.clone(),
        })
    }

    /// Convert to the UbiArt representation
    ///
    /// # Errors
    /// Will error if parsing the filename fails
    pub fn into_offline_playlist(self) -> Result<cooked::json::OfflinePlaylist<'a>, Error> {
        let file_stem = AsRef::<Path>::as_ref(self.cover.as_str())
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        Ok(cooked::json::OfflinePlaylist {
            class: Some(cooked::json::OfflinePlaylist::CLASS),
            title_id: self.title,
            description_id: self.description,
            cover_path: HipStr::from(format!(
                "world/ui/textures/covers/playlists_offline/{file_stem}.act"
            )),
            maps: self.maps,
        })
    }
}
