//! # Playlists
//! Types for playlists
use std::{borrow::Cow, path::Path};

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use ubiart_toolkit::json_types;

use crate::types::{localisation::LocaleId, ImportState};

/// Describes a playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist<'a> {
    /// Title of the playlist
    pub title: LocaleId,
    /// Description of the playlist
    pub description: LocaleId,
    /// Path to the cover image
    pub cover: Cow<'a, str>,
    /// Codenames of the maps in the playlist
    pub maps: Vec<Cow<'a, str>>,
}

impl<'a> Playlist<'a> {
    /// Convert from the UbiArt representation
    ///
    /// # Errors
    /// Will error if the parsing of the filename fails or if the game is not supported
    pub fn from_offline_playlist(
        is: &ImportState<'_>,
        offline_playlist: &json_types::isg::OfflinePlaylist<'a>,
    ) -> Result<Self, Error> {
        let file_stem = AsRef::<Path>::as_ref(offline_playlist.cover_path.as_ref())
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        let cover = if file_stem.starts_with("jd") {
            Cow::Owned(format!("{file_stem}.png"))
        } else {
            match is.game {
                ubiart_toolkit::utils::Game::JustDance2014 => {
                    Cow::Owned(format!("jd14_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2015 => {
                    Cow::Owned(format!("jd15_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2016 => {
                    Cow::Owned(format!("jd16_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2017 => {
                    Cow::Owned(format!("jd17_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2018 => {
                    Cow::Owned(format!("jd18_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2019 => {
                    Cow::Owned(format!("jd19_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2020 => {
                    Cow::Owned(format!("jd20_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDanceChina => {
                    Cow::Owned(format!("jdc_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2021 => {
                    Cow::Owned(format!("jd21_{file_stem}.png"))
                }
                ubiart_toolkit::utils::Game::JustDance2022 => {
                    Cow::Owned(format!("jd22_{file_stem}.png"))
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
    pub fn to_offline_playlist(self) -> Result<json_types::isg::OfflinePlaylist<'a>, Error> {
        let file_stem = AsRef::<Path>::as_ref(self.cover.as_ref())
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| anyhow!("Failure parsing filename!"))?;
        Ok(json_types::isg::OfflinePlaylist {
            class: Some(json_types::isg::OfflinePlaylist::CLASS),
            title_id: self.title,
            description_id: self.description,
            cover_path: Cow::Owned(format!(
                "world/ui/textures/covers/playlists_offline/{file_stem}.act"
            )),
            maps: self.maps,
        })
    }
}
