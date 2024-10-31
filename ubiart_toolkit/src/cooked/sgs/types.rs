use std::collections::HashMap;

use hipstr::HipStr;
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
    pub fn into_scene_settings(self) -> Result<SceneSettings<'a>, ParserError> {
        if let Sgs::SceneSettings(scene_settings) = self {
            Ok(scene_settings)
        } else {
            Err(ParserError::custom(format!(
                "Sgs is not a SceneSettings: {self:?}"
            )))
        }
    }

    /// Use this a `SceneConfigManager` type
    pub fn into_scene_config_manager(self) -> Result<SceneConfigManager<'a>, ParserError> {
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub keys: HashMap<HipStr<'a>, Settings<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "Pause_Level")]
    pub pause_level: u8,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(rename = "type")]
    pub typed: u8,
    pub musicscore: u8,
    #[serde(borrow)]
    pub sound_context: HipStr<'a>,
    pub hud: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_title_loc_id: Option<usize>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub phone_image: Option<HipStr<'a>>,
}

impl Default for MapSceneConfig<'_> {
    fn default() -> Self {
        Self {
            class: None,
            pause_level: 6,
            name: HipStr::borrowed(""),
            typed: 1,
            musicscore: 2,
            sound_context: HipStr::borrowed(""),
            hud: 0,
            phone_title_loc_id: None,
            phone_image: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionSceneConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "Pause_Level")]
    pub pause_level: u64,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub param_bindings: Vec<ParamBinding<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamBinding<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub param_name: HipStr<'a>,
    #[serde(borrow)]
    pub provider_class: HipStr<'a>,
    #[serde(borrow)]
    pub patcher_marker: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SongDatabaseSceneConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    /// Not in JD2016
    #[serde(
        rename = "Pause_Level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_level: Option<u64>,
    /// Not in JD2016
    #[serde(borrow, rename = "name", default)]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "SKU")]
    pub sku: HipStr<'a>,
    #[serde(borrow)]
    pub territory: HipStr<'a>,
    #[serde(borrow, rename = "RatingUI")]
    pub rating_ui: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub coverflow_sku_songs: Vec<CoverflowSong<'a>>,
}

impl Default for SongDatabaseSceneConfig<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            pause_level: Some(6),
            name: HipStr::borrowed(""),
            sku: HipStr::borrowed("jd2022-nx-all"),
            territory: HipStr::borrowed("NCSA"),
            rating_ui: HipStr::borrowed("world/ui/screens/boot_warning/boot_warning_esrb.isc"),
            coverflow_sku_songs: Vec::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct CoverflowSong<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub cover_path: HipStr<'a>,
}

impl CoverflowSong<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("CoverflowSong");
}
