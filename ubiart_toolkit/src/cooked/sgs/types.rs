use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::utils::errors::ParserError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields, untagged)]
pub enum Sgs<'a> {
    #[serde(borrow)]
    SceneSettings(SceneSettings<'a>),
    #[serde(borrow)]
    SceneConfigManager(SceneConfigManager<'a>),
}

impl<'a> Sgs<'a> {
    /// Use this a `SceneSettings` type
    ///
    /// # Errors
    /// Will error if this is not a `SceneSettings` type
    pub fn as_scene_settings(self) -> Result<SceneSettings<'a>, ParserError> {
        if let Sgs::SceneSettings(scene_settings) = self {
            Ok(scene_settings)
        } else {
            Err(ParserError::custom(format!(
                "Sgs is not a SceneSettings: {self:?}"
            )))
        }
    }

    /// Use this a `SceneConfigManager` type
    ///
    /// # Errors
    /// Will error if this is not a `SceneConfigManager` type
    pub fn as_scene_config_manager(self) -> Result<SceneConfigManager<'a>, ParserError> {
        if let Sgs::SceneConfigManager(scene_config_manager) = self {
            Ok(scene_config_manager)
        } else {
            Err(ParserError::custom(format!(
                "Sgs is not a SceneConfigManager: {self:?}"
            )))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneSettings<'a> {
    #[serde(borrow)]
    pub settings: Settings<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SceneConfigManager<'a> {
    pub version: u8,
    #[serde(borrow)]
    pub sgs_map: SgsKey<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SgsKey<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub keys: HashMap<Cow<'a, str>, Settings<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "__class")]
pub enum Settings<'a> {
    #[serde(borrow, rename = "JD_MapSceneConfig")]
    MapSceneConfig(MapSceneConfig<'a>),
    #[serde(borrow, rename = "JD_SongDatabaseSceneConfig")]
    SongDatabaseSceneConfig(SongDatabaseSceneConfig<'a>),
    #[serde(borrow, rename = "JD_TransitionSceneConfig")]
    TransitionSceneConfig(TransitionSceneConfig<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MapSceneConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "Pause_Level")]
    pub pause_level: u8,
    pub name: Cow<'a, str>,
    #[serde(rename = "type")]
    pub typed: u8,
    pub musicscore: u8,
    pub sound_context: Cow<'a, str>,
    pub hud: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_title_loc_id: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_image: Option<Cow<'a, str>>,
}

impl<'a> Default for MapSceneConfig<'a> {
    fn default() -> Self {
        Self {
            class: None,
            pause_level: 6,
            name: Cow::Borrowed(""),
            typed: 1,
            musicscore: 2,
            sound_context: Cow::Borrowed(""),
            hud: 0,
            phone_title_loc_id: None,
            phone_image: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionSceneConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    #[serde(rename = "Pause_Level")]
    pub pause_level: u64,
    pub name: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub param_bindings: Vec<ParamBinding<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamBinding<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub param_name: Cow<'a, str>,
    pub provider_class: Cow<'a, str>,
    pub patcher_marker: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDatabaseSceneConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "Pause_Level")]
    pub pause_level: u64,
    #[serde(rename = "name")]
    pub name: Cow<'a, str>,
    #[serde(rename = "SKU")]
    pub sku: Cow<'a, str>,
    pub territory: Cow<'a, str>,
    #[serde(rename = "RatingUI")]
    pub rating_ui: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coverflow_sku_songs: Vec<CoverflowSong<'a>>,
}

impl Default for SongDatabaseSceneConfig<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            pause_level: 6,
            name: Cow::Borrowed(""),
            sku: Cow::Borrowed("jd2022-nx-all"),
            territory: Cow::Borrowed("NCSA"),
            rating_ui: Cow::Borrowed("world/ui/screens/boot_warning/boot_warning_esrb.isc"),
            coverflow_sku_songs: Vec::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct CoverflowSong<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub cover_path: Cow<'a, str>,
}

impl<'a> CoverflowSong<'a> {
    pub const CLASS: &'static str = "CoverflowSong";
}
