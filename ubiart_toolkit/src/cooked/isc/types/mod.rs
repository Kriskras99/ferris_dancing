#![allow(
    clippy::struct_excessive_bools,
    reason = "The booleans are imposed by the UbiArt engine"
)]

use hipstr::HipStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use ubiart_toolkit_shared_types::Color;

mod binary;
pub mod property_patcher;

use property_patcher::WrappedPropertyPatcher;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Root<'a> {
    #[serde(borrow)]
    pub scene: Scene<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Scene<'a> {
    #[serde(rename = "@ENGINE_VERSION")]
    pub engine_version: u32,
    #[serde(rename = "@GRIDUNIT")]
    pub gridunit: f32,
    #[serde(rename = "@DEPTH_SEPARATOR")]
    pub depth_separator: usize,
    #[serde(
        rename = "@NEAR_SEPARATOR",
        deserialize_with = "deser_separator",
        serialize_with = "ser_separator"
    )]
    pub near_separator: [Color; 4],
    #[serde(
        rename = "@FAR_SEPARATOR",
        deserialize_with = "deser_separator",
        serialize_with = "ser_separator"
    )]
    pub far_separator: [Color; 4],
    #[serde(rename = "@viewFamily", serialize_with = "ser_bool")]
    pub view_family: bool,
    #[serde(default, rename = "@isPopup", serialize_with = "ser_bool")]
    pub is_popup: bool,
    #[serde(
        borrow,
        rename = "PLATFORM_FILTER",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub platform_filters: Vec<PlatformFilter<'a>>,
    #[serde(
        borrow,
        default,
        rename = "ACTORS",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actors: Vec<WrappedActors<'a>>,
    #[serde(borrow, rename = "sceneConfigs")]
    pub scene_configs: WrappedSceneConfigs<'a>,
}

impl Default for Scene<'_> {
    fn default() -> Self {
        Self {
            engine_version: 326_704,
            gridunit: 0.5,
            depth_separator: Default::default(),
            near_separator: [
                Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            far_separator: [
                Color {
                    color: (1.0, 0.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 1.0, 0.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 1.0, 0.0),
                },
                Color {
                    color: (0.0, 0.0, 0.0, 1.0),
                },
            ],
            view_family: Default::default(),
            is_popup: Default::default(),
            platform_filters: Vec::default(),
            actors: Vec::default(),
            scene_configs: SceneConfigs::default().into(),
        }
    }
}

impl Scene<'_> {
    /// Get an `SubSceneActor` from `self.actors` matching `userfriendly`
    pub fn get_subscene_by_userfriendly(
        &self,
        userfriendly: &str,
    ) -> Result<&SubSceneActor, ParserError> {
        self.actors
            .iter()
            .map(WrappedActors::sub_scene_actor)
            .filter_map(Result::ok)
            .find(|a| a.userfriendly == userfriendly)
            .ok_or_else(|| {
                ParserError::custom(format!(
                    "SubSceneActor matching '{userfriendly}' not found: SubSceneActors: {:?}",
                    self.actors
                        .iter()
                        .map(WrappedActors::sub_scene_actor)
                        .filter_map(Result::ok)
                        .map(|a| a.userfriendly.as_str())
                ))
            })
    }

    /// Get an `SubSceneActor` from `self.actors` ending in `userfriendly`
    pub fn get_subscene_by_userfriendly_end(
        &self,
        userfriendly: &str,
        also_lowercase: bool,
    ) -> Result<&SubSceneActor, ParserError> {
        let userfriendly = if also_lowercase {
            HipStr::from(userfriendly.to_lowercase())
        } else {
            HipStr::borrowed(userfriendly)
        };
        self.actors
            .iter()
            .map(WrappedActors::sub_scene_actor)
            .filter_map(Result::ok)
            .find(|a| {
                if also_lowercase {
                    a.userfriendly
                        .to_lowercase()
                        .ends_with(userfriendly.as_str())
                } else {
                    a.userfriendly.ends_with(&userfriendly.as_str())
                }
            })
            .ok_or_else(|| {
                ParserError::custom(format!(
                    "SubSceneActor ending in '{userfriendly}' not found: SubSceneActors: {:?}",
                    self.actors
                        .iter()
                        .map(WrappedActors::sub_scene_actor)
                        .filter_map(Result::ok)
                        .map(|a| a.userfriendly.as_str())
                ))
            })
    }

    /// Get an `Actor` from `self.actors` matching `userfriendly`
    pub fn get_actor_by_userfriendly_end(
        &self,
        userfriendly: &str,
        lax: bool,
    ) -> Result<&Actor, ParserError> {
        let userfriendly = if lax {
            HipStr::from(userfriendly.to_lowercase())
        } else {
            HipStr::borrowed(userfriendly)
        };
        self.actors
            .iter()
            .map(WrappedActors::actor)
            .filter_map(Result::ok)
            .find(|a| {
                if lax {
                    a.userfriendly
                        .to_lowercase()
                        .ends_with(userfriendly.as_str())
                } else {
                    a.userfriendly.ends_with(&userfriendly.as_str())
                }
            })
            .ok_or_else(|| {
                ParserError::custom(format!(
                    "Actor ending in '{userfriendly}' not found: Actors: {:?}",
                    self.actors
                        .iter()
                        .map(WrappedActors::actor)
                        .filter_map(Result::ok)
                        .map(|a| a.userfriendly.as_str())
                ))
            })
    }

    /// Get an actor from `self.actors` matching `userfriendly`
    pub fn get_actor_by_userfriendly(&self, userfriendly: &str) -> Result<&Actor, ParserError> {
        self.actors
            .iter()
            .map(WrappedActors::actor)
            .filter_map(Result::ok)
            .find(|a| a.userfriendly == userfriendly)
            .ok_or_else(|| {
                ParserError::custom(format!(
                    "Actor matching '{userfriendly}' not found: Actors: {:?}",
                    self.actors
                        .iter()
                        .map(WrappedActors::actor)
                        .filter_map(Result::ok)
                        .map(|a| a.userfriendly.as_str())
                ))
            })
    }
}

wrap!(WrappedSceneConfigs, SceneConfigs, "SceneConfigs", 'a);

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SceneConfigs<'a> {
    #[serde(rename = "@activeSceneConfig")]
    pub active_scene_config: u32,
    #[serde(
        borrow,
        rename = "sceneConfigs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub jd_scene_config: Vec<WrappedJdSceneConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIBannerSceneConfig<'a> {
    #[serde(borrow, rename = "@name")]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "@theme")]
    pub theme: HipStr<'a>,
    #[serde(borrow, rename = "@type")]
    pub typed: HipStr<'a>,
    #[serde(borrow, rename = "@context")]
    pub context: HipStr<'a>,
    #[serde(
        borrow,
        rename = "@enterChain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enter_chain: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@activeChain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_chain: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@leaveChain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leave_chain: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "paramBindings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub param_bindings: Vec<WrappedParamBinding<'a>>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIItemTextField<'a> {
    #[serde(rename = "@isPassword", serialize_with = "ser_bool")]
    pub is_password: bool,
    #[serde(rename = "@dialogMaxChar")]
    pub dialog_max_char: u32,
    #[serde(borrow, rename = "@dialogNameRaw")]
    pub dialog_name_raw: HipStr<'a>,
    #[serde(rename = "@dialogNameLoc")]
    pub dialog_name_loc: u32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TransitionSceneConfig<'a> {
    #[serde(borrow, rename = "@name")]
    pub name: HipStr<'a>,
    #[serde(
        borrow,
        rename = "paramBindings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub param_bindings: Vec<WrappedParamBinding<'a>>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

wrap!(WrappedParamBinding, ParamBinding, "ParamBinding", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ParamBinding<'a> {
    #[serde(borrow, rename = "@paramName")]
    pub param_name: HipStr<'a>,
    #[serde(borrow, rename = "@providerClass")]
    pub provider_class: HipStr<'a>,
    #[serde(borrow, rename = "@patcherMarker")]
    pub patcher_marker: HipStr<'a>,
    #[serde(
        borrow,
        rename = "dataBindings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_bindings: Vec<DataBindings<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct DataBindings<'a> {
    #[serde(borrow, rename = "@KEY")]
    pub key: HipStr<'a>,
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MapSceneConfig<'a> {
    #[serde(borrow, rename = "@name")]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(rename = "@hud")]
    pub hud: u32,
    #[serde(
        rename = "@phoneTitleLocId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_title_loc_id: Option<u32>,
    #[serde(
        borrow,
        rename = "@phoneImage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_image: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SongDatabaseSceneConfig<'a> {
    /// Not in 2016, always empty string
    #[serde(borrow, rename = "@name", default)]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "@SKU")]
    pub sku: HipStr<'a>,
    #[serde(borrow, rename = "@Territory")]
    pub territory: HipStr<'a>,
    #[serde(borrow, rename = "@RatingUI")]
    pub rating_ui: HipStr<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(
        borrow,
        rename = "CoverflowSkuSongs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub coverflow_sku_songs: Vec<CoverflowSkuSongs<'a>>,
}

impl Default for SongDatabaseSceneConfig<'static> {
    fn default() -> Self {
        Self {
            name: HipStr::borrowed(""),
            sku: HipStr::borrowed(""),
            territory: HipStr::borrowed("NCSA"),
            rating_ui: HipStr::borrowed(""),
            enums: vec![Enum {
                name: HipStr::borrowed("Pause_Level"),
                selection: 6,
            }],
            coverflow_sku_songs: Vec::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CoverflowSkuSongs<'a> {
    #[serde(borrow, rename = "CoverflowSong")]
    pub coverflow_song: CoverflowSong<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CoverflowSong<'a> {
    #[serde(borrow, rename = "@name")]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "@cover_path")]
    pub cover_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Actor<'a> {
    #[serde(rename = "@RELATIVEZ")]
    pub relativez: f32,
    #[serde(rename = "@SCALE")]
    pub scale: (f32, f32),
    #[serde(rename = "@xFLIPPED", serialize_with = "ser_bool")]
    pub x_flipped: bool,
    #[serde(borrow, rename = "@USERFRIENDLY")]
    pub userfriendly: HipStr<'a>,
    #[serde(borrow, rename = "@MARKER", skip_serializing_if = "Option::is_none")]
    pub marker: Option<HipStr<'a>>,
    /// Not used in nx2017
    #[serde(
        default,
        rename = "@DEFAULTENABLE",
        serialize_with = "ser_option_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub defaultenable: Option<bool>,
    #[serde(
        default,
        rename = "@isEnabled",
        serialize_with = "ser_option_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enabled: Option<bool>,
    #[serde(rename = "@POS2D")]
    pub pos2d: (f32, f32),
    #[serde(rename = "@ANGLE")]
    pub angle: f32,
    #[serde(borrow, rename = "@INSTANCEDATAFILE")]
    pub instancedatafile: HipStr<'a>,
    #[serde(borrow, rename = "@LUA")]
    pub lua: HipStr<'a>,
    #[serde(
        borrow,
        rename = "COMPONENTS",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub components: Vec<WrappedComponent<'a>>,
    #[serde(borrow, rename = "parentBind", skip_serializing_if = "Option::is_none")]
    pub parent_bind: Option<ParentBind<'a>>,
    #[serde(
        borrow,
        rename = "MARKERS",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub markers: Vec<Marker<'a>>,
}

impl Default for Actor<'_> {
    fn default() -> Self {
        Self {
            relativez: Default::default(),
            scale: (1.0, 1.0),
            x_flipped: Default::default(),
            userfriendly: HipStr::borrowed(""),
            marker: Some(HipStr::borrowed("")),
            defaultenable: Some(true),
            pos2d: Default::default(),
            angle: Default::default(),
            instancedatafile: HipStr::borrowed(""),
            lua: HipStr::borrowed(""),
            components: Vec::default(),
            parent_bind: Option::default(),
            markers: Vec::default(),
            is_enabled: Option::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ParentBind<'a> {
    #[serde(borrow, rename = "Bind")]
    pub bind: Bind<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Bind<'a> {
    #[serde(borrow, rename = "@parentPath")]
    pub parent_path: HipStr<'a>,
    #[serde(rename = "@typeData")]
    pub type_data: u32,
    #[serde(rename = "@offsetPos")]
    pub offset_pos: (f32, f32, f32),
    #[serde(rename = "@offsetAngle")]
    pub offset_angle: f32,
    #[serde(rename = "@localScale")]
    pub local_scale: (f32, f32),
    #[serde(rename = "@useParentFlip", serialize_with = "ser_bool")]
    pub use_parent_flip: bool,
    #[serde(rename = "@useParentAlpha", serialize_with = "ser_bool")]
    pub use_parent_alpha: bool,
    #[serde(rename = "@useParentColor", serialize_with = "ser_bool")]
    pub use_parent_color: bool,
    #[serde(rename = "@removeWithParent", serialize_with = "ser_bool")]
    pub remove_with_parent: bool,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIUploadIcon {
    #[serde(rename = "@timeSuccessDisplayed")]
    pub time_success_displayed: f32,
    #[serde(rename = "@timeErrorDisplayed")]
    pub time_error_displayed: f32,
    #[serde(rename = "@timeProgressDisplayed")]
    pub time_progress_displayed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CreditsComponent<'a> {
    #[serde(rename = "@linesNumber")]
    pub lines_number: f32,
    #[serde(rename = "@nameFontSize")]
    pub name_font_size: f32,
    #[serde(rename = "@titleFontSize")]
    pub title_font_size: f32,
    #[serde(rename = "@bigTitleFontSize")]
    pub big_title_font_size: f32,
    #[serde(rename = "@veryBigTitleFontSize")]
    pub very_big_title_font_size: f32,
    #[serde(rename = "@animDuration")]
    pub anim_duration: f32,
    #[serde(rename = "@linesPosOffset")]
    pub lines_pos_offset: f32,
    #[serde(
        rename = "@minAnimDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_anim_duration: Option<f32>,
    #[serde(
        rename = "@speedSteps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub speed_steps: Option<f32>,
    #[serde(
        rename = "@bottomSpawnY",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bottom_spawn_y: Option<f32>,
    #[serde(
        rename = "@topSpawnY",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub top_spawn_y: Option<f32>,
    #[serde(borrow, rename = "creditsLines")]
    pub credits_lines: Vec<CreditsLine<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CreditsLine<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SceneSpawnerComponent<'a> {
    #[serde(borrow, rename = "@editorScenePath")]
    pub editor_scene_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WDFThemePresentationComponent<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WDFTeamBattlePresentationComponent<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WDFBossSpawnerComponent<'a> {
    #[serde(rename = "@editorOnly", serialize_with = "ser_bool")]
    pub editor_only: bool,
    #[serde(borrow, rename = "@editorBossId")]
    pub editor_boss_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ScrollBarComponent {
    #[serde(rename = "@MinCursorHalfSize")]
    pub min_cursor_half_size: f32,
    #[serde(rename = "@MaxCursorHalfSize")]
    pub max_cursor_half_size: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PictoTimeline<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(rename = "@RelativeStartingPositionSolo")]
    pub relative_start_position_solo: (f32, f32, f32),
    #[serde(rename = "@RelativeStartingPositionDuo")]
    pub relative_start_position_duo: (f32, f32, f32),
    #[serde(rename = "@RelativeStartingPositionTrio")]
    pub relative_start_position_trio: (f32, f32, f32),
    #[serde(rename = "@RelativeStartingPositionQuatro")]
    pub relative_start_position_quatro: (f32, f32, f32),
    #[serde(rename = "@RelativeStartingPositionSextet")]
    pub relative_start_position_sextet: (f32, f32, f32),
    #[serde(rename = "@ShiftingPositionSolo")]
    pub shifting_position_solo: (f32, f32, f32),
    #[serde(rename = "@ShiftingPositionDuo")]
    pub shifting_position_duo: (f32, f32, f32),
    #[serde(rename = "@ShiftingPositionTrio")]
    pub shifting_position_trio: (f32, f32, f32),
    #[serde(rename = "@ShiftingPositionQuatro")]
    pub shifting_position_quatro: (f32, f32, f32),
    #[serde(rename = "@ShiftingPositionSextet")]
    pub shifting_position_sextet: (f32, f32, f32),
    #[serde(rename = "@PictoTrackOffset")]
    pub picto_track_offset: u32,
    #[serde(rename = "@PictoScale")]
    pub picto_scale: (f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIHudAutodanceRecorderComponent {
    #[serde(rename = "@IconDefaultPosition")]
    pub icon_default_position: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionSolo")]
    pub icon_relative_start_position_solo: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionDuo")]
    pub icon_relative_start_position_duo: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionTrio")]
    pub icon_relative_start_position_trio: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionQuatro")]
    pub icon_relative_start_position_quatro: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionSextet")]
    pub icon_relative_start_position_sextet: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionSolo")]
    pub icon_shifting_position_solo: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionDuo")]
    pub icon_shifting_position_duo: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionTrio")]
    pub icon_shifting_position_trio: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionQuatro")]
    pub icon_shifting_position_quatro: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionSextet")]
    pub icon_shifting_position_sextet: (f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BeatPulseComponent<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(borrow, rename = "Elements")]
    pub elements: Vec<WrappedUIWidgetElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetGroupHUDAutodanceRecorder<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(rename = "@IconDefaultPosition")]
    pub icon_default_position: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionSolo")]
    pub icon_relative_start_position_solo: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionDuo")]
    pub icon_relative_start_position_duo: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionTrio")]
    pub icon_relative_start_position_trio: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionQuatro")]
    pub icon_relative_start_position_quatro: (f32, f32, f32),
    #[serde(rename = "@IconRelativeStartPositionSextet")]
    pub icon_relative_start_position_sextet: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionSolo")]
    pub icon_shifting_position_solo: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionDuo")]
    pub icon_shifting_position_duo: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionTrio")]
    pub icon_shifting_position_trio: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionQuatro")]
    pub icon_shifting_position_quatro: (f32, f32, f32),
    #[serde(rename = "@IconShiftingPositionSextet")]
    pub icon_shifting_position_sextet: (f32, f32, f32),
    #[serde(borrow, rename = "Elements")]
    pub elements: Vec<WrappedUIWidgetElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetGroupHUDLyrics<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(borrow, rename = "Elements")]
    pub elements: Vec<WrappedUIWidgetElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetGroupHUDPauseIcon<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(borrow, rename = "Elements")]
    pub elements: Vec<WrappedUIWidgetElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetGroupHUD<'a> {
    #[serde(borrow, rename = "@text")]
    pub text: HipStr<'a>,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(borrow, rename = "@modelName")]
    pub model_name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(borrow, rename = "Elements")]
    pub elements: Vec<WrappedUIWidgetElementDesc<'a>>,
}

wrap!(WrappedUIWidgetElementDesc, UIWidgetElementDesc, "JD_UIWidgetElementDesc", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetElementDesc<'a> {
    #[serde(borrow, rename = "@elementPath")]
    pub element_path: HipStr<'a>,
    #[serde(borrow, rename = "@name")]
    pub name: HipStr<'a>,
    #[serde(borrow, rename = "@flag")]
    pub flag: HipStr<'a>,
    #[serde(rename = "@parentIndex")]
    pub parent_index: i32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct WDFTeamBattleTransitionComponent<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIItemSlot {
    #[serde(rename = "@slot")]
    pub slot: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIHudSweatCounter<'a> {
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIHudVersusPlayerComponent<'a> {
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RegistrationComponent<'a> {
    #[serde(borrow, rename = "@Tag")]
    pub tag: HipStr<'a>,
    #[serde(borrow, rename = "@UserData")]
    pub user_data: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ViewportUIComponent<'a> {
    #[serde(rename = "@active")]
    pub active: u32,
    #[serde(rename = "@focale")]
    pub focale: f32,
    #[serde(rename = "@farPlane")]
    pub far_plane: f32,
    #[serde(rename = "@Position")]
    pub position: (f32, f32),
    #[serde(rename = "@Size")]
    pub size: (f32, f32),
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Mesh3DComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@ScaleZ")]
    pub scale_z: f32,
    #[serde(borrow, rename = "@mesh3D")]
    pub mesh_3d: HipStr<'a>,
    #[serde(borrow, rename = "@skeleton3D")]
    pub skeleton_3d: HipStr<'a>,
    #[serde(borrow, rename = "@animation3D")]
    pub animation_3d: HipStr<'a>,
    #[serde(borrow, rename = "@animationNode")]
    pub animation_node: HipStr<'a>,
    #[serde(rename = "@orientation", deserialize_with = "deser_separator")]
    pub orientation: [Color; 4],
    #[serde(default, rename = "@force2DRender", serialize_with = "ser_bool")]
    pub force_2d_render: bool,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(borrow, rename = "materialList")]
    pub material_list: Material<'a>,
    #[serde(rename = "animation3DSet")]
    pub animation_3d_set: Animation3DSet,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TexturePatcherComponent<'a> {
    #[serde(borrow, rename = "@Diffuse1")]
    pub diffuse_1: HipStr<'a>,
    #[serde(borrow, rename = "@Diffuse2")]
    pub diffuse_2: HipStr<'a>,
    #[serde(borrow, rename = "@Diffuse3")]
    pub diffuse_3: HipStr<'a>,
    #[serde(borrow, rename = "@Diffuse4")]
    pub diffuse_4: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Animation3DSet {
    #[serde(rename = "Animation3DSet")]
    pub animation_3d_set: (),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIControl<'a> {
    #[serde(rename = "@isDpadSensitive", serialize_with = "ser_bool")]
    pub is_dpad_sensitive: bool,
    #[serde(rename = "@isCursorSensitive", serialize_with = "ser_bool")]
    pub is_cursor_sensitive: bool,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadLeft")]
    pub cursor_dpad_left: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadRight")]
    pub cursor_dpad_right: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadUp")]
    pub cursor_dpad_up: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadDown")]
    pub cursor_dpad_down: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIChangePage<'a> {
    #[serde(rename = "@isDpadSensitive", serialize_with = "ser_bool")]
    pub is_dpad_sensitive: bool,
    #[serde(rename = "@isCursorSensitive", serialize_with = "ser_bool")]
    pub is_cursor_sensitive: bool,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadLeft")]
    pub cursor_dpad_left: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadRight")]
    pub cursor_dpad_right: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadUp")]
    pub cursor_dpad_up: HipStr<'a>,
    #[serde(borrow, rename = "@cursorDpadDown")]
    pub cursor_dpad_down: HipStr<'a>,
    #[serde(borrow, rename = "@destination")]
    pub destination: HipStr<'a>,
    #[serde(rename = "@isBack", serialize_with = "ser_bool")]
    pub is_back: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIPhoneData<'a> {
    #[serde(rename = "@phoneLocId")]
    pub phone_loc_id: u32,
    #[serde(borrow, rename = "@phoneImage")]
    pub phone_image: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UICountdown {
    #[serde(rename = "@delay")]
    pub delay: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CameraFeedComponent<'a> {
    #[serde(rename = "@Picture_Beat")]
    pub picture_beat: f32,
    #[serde(rename = "@Record_StartBeat")]
    pub record_start_beat: f32,
    #[serde(rename = "@Record_Duration")]
    pub record_duration: f32,
    #[serde(rename = "@Replay_StartBeat")]
    pub replay_start_beat: f32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AFXPostProcessComponent {
    pub blur: Blur,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ClearColorComponent {
    #[serde(rename = "@clearColor")]
    pub clear_color: Color,
    #[serde(rename = "@clearFrontLightColor")]
    pub clear_front_light_color: Color,
    #[serde(rename = "@clearBackLightColor")]
    pub clear_back_light_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Blur {
    #[serde(rename = "AFX_BlurParam")]
    pub afx_blur_param: AFXBlurParam,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AFXBlurParam {
    #[serde(rename = "@use", serialize_with = "ser_bool")]
    pub to_use: bool,
    #[serde(rename = "@pixelSize")]
    pub pixel_size: u32,
    #[serde(rename = "@quality")]
    pub quality: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BoxInterpolatorComponent {
    #[serde(rename = "innerBox")]
    pub inner_box: IscBox,
    #[serde(rename = "outerBox")]
    pub outer_box: IscBox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct IscBox {
    #[serde(rename = "AABB")]
    pub aabb: AaBb,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AaBb {
    #[serde(rename = "@MIN")]
    pub min: (f32, f32),
    #[serde(rename = "@MAX")]
    pub max: (f32, f32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ConvertedTmlTapeComponent<'a> {
    #[serde(borrow, rename = "@MapName")]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIScreenComponent<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(rename = "@allowDpadNavigation", serialize_with = "ser_bool")]
    pub allow_dpad_navigation: bool,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_DEFAULT",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_default: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_SWITCH",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_switch: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_PS4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_ps4: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_XB1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_xb1: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_PC",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_pc: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_GGP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_ggp: Option<HipStr<'a>>,
    /// Not in nx2020 or earlier
    #[serde(
        borrow,
        default,
        rename = "@shortcutsConfig_PROSPERO",
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_prospero: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_SCARLETT",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_scarlett: Option<HipStr<'a>>,
    #[serde(
        rename = "@shortcutsFromCenterInsteadFromLeft",
        serialize_with = "ser_option_bool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_from_center_instead_from_left: Option<bool>,
    #[serde(
        rename = "@shortcutsHorizontalShift",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_horizontal_shift: Option<i32>,
    #[serde(borrow, rename = "@shortcutConfig")]
    pub shortcuts: Option<HipStr<'a>>,
    #[serde(rename = "@shortcutShift")]
    pub shortcut_shift: Option<u32>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow, rename = "phoneSetupUiData")]
    pub phone_setup_ui_data: PhoneSetupUIData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhoneSetupUIData<'a> {
    #[serde(borrow, rename = "PhoneSetupData")]
    pub phone_setup_data: PhoneSetupData<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PhoneSetupData<'a> {
    #[serde(default, rename = "@isPopup", serialize_with = "ser_bool")]
    pub is_popup: bool,
    #[serde(rename = "@hasVisibleActions", serialize_with = "ser_bool")]
    pub has_visible_actions: bool,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(
        borrow,
        rename = "userFriendlyBindings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_friendly_bindings: Vec<UserFriendlyBindings<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UserFriendlyBindings<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct LineGrid<'a> {
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(rename = "@switchSpeed")]
    pub switch_speed: f32,
    #[serde(borrow, rename = "@shortcutsConfig_DEFAULT")]
    pub shortcuts_config_default: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_SWITCH")]
    pub shortcuts_config_switch: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PS4")]
    pub shortcuts_config_ps4: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_XB1")]
    pub shortcuts_config_xb1: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PC")]
    pub shortcuts_config_pc: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_GGP")]
    pub shortcuts_config_ggp: HipStr<'a>,
    /// Not in nx2020 or earlier
    #[serde(
        borrow,
        default,
        rename = "@shortcutsConfig_Prospero",
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_prospero: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_Scarlett",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_scarlett: Option<HipStr<'a>>,
    #[serde(
        rename = "@shortcutsFromCenterInsteadFromLeft",
        serialize_with = "ser_bool"
    )]
    pub shortcuts_from_center_instead_from_left: bool,
    #[serde(rename = "@shortcutsHorizontalShift")]
    pub shortcuts_horizontal_shift: i32,
    #[serde(borrow, rename = "@initialBehaviour")]
    pub initial_behaviour: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
    #[serde(
        borrow,
        rename = "animItemsDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anim_item_desc: Option<WrappedAnimItemsDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIGrid<'a> {
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(borrow, rename = "@initialBehaviour")]
    pub initial_behaviour: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
    #[serde(
        borrow,
        rename = "animItemsDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anim_item_desc: Option<WrappedAnimItemsDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AnthologyGrid<'a> {
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(rename = "@switchSpeed")]
    pub switch_speed: f32,
    #[serde(borrow, rename = "@shortcutsConfig_DEFAULT")]
    pub shortcuts_config_default: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_SWITCH")]
    pub shortcuts_config_switch: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PS4")]
    pub shortcuts_config_ps4: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_XB1")]
    pub shortcuts_config_xb1: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PC")]
    pub shortcuts_config_pc: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_GGP")]
    pub shortcuts_config_ggp: HipStr<'a>,
    #[serde(
        rename = "@shortcutsFromCenterInsteadFromLeft",
        serialize_with = "ser_bool"
    )]
    pub shortcuts_from_center_instead_from_left: bool,
    #[serde(rename = "@shortcutsHorizontalShift")]
    pub shortcuts_horizontal_shift: i32,
    #[serde(borrow, rename = "@initialBehaviour")]
    pub initial_behaviour: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
    #[serde(
        borrow,
        rename = "animItemsDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anim_item_desc: Option<WrappedAnimItemsDesc<'a>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Carousel<'a> {
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(rename = "@switchSpeed")]
    pub switch_speed: f32,
    #[serde(borrow, rename = "@shortcutsConfig_DEFAULT")]
    pub shortcuts_config_default: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_SWITCH")]
    pub shortcuts_config_switch: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PS4")]
    pub shortcuts_config_ps4: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_XB1")]
    pub shortcuts_config_xb1: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PC")]
    pub shortcuts_config_pc: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_GGP")]
    pub shortcuts_config_ggp: HipStr<'a>,
    /// Not in nx2020 or earlier
    #[serde(
        borrow,
        default,
        rename = "@shortcutsConfig_Prospero",
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_prospero: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_Scarlett",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_scarlett: Option<HipStr<'a>>,
    #[serde(
        rename = "@shortcutsFromCenterInsteadFromLeft",
        serialize_with = "ser_bool"
    )]
    pub shortcuts_from_center_instead_from_left: bool,
    #[serde(rename = "@shortcutsHorizontalShift")]
    pub shortcuts_horizontal_shift: i32,
    #[serde(borrow, rename = "@initialBehaviour")]
    pub initial_behaviour: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(rename = "@minNbItemsToLoop")]
    pub min_nb_items_to_loop: u32,
    #[serde(rename = "@forceLoop", serialize_with = "ser_bool")]
    pub force_loop: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
    #[serde(
        borrow,
        rename = "animItemsDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anim_item_desc: Option<WrappedAnimItemsDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Grid<'a> {
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(rename = "@switchSpeed")]
    pub switch_speed: f32,
    #[serde(borrow, rename = "@shortcutsConfig_DEFAULT")]
    pub shortcuts_config_default: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_SWITCH")]
    pub shortcuts_config_switch: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PS4")]
    pub shortcuts_config_ps4: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_XB1")]
    pub shortcuts_config_xb1: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_PC")]
    pub shortcuts_config_pc: HipStr<'a>,
    #[serde(borrow, rename = "@shortcutsConfig_GGP")]
    pub shortcuts_config_ggp: HipStr<'a>,
    /// Not in nx2020 or earlier
    #[serde(
        borrow,
        default,
        rename = "@shortcutsConfig_Prospero",
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_prospero: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@shortcutsConfig_Scarlett",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub shortcuts_config_scarlett: Option<HipStr<'a>>,
    #[serde(
        rename = "@shortcutsFromCenterInsteadFromLeft",
        serialize_with = "ser_bool"
    )]
    pub shortcuts_from_center_instead_from_left: bool,
    #[serde(rename = "@shortcutsHorizontalShift")]
    pub shortcuts_horizontal_shift: i32,
    #[serde(borrow, rename = "@initialBehaviour")]
    pub initial_behaviour: HipStr<'a>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, rename = "@gridArea_topLeftMarker")]
    pub grid_area_top_left_marker: HipStr<'a>,
    #[serde(borrow, rename = "@gridArea_bottomRightMarker")]
    pub grid_area_bottom_right_marker: HipStr<'a>,
    #[serde(borrow, rename = "@cursorArea_topLeftMarker")]
    pub cursor_area_top_left_marker: HipStr<'a>,
    #[serde(borrow, rename = "@cursorArea_bottomRightMarker")]
    pub cursor_area_bottom_right_marker: HipStr<'a>,
    #[serde(rename = "@centerGrid")]
    pub center_grid: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
    #[serde(
        borrow,
        rename = "animItemsDesc",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub anim_item_desc: Option<WrappedAnimItemsDesc<'a>>,
}

wrap!(WrappedAnimItemsDesc, AnimItemsDesc, "$value", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub enum AnimItemsDesc<'a> {
    #[serde(rename = "BrowserAnimItemsDesc")]
    Browser(BrowserAnimItemsDesc),
    #[serde(borrow, rename = "CarouselAnimItemsDesc")]
    Carousel(CarouselAnimItemsDesc<'a>),
}

wrap!(
    WrappedBrowserAnimItemsDesc,
    BrowserAnimItemsDesc,
    "BrowserAnimItemsDesc"
);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BrowserAnimItemsDesc {
    #[serde(rename = "@enable", serialize_with = "ser_bool")]
    pub enable: bool,
    #[serde(rename = "@showItemsAtInit", serialize_with = "ser_bool")]
    pub show_items_at_init: bool,
    #[serde(rename = "@enableBrowserOnAnimEnds", serialize_with = "ser_bool")]
    pub enable_browser_on_anim_ends: bool,
    #[serde(
        rename = "@checkItemsVisibilityOnAnimEnds",
        serialize_with = "ser_bool"
    )]
    pub check_items_visibility_on_anim_ends: bool,
}

wrap!(WrappedCarouselAnimItemsDesc, CarouselAnimItemsDesc, "CarouselAnimItemsDesc", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CarouselAnimItemsDesc<'a> {
    #[serde(rename = "@enable", serialize_with = "ser_bool")]
    pub enable: bool,
    #[serde(rename = "@showItemsAtInit", serialize_with = "ser_bool")]
    pub show_items_at_init: bool,
    #[serde(rename = "@enableCarouselOnAnimEnds", serialize_with = "ser_bool")]
    pub enable_carousel_on_anim_ends: bool,
    #[serde(
        rename = "@checkItemsVisibilityOnAnimEnds",
        serialize_with = "ser_bool"
    )]
    pub check_items_visibility_on_anim_ends: bool,
    #[serde(
        borrow,
        rename = "animsToListen",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub anims_to_listen: Vec<AnimsToListen<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AnimsToListen<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub val: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CarouselBehaviourNavigation<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoNext")]
    pub sound_notif_go_next: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoPrev")]
    pub sound_notif_go_prev: HipStr<'a>,
    #[serde(borrow, rename = "@animSetupID")]
    pub anim_setup_id: HipStr<'a>,
    #[serde(borrow, rename = "@decelTapeLabel")]
    pub decel_tape_label: HipStr<'a>,
    #[serde(rename = "@timeBetweenStep")]
    pub time_between_step: f32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(
        borrow,
        rename = "stopConditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stop_conditions: Vec<StopConditions<'a>>,
    #[serde(
        borrow,
        rename = "nextActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "prevActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prev_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "upActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub up_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "downActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub down_actions: Vec<Value<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CarouselBehaviourNavigationGoToElement<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoNext")]
    pub sound_notif_go_next: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoPrev")]
    pub sound_notif_go_prev: HipStr<'a>,
    #[serde(borrow, rename = "@animSetupID")]
    pub anim_setup_id: HipStr<'a>,
    #[serde(borrow, rename = "@decelTapeLabel")]
    pub decel_tape_label: HipStr<'a>,
    #[serde(rename = "@timeBetweenStep")]
    pub time_between_steps: f32,
    #[serde(rename = "@idxToReach")]
    pub idx_to_reach: i32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(
        borrow,
        rename = "stopConditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stop_conditions: Vec<StopConditions<'a>>,
    #[serde(
        borrow,
        rename = "nextActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "prevActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prev_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "upActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub up_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "downActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub down_actions: Vec<Value<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct CarouselBehaviourNavigationAutoScroll<'a> {
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoNext")]
    pub sound_notif_go_next: HipStr<'a>,
    #[serde(borrow, rename = "@soundNotifGoPrev")]
    pub sound_notif_go_prev: HipStr<'a>,
    #[serde(borrow, rename = "@animSetupID")]
    pub anim_setup_id: HipStr<'a>,
    #[serde(borrow, rename = "@decelTapeLabel")]
    pub decel_tape_label: HipStr<'a>,
    #[serde(rename = "@timeBetweenStep")]
    pub time_between_steps: f32,
    #[serde(rename = "@activeOnDisabledCarousel")]
    pub active_on_disabled_carousel: u32,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(
        borrow,
        rename = "stopConditions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stop_conditions: Vec<StopConditions<'a>>,
    #[serde(
        borrow,
        rename = "nextActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "prevActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prev_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "upActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub up_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "downActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub down_actions: Vec<Value<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct StopConditions<'a> {
    #[serde(borrow, rename = "StopCondition")]
    pub stop_condition: StopCondition<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct StopCondition<'a> {
    #[serde(rename = "@waitingTime")]
    pub waiting_time: f32,
    #[serde(rename = "@countToReach")]
    pub count_to_reach: u32,
    #[serde(borrow, rename = "@nextBehaviour")]
    pub next_behaviour: HipStr<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Value<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BezierTreeComponent<'a> {
    #[serde(borrow, rename = "branch")]
    pub branch: Branch<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Branch<'a> {
    #[serde(borrow, rename = "BezierBranch")]
    pub branch: BezierBranch<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BezierBranch<'a> {
    #[serde(rename = "@autoStartTweening", serialize_with = "ser_bool")]
    pub auto_start_tweening: bool,
    pub nodes: Vec<Node>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<WrappedComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Node {
    #[serde(rename = "BezierNode")]
    pub bezier_node: BezierNode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BezierNode {
    #[serde(rename = "@pos")]
    pub pos: (f32, f32, f32),
    #[serde(rename = "@tangent")]
    pub tangent: (f32, f32),
    #[serde(rename = "@scale")]
    pub scale: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FXControllerComponent {
    #[serde(rename = "@allowBusMixEvents", serialize_with = "ser_bool")]
    pub allow_bus_mix_events: bool,
    #[serde(rename = "@allowMusicEvents", serialize_with = "ser_bool")]
    pub allow_music_events: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FxBankComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@drawDebug", serialize_with = "ser_bool")]
    pub draw_debug: bool,
    #[serde(rename = "@drawDebugTextOffset")]
    pub draw_debug_text_offset: (f32, f32),
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIAnchor {
    #[serde(rename = "@anchorIdx")]
    pub anchor_idx: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIHudRacelineDM {
    #[serde(rename = "@progressAnimSpeed")]
    pub progess_anim_speed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UICarousel<'a> {
    #[serde(
        rename = "@acceleration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub acceleration: Option<f32>,
    #[serde(
        rename = "@deceleration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deceleration: Option<f32>,
    #[serde(rename = "@minSpeed", default, skip_serializing_if = "Option::is_none")]
    pub min_speed: Option<f32>,
    #[serde(rename = "@maxSpeed", default, skip_serializing_if = "Option::is_none")]
    pub max_speed: Option<f32>,
    #[serde(rename = "@mainAnchor")]
    pub main_anchor: u32,
    #[serde(
        rename = "@minDecelerationStartRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_deceleration_start_ratio: Option<f32>,
    #[serde(
        rename = "@maxDecelerationStartRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_deceleration_start_ratio: Option<f32>,
    #[serde(borrow, rename = "@validateAction")]
    pub validate_action: HipStr<'a>,
    #[serde(borrow, rename = "@carouselDataID")]
    pub carousel_data_id: HipStr<'a>,
    #[serde(
        rename = "@timeBetweenStep",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_between_step: Option<f32>,
    #[serde(borrow, rename = "@soundContext")]
    pub sound_context: HipStr<'a>,
    #[serde(
        borrow,
        rename = "@soundNotifGoNext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sound_notif_go_next: Option<HipStr<'a>>,
    #[serde(
        borrow,
        rename = "@soundNotifGoPrev",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sound_notif_go_prev: Option<HipStr<'a>>,
    #[serde(rename = "@forceLoop", serialize_with = "ser_bool")]
    pub force_loop: bool,
    #[serde(
        rename = "@focusAnimsOnDisabledItems",
        serialize_with = "ser_option_bool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub focus_anims_on_disabled_items: Option<bool>,
    #[serde(rename = "@manageCarouselHistory", serialize_with = "ser_bool")]
    pub manage_carousel_history: bool,
    #[serde(rename = "@minNbItemsToLoop")]
    pub min_nb_items_to_loop: u32,
    #[serde(
        rename = "@autoScroll",
        serialize_with = "ser_option_bool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_scroll: Option<bool>,
    #[serde(
        rename = "@autoScrollPauseTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_scroll_pause_time: Option<f32>,
    #[serde(
        rename = "@autoScrollMaxSpeedRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_scroll_max_speed_ratio: Option<f32>,
    #[serde(
        borrow,
        rename = "nextActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub next_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "prevActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prev_actions: Vec<Value<'a>>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow, rename = "animItemsDesc")]
    pub anim_items_desc: WrappedAnimItemsDesc<'a>,
    #[serde(
        borrow,
        rename = "@initialBehaviour",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_behaviour: Option<HipStr<'a>>,
    #[serde(
        rename = "@switchSpeed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_speed: Option<f32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub behaviours: Vec<ValWrappedCarouselBehaviour<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UINineSliceComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@AtlasIndex")]
    pub atlas_index: u32,
    #[serde(rename = "@customAnchor")]
    pub custom_anchor: (f32, f32),
    #[serde(rename = "@SinusAmplitude")]
    pub sinus_amplitude: (f32, f32, f32),
    #[serde(rename = "@SinusSpeed")]
    pub sinus_speed: f32,
    #[serde(rename = "@AngleX")]
    pub angle_x: f32,
    #[serde(rename = "@AngleY")]
    pub angle_y: f32,
    #[serde(rename = "@TopSlice")]
    pub top_slice: f32,
    #[serde(rename = "@BottomSlice")]
    pub bottom_slice: f32,
    #[serde(rename = "@LeftSlice")]
    pub left_slice: f32,
    #[serde(rename = "@RightSlice")]
    pub right_slice: f32,
    #[serde(rename = "@TopSliceScale")]
    pub top_slice_scale: f32,
    #[serde(rename = "@BottomSliceScale")]
    pub bottom_slice_scale: f32,
    #[serde(rename = "@LeftSliceScale")]
    pub left_slice_scale: f32,
    #[serde(rename = "@RightSliceScale")]
    pub right_slice_scale: f32,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow)]
    pub material: Material<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UINineSliceMaskComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@AtlasIndex")]
    pub atlas_index: u32,
    #[serde(rename = "@customAnchor")]
    pub custom_anchor: (f32, f32),
    #[serde(rename = "@SinusAmplitude")]
    pub sinus_amplitude: (f32, f32, f32),
    #[serde(rename = "@SinusSpeed")]
    pub sinus_speed: f32,
    #[serde(rename = "@AngleX")]
    pub angle_x: f32,
    #[serde(rename = "@AngleY")]
    pub angle_y: f32,
    #[serde(rename = "@TopSlice")]
    pub top_slice: f32,
    #[serde(rename = "@BottomSlice")]
    pub bottom_slice: f32,
    #[serde(rename = "@LeftSlice")]
    pub left_slice: f32,
    #[serde(rename = "@RightSlice")]
    pub right_slice: f32,
    #[serde(rename = "@TopSliceScale")]
    pub top_slice_scale: f32,
    #[serde(rename = "@BottomSliceScale")]
    pub bottom_slice_scale: f32,
    #[serde(rename = "@LeftSliceScale")]
    pub left_slice_scale: f32,
    #[serde(rename = "@RightSliceScale")]
    pub right_slice_scale: f32,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow)]
    pub material: Material<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UITextBox<'a> {
    #[serde(rename = "@style")]
    pub style: u32,
    #[serde(rename = "@overridingFontSize")]
    pub overriding_font_size: f32,
    #[serde(rename = "@offset")]
    pub offset: (f32, f32),
    #[serde(rename = "@scale")]
    pub scale: (f32, f32),
    #[serde(rename = "@alpha")]
    pub alpha: f32,
    #[serde(rename = "@maxWidth")]
    pub max_width: f32,
    #[serde(rename = "@maxHeight")]
    pub max_height: f32,
    #[serde(rename = "@area")]
    pub area: (f32, f32),
    #[serde(borrow, rename = "@rawText")]
    pub raw_text: HipStr<'a>,
    #[serde(rename = "@useLinesMaxCount", serialize_with = "ser_bool")]
    pub use_lines_max_count: bool,
    #[serde(rename = "@linesMaxCount")]
    pub lines_max_count: i32,
    #[serde(rename = "@locId")]
    pub loc_id: u32,
    #[serde(rename = "@autoScrollSpeed")]
    pub auto_scroll_speed: f32,
    #[serde(rename = "@autoScrollSpeedY")]
    pub auto_scroll_speed_y: f32,
    #[serde(rename = "@autoScrollWaitTime")]
    pub auto_scroll_wait_time: f32,
    #[serde(rename = "@autoScrollWaitTimeY")]
    pub auto_scroll_wait_time_y: f32,
    #[serde(borrow, rename = "@autoScrollFontEffectName")]
    pub auto_scroll_font_effect_name: HipStr<'a>,
    #[serde(rename = "@autoScrollResetOnInactive", serialize_with = "ser_bool")]
    pub auto_scroll_reset_on_inactive: bool,
    #[serde(rename = "@scrollOnce", serialize_with = "ser_bool")]
    pub scroll_once: bool,
    #[serde(
        rename = "@autoScrollSharpAlignMinimum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_scroll_sharp_align_minimum: Option<f32>,
    #[serde(rename = "@linesBetweenLoopingText")]
    pub lines_between_looping_text: f32,
    #[serde(rename = "@numberOfCharactersToForceBreak")]
    pub number_of_characters_to_force_break: u32,
    #[serde(rename = "@overridingColor")]
    pub overriding_color: Color,
    #[serde(rename = "@overridingShadowColor")]
    pub overriding_shadow_color: Color,
    #[serde(rename = "@overridingShadowOffset")]
    pub overriding_shadow_offset: (f32, f32),
    #[serde(rename = "@overridingLineSpacing")]
    pub overriding_line_spacing: f32,
    #[serde(rename = "@adapteFontSize", serialize_with = "ser_bool")]
    pub adapte_font_size: bool,
    #[serde(rename = "@overridingFontSizeMin")]
    pub overriding_font_size_min: f32,
    #[serde(rename = "@endingDots", serialize_with = "ser_bool")]
    pub ending_dots: bool,
    #[serde(
        rename = "@colorizeIcons",
        serialize_with = "ser_option_bool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub colorize_icons: Option<bool>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIRootComponent {
    #[serde(rename = "@snapOffset")]
    pub snap_offset: (f32, f32),
    #[serde(rename = "Collision")]
    pub collision: Collision,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Collision {
    #[serde(rename = "UIWidgetCollisionBox")]
    pub ui_widget_collision_box: UIWidgetCollisionBox,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct UIWidgetCollisionBox {
    #[serde(rename = "@Width")]
    pub width: f32,
    #[serde(rename = "@Height")]
    pub height: f32,
    #[serde(rename = "@CenterOffsetX")]
    pub center_offset_x: f32,
    #[serde(rename = "@CenterOffsetY")]
    pub center_offset_y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MaterialGraphicComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@AtlasIndex")]
    pub atlas_index: u32,
    #[serde(rename = "@customAnchor")]
    pub custom_anchor: (f32, f32),
    #[serde(rename = "@SinusAmplitude")]
    pub sinus_amplitude: (f32, f32, f32),
    #[serde(rename = "@SinusSpeed")]
    pub sinus_speed: f32,
    #[serde(rename = "@AngleX")]
    pub angle_x: f32,
    #[serde(rename = "@AngleY")]
    pub angle_y: f32,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow)]
    pub material: Material<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextureGraphicComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@spriteIndex")]
    pub sprite_index: u32,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow)]
    pub material: Material<'a>,
}

impl Default for MaterialGraphicComponent<'static> {
    fn default() -> Self {
        Self {
            color_computer_tag_id: Default::default(),
            render_in_target: Default::default(),
            disable_light: Default::default(),
            disable_shadow: u32::MAX,
            atlas_index: Default::default(),
            custom_anchor: Default::default(),
            sinus_amplitude: Default::default(),
            sinus_speed: 1.0,
            angle_x: Default::default(),
            angle_y: Default::default(),
            primitive_parameters: PrimitiveParameters::default(),
            enums: vec![
                Enum {
                    name: HipStr::borrowed("anchor"),
                    selection: 1,
                },
                Enum {
                    name: HipStr::borrowed("oldAnchor"),
                    selection: 1,
                },
            ],
            material: Material::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct FixedCameraComponent {
    #[serde(rename = "@remote")]
    pub remote: f32,
    #[serde(rename = "@offset")]
    pub offset: (f32, f32, f32),
    #[serde(rename = "@startAsMainCam")]
    pub start_as_main_cam: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PleoComponent<'a> {
    #[serde(borrow, rename = "@video")]
    pub video: HipStr<'a>,
    #[serde(borrow, rename = "@dashMPD")]
    pub dash_mpd: HipStr<'a>,
    #[serde(borrow, rename = "@channelID")]
    pub channel_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PleoTextureGraphicComponent<'a> {
    #[serde(rename = "@colorComputerTagId")]
    pub color_computer_tag_id: u32,
    #[serde(rename = "@renderInTarget", serialize_with = "ser_bool")]
    pub render_in_target: bool,
    #[serde(rename = "@disableLight", serialize_with = "ser_bool")]
    pub disable_light: bool,
    #[serde(rename = "@disableShadow")]
    pub disable_shadow: u32,
    #[serde(rename = "@AtlasIndex")]
    pub atlas_index: u32,
    #[serde(rename = "@customAnchor")]
    pub custom_anchor: (f32, f32),
    #[serde(rename = "@SinusAmplitude")]
    pub sinus_amplitude: (f32, f32, f32),
    #[serde(rename = "@SinusSpeed")]
    pub sinus_speed: f32,
    #[serde(rename = "@AngleX")]
    pub angle_x: f32,
    #[serde(rename = "@AngleY")]
    pub angle_y: f32,
    #[serde(borrow, rename = "@channelID")]
    pub channel_id: HipStr<'a>,
    #[serde(borrow, rename = "PrimitiveParameters")]
    pub primitive_parameters: PrimitiveParameters<'a>,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow, rename = "material")]
    pub material: Material<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct PrimitiveParameters<'a> {
    #[serde(borrow, rename = "GFXPrimitiveParam")]
    pub gfx_primitive_param: GFXPrimitiveParam<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXPrimitiveParam<'a> {
    #[serde(rename = "@colorFactor")]
    pub color_factor: Color,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
}

impl Default for GFXPrimitiveParam<'static> {
    fn default() -> Self {
        Self {
            color_factor: Color {
                color: (1.0, 1.0, 1.0, 1.0),
            },
            enums: vec![Enum {
                name: HipStr::borrowed("gfxOccludeInfo"),
                selection: 0,
            }],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct Material<'a> {
    #[serde(borrow, rename = "GFXMaterialSerializable")]
    pub gfx_material_serializable: GFXMaterialSerializable<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GFXMaterialSerializable<'a> {
    #[serde(rename = "@ATL_Channel")]
    pub atl_channel: u32,
    #[serde(borrow, rename = "@ATL_Path")]
    pub atl_path: HipStr<'a>,
    #[serde(borrow, rename = "@shaderPath")]
    pub shader_path: HipStr<'a>,
    #[serde(
        default,
        rename = "@stencilTest",
        skip_serializing_if = "Option::is_none"
    )]
    pub stencil_test: Option<u32>,
    #[serde(rename = "@alphaTest")]
    pub alpha_test: u32,
    #[serde(rename = "@alphaRef")]
    pub alpha_ref: u32,
    #[serde(borrow)]
    pub texture_set: TextureSet<'a>,
    pub material_params: MaterialParams,
    /// Missing sometimes in nx2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outlined_mask_params: Option<OutlinedMaskParams>,
}

impl Default for GFXMaterialSerializable<'static> {
    fn default() -> Self {
        Self {
            atl_channel: Default::default(),
            atl_path: HipStr::borrowed(""),
            shader_path: HipStr::borrowed(""),
            stencil_test: Option::default(),
            alpha_test: u32::MAX,
            alpha_ref: u32::MAX,
            texture_set: TextureSet::default(),
            material_params: MaterialParams::default(),
            outlined_mask_params: Some(OutlinedMaskParams::default()),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TextureSet<'a> {
    #[serde(borrow, rename = "GFXMaterialTexturePathSet")]
    pub gfx_material_texture_path_set: GFXMaterialTexturePathSet<'a>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialTexturePathSet<'a> {
    #[serde(borrow, rename = "@diffuse")]
    pub diffuse: HipStr<'a>,
    #[serde(borrow, rename = "@back_light")]
    pub back_light: HipStr<'a>,
    #[serde(borrow, rename = "@normal")]
    pub normal: HipStr<'a>,
    #[serde(borrow, rename = "@separateAlpha")]
    pub separate_alpha: HipStr<'a>,
    #[serde(borrow, rename = "@diffuse_2")]
    pub diffuse_2: HipStr<'a>,
    #[serde(borrow, rename = "@back_light_2")]
    pub back_light_2: HipStr<'a>,
    #[serde(borrow, rename = "@anim_impostor")]
    pub anim_impostor: HipStr<'a>,
    #[serde(borrow, rename = "@diffuse_3")]
    pub diffuse_3: HipStr<'a>,
    #[serde(borrow, rename = "@diffuse_4")]
    pub diffuse_4: HipStr<'a>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MaterialParams {
    #[serde(rename = "GFXMaterialSerializableParam")]
    pub gfx_material_serializable_param: GFXMaterialSerializableParam,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXMaterialSerializableParam {
    #[serde(rename = "@Reflector_factor")]
    pub reflector_factor: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct OutlinedMaskParams {
    #[serde(rename = "OutlinedMaskMaterialParams")]
    pub outline_mask_material_params: OutlinedMaskMaterialParams,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct OutlinedMaskMaterialParams {
    #[serde(rename = "@maskColor")]
    pub mask_color: Color,
    #[serde(rename = "@outlineColor")]
    pub outline_color: Color,
    #[serde(rename = "@thickness")]
    pub thickness: f32,
}

impl Default for OutlinedMaskMaterialParams {
    fn default() -> Self {
        Self {
            mask_color: Color::default(),
            outline_color: Color::default(),
            thickness: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SubSceneActor<'a> {
    #[serde(rename = "@RELATIVEZ")]
    pub relativez: f32,
    #[serde(rename = "@SCALE")]
    pub scale: (f32, f32),
    #[serde(rename = "@xFLIPPED", serialize_with = "ser_bool")]
    pub x_flipped: bool,
    #[serde(borrow, rename = "@USERFRIENDLY")]
    pub userfriendly: HipStr<'a>,
    #[serde(borrow, rename = "@MARKER", skip_serializing_if = "Option::is_none")]
    pub marker: Option<HipStr<'a>>,
    /// Not used in nx2017
    #[serde(
        default,
        rename = "@DEFAULTENABLE",
        serialize_with = "ser_option_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub defaultenable: Option<bool>,
    #[serde(
        default,
        rename = "@isEnabled",
        serialize_with = "ser_option_bool",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_enabled: Option<bool>,
    #[serde(rename = "@POS2D")]
    pub pos2d: (f32, f32),
    #[serde(rename = "@ANGLE")]
    pub angle: f32,
    #[serde(borrow, rename = "@INSTANCEDATAFILE")]
    pub instancedatafile: HipStr<'a>,
    #[serde(borrow, rename = "@LUA")]
    pub lua: HipStr<'a>,
    #[serde(borrow, rename = "@RELATIVEPATH")]
    pub relativepath: HipStr<'a>,
    #[serde(rename = "@EMBED_SCENE", serialize_with = "ser_bool")]
    pub embed_scene: bool,
    #[serde(rename = "@IS_SINGLE_PIECE", serialize_with = "ser_bool")]
    pub is_single_piece: bool,
    #[serde(rename = "@ZFORCED", serialize_with = "ser_bool")]
    pub zforced: bool,
    #[serde(rename = "@DIRECT_PICKING", serialize_with = "ser_bool")]
    pub direct_picking: bool,
    #[serde(rename = "@IGNORE_SAVE", serialize_with = "ser_bool")]
    pub ignore_save: bool,
    #[serde(
        borrow,
        rename = "ENUM",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enums: Vec<Enum<'a>>,
    #[serde(borrow, rename = "SCENE")]
    pub wrapped_scene: WrappedScene<'a>,
    #[serde(
        borrow,
        rename = "COMPONENTS",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub components: Vec<WrappedComponent<'a>>,
    #[serde(borrow, rename = "parentBind", skip_serializing_if = "Option::is_none")]
    pub parent_bind: Option<ParentBind<'a>>,
    #[serde(
        borrow,
        rename = "MARKERS",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub markers: Vec<Marker<'a>>,
}

impl Default for SubSceneActor<'_> {
    fn default() -> Self {
        Self {
            relativez: Default::default(),
            scale: (1.0, 1.0),
            x_flipped: Default::default(),
            userfriendly: HipStr::borrowed(""),
            marker: Some(HipStr::borrowed("")),
            defaultenable: Some(true),
            pos2d: Default::default(),
            angle: Default::default(),
            instancedatafile: HipStr::borrowed(""),
            lua: HipStr::borrowed(""),
            relativepath: HipStr::borrowed(""),
            embed_scene: true,
            is_single_piece: Default::default(),
            zforced: true,
            direct_picking: true,
            ignore_save: Default::default(),
            enums: Vec::default(),
            wrapped_scene: Scene::default().into(),
            components: Vec::default(),
            parent_bind: Option::default(),
            markers: Vec::default(),
            is_enabled: Option::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Marker<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

wrap!(WrappedScene, Scene, "Scene", 'a);

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Enum<'a> {
    #[serde(borrow, rename = "@NAME")]
    pub name: HipStr<'a>,
    #[serde(rename = "@SEL")]
    pub selection: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct PlatformFilter<'a> {
    #[serde(borrow)]
    pub target_filter_list: TargetFilterList<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TargetFilterList<'a> {
    #[serde(borrow, rename = "@platform")]
    pub platform: HipStr<'a>,
    #[serde(borrow)]
    pub objects: Vec<TargetFilterObject<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TargetFilterObject<'a> {
    #[serde(borrow, rename = "@VAL")]
    pub value: HipStr<'a>,
}

/// Serialize a boolean as a "1" or a "0"
#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "required by the Serde api"
)]
fn ser_bool<S>(data: &bool, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if *data {
        ser.serialize_str("1")
    } else {
        ser.serialize_str("0")
    }
}

/// Serialize a Option<boolean> as a "1" or a "0"
///
/// The Option needs to be Some!
#[allow(
    clippy::trivially_copy_pass_by_ref,
    clippy::ref_option,
    reason = "required by the Serde api"
)]
fn ser_option_bool<S>(data: &Option<bool>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;
    let act_data = data.ok_or_else(|| Error::custom("Option<bool> is empty!"))?;
    if act_data {
        ser.serialize_str("1")
    } else {
        ser.serialize_str("0")
    }
}

/// Serialize the separator as a string
fn ser_separator<S>(data: &[Color; 4], ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_str(&format!("{:.6} {:.6} {:.6} {:.6}, {:.6} {:.6} {:.6} {:.6}, {:.6} {:.6} {:.6} {:.6}, {:.6} {:.6} {:.6} {:.6}",
        data[0].color.0, data[0].color.1, data[0].color.2, data[0].color.3, data[1].color.0, data[1].color.1, data[1].color.2, data[1].color.3, data[2].color.0, data[2].color.1, data[2].color.2, data[2].color.3, data[3].color.0, data[3].color.1, data[3].color.2, data[3].color.3))
}

/// Deserialize the separator from a string
fn deser_separator<'de, D>(deser: D) -> Result<[Color; 4], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: &str = Deserialize::deserialize(deser)?;
    let mut result: [Color; 4] = [Color::default(); 4];
    let mut max_i = 0;
    for (i, split) in s.split(", ").enumerate() {
        let mut second_split = split.split(' ');
        let first = second_split
            .next()
            .ok_or_else(|| Error::custom("Not enough floats in separator"))?;
        result[i].color.0 = first
            .parse::<f32>()
            .map_err(|_| Error::custom(format!("Could not parse '{first}' as a float!")))?;
        let second = second_split
            .next()
            .ok_or_else(|| Error::custom("Not enough floats in separator"))?;
        result[i].color.1 = second
            .parse::<f32>()
            .map_err(|_| Error::custom(format!("Could not parse '{second}' as a float!")))?;
        let third = second_split
            .next()
            .ok_or_else(|| Error::custom("Not enough floats in separator"))?;
        result[i].color.2 = third
            .parse::<f32>()
            .map_err(|_| Error::custom(format!("Could not parse '{third}' as a float!")))?;
        let fourth = second_split
            .next()
            .ok_or_else(|| Error::custom("Not enough floats in separator"))?;
        result[i].color.3 = fourth
            .parse::<f32>()
            .map_err(|_| Error::custom(format!("Could not parse '{fourth}' as a float!")))?;
        max_i = i;
    }
    if max_i != 3 {
        return Err(Error::custom("max_i is not 3!"));
    }
    Ok(result)
}

macro_rules! impl_deserialize_for_internally_tagged_enum {
    (
        $enum:ty,
        $tag:literal,
        $(
            ($variant_tag:literal => $($variant:tt)+ )
        ),* $(,)?
    ) => {
        impl<'de: 'a, 'a> ::serde::de::Deserialize<'de> for $enum {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                use ::serde::de::{Error, MapAccess, Visitor};

                // The Visitor struct is normally used for state, but none is needed
                #[derive(Default)]
                struct TheVisitor<'a> {
                    _lifetime: ::std::marker::PhantomData<&'a ()>
                }

                // The main logic of the deserializing happens in the Visitor trait
                #[automatically_derived]
                impl<'de: 'a, 'a> Visitor<'de> for TheVisitor<'a> {
                    // The type that is being deserialized
                    type Value = $enum;

                    // Try to give a better error message when this is used wrong
                    fn expecting(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
                        f.write_str("expecting map with tag in ")?;
                        f.write_str($tag)
                    }

                    // The xml data is provided as an opaque map,
                    // that map is parsed into the type
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        // Here the assumption is made that only one attribute
                        // exists and it's the discriminator (enum "tag").
                        let entry: Option<(::std::string::String, ::std::string::String)> = map.next_entry()?;
                        // If there are more attributes those would need
                        // to be parsed as well.
                        let tag = match entry {
                            // Return an error if the no attributes are found,
                            // and indicate that the @tag attribute is missing.
                            None => Err(A::Error::missing_field($tag)),
                            // Check if the attribute is the tag
                            Some((attribute, value)) => {
                                if attribute == $tag {
                                    // return the value of the tag
                                    Ok(value)
                                } else {
                                    // The attribute is not @tag, return an error
                                    // indicating that there is an unexpected attribute
                                    Err(A::Error::unknown_field(&attribute, &[$tag]))
                                }
                            }
                        }?;

                        let de = ::serde::de::value::MapAccessDeserializer::new(map);
                        match tag.as_ref() {
                            $(
                                $variant_tag => Ok($crate::cooked::isc::deserialize_variant!( de, $enum, $($variant)+ )),
                            )*
                            _ => Err(A::Error::unknown_field(&tag, &[$($variant_tag),+])),
                        }
                    }
                }
                // Tell the deserializer to deserialize the data as a map,
                // using the TheVisitor as the decoder
                deserializer.deserialize_map(TheVisitor::default())
            }
        }
    }
}

pub(crate) use impl_deserialize_for_internally_tagged_enum;

macro_rules! deserialize_variant {
    // Produce struct enum variant
    ( $de:expr, $enum:tt, $variant:ident {
        $(
            $(#[$meta:meta])*
            $field:ident : $typ:ty
        ),* $(,)?
    } ) => ({
        let var = {
            // Create anonymous type
            #[derive(::serde::Deserialize)]
            struct $variant {
                $(
                    $(#[$meta])*
                    $field: $typ,
                )*
            }
            <$variant>::deserialize($de)?
        };
        // Due to https://github.com/rust-lang/rust/issues/86935 we cannot use
        // <$enum> :: $variant
        use $enum :: *;
        $variant {
            $($field: var.$field,)*
        }
    });

    // Produce newtype enum variant
    ( $de:expr, $enum:tt, $variant:ident($typ:ty) ) => ({
        let var = <$typ>::deserialize($de)?;
        <$enum> :: $variant(var)
    });

    // Produce unit enum variant
    ( $de:expr, $enum:tt, $variant:ident ) => ({
        ::serde::de::IgnoredAny::deserialize($de)?;
        <$enum> :: $variant
    });
}

pub(crate) use deserialize_variant;

/// Macro to create structs that wrap around other types
///
/// With a wrapping struct:
/// ```xml
/// <values NAME="PropertyPatchValue_String">
///         <PropertyPatchValue_String VALUE="loc ID + ICON" />
/// </values>
/// ```
/// Without a wrapping struct
/// ```xml
/// <values NAME="PropertyPatchValue_String" VALUE="loc ID + ICON" />
/// ```
///
/// # Variants
/// For structs without any members:
/// `wrap!(SomeStructName, "SomeStructName")`
/// For structs without lifetimes:
/// `wrap!(WrappedSomeStructName, SomeStructName, "SomeStructName")`
/// For structs with lifetimes:
/// `wrap!(WrappedSomeStructName, SomeStructName, "SomeStructName", 'a)`
macro_rules! wrap {
    ( $wrapper_struct_name:ident, $rename:literal) => {
        /// Automatically generated empty struct for correct XML parsing
        #[derive(Debug, Clone, Copy, Default, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        #[repr(transparent)]
        pub struct $wrapper_struct_name {
            #[serde(rename = $rename)]
            pub wrapped: (),
        }
    };
    ( $wrapper_struct_name:ident, $wrapped_struct:ident, $rename:literal) => {
        /// Automatically generated wrapper struct for correct XML parsing
        ///
        /// Can automatically be dereferenced to the inner type because it implements [`Deref`].
        /// Can also be automatically constructed and deconstructed from and into the inner type
        /// using the [`From`] implementations.
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        #[repr(transparent)]
        pub struct $wrapper_struct_name {
            #[serde(rename = $rename)]
            pub wrapped: $wrapped_struct,
        }

        impl AsRef<$wrapped_struct> for $wrapper_struct_name {
            fn as_ref(&self) -> &$wrapped_struct {
                &self.wrapped
            }
        }

        impl From<$wrapped_struct> for $wrapper_struct_name {
            fn from(wrapped: $wrapped_struct) -> Self {
                Self { wrapped }
            }
        }

        impl From<$wrapper_struct_name> for $wrapped_struct {
            fn from(value: $wrapper_struct_name) -> Self {
                value.wrapped
            }
        }
    };
    ( $wrapper_struct_name:ident, $wrapped_struct:ident, $rename:literal, $lifetime:lifetime) => {
        /// Automatically generated wrapper struct for correct XML parsing
        ///
        /// Can automatically be dereferenced to the inner type because it implements [`Deref`].
        /// Can also be automatically constructed and deconstructed from and into the inner type
        /// using the [`From`] implementations.
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        #[repr(transparent)]
        pub struct $wrapper_struct_name<$lifetime> {
            #[serde(borrow, rename = $rename)]
            pub wrapped: $wrapped_struct<$lifetime>,
        }

        impl<$lifetime> AsRef<$wrapped_struct<$lifetime>> for $wrapper_struct_name<$lifetime> {
            fn as_ref(&self) -> &$wrapped_struct<$lifetime> {
                &self.wrapped
            }
        }

        impl<$lifetime> From<$wrapped_struct<$lifetime>> for $wrapper_struct_name<$lifetime> {
            fn from(wrapped: $wrapped_struct<$lifetime>) -> Self {
                Self { wrapped }
            }
        }

        impl<$lifetime> From<$wrapper_struct_name<$lifetime>> for $wrapped_struct<$lifetime> {
            fn from(value: $wrapper_struct_name<$lifetime>) -> Self {
                value.wrapped
            }
        }
    };
}

pub(crate) use wrap;
pub use wrapped_actors::*;
mod wrapped_actors {
    #![allow(
        clippy::wildcard_imports,
        clippy::module_name_repetitions,
        reason = "too many imports"
    )]
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(tag = "@NAME", deny_unknown_fields)]
    pub enum WrappedActors<'a> {
        #[serde(rename = "SubSceneActor")]
        SubSceneActor(WrappedSubSceneActor<'a>),
        #[serde(rename = "Actor")]
        Actor(WrappedActor<'a>),
    }

    impl<'a> WrappedActors<'a> {
        /// Convert this Actors to a `Actor`.
        pub fn actor(&'a self) -> Result<&'a Actor<'a>, ParserError> {
            if let WrappedActors::Actor(actor) = self {
                Ok(&actor.actor)
            } else {
                Err(ParserError::custom(format!(
                    "Actor not found in WrappedActors: {self:?}"
                )))
            }
        }

        /// Convert this Actors to a `SubSceneActor`.
        pub fn sub_scene_actor(&'a self) -> Result<&'a SubSceneActor<'a>, ParserError> {
            if let WrappedActors::SubSceneActor(ss_actor) = self {
                Ok(&ss_actor.sub_scene_actor)
            } else {
                Err(ParserError::custom(format!(
                    "Actor not found in WrappedActors: {self:?}"
                )))
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase", deny_unknown_fields)]
    #[repr(transparent)]
    pub struct WrappedSubSceneActor<'a> {
        #[serde(borrow)]
        pub sub_scene_actor: Box<SubSceneActor<'a>>,
    }

    impl<'a> AsRef<SubSceneActor<'a>> for WrappedSubSceneActor<'a> {
        fn as_ref(&self) -> &SubSceneActor<'a> {
            &self.sub_scene_actor
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase", deny_unknown_fields)]
    #[repr(transparent)]
    pub struct WrappedActor<'a> {
        #[serde(borrow)]
        pub actor: Box<Actor<'a>>,
    }

    impl<'a> AsRef<Actor<'a>> for WrappedActor<'a> {
        fn as_ref(&self) -> &Actor<'a> {
            &self.actor
        }
    }

    impl_deserialize_for_internally_tagged_enum! {
        WrappedActors<'a>, "@NAME",
        ("SubSceneActor" => SubSceneActor(WrappedSubSceneActor<'a>)),
        ("Actor" => Actor(WrappedActor<'a>)),
    }
}

pub use wrapped_jd_scene_config::*;
mod wrapped_jd_scene_config {
    #![allow(
        clippy::wildcard_imports,
        clippy::module_name_repetitions,
        reason = "too many imports"
    )]
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(tag = "@NAME", deny_unknown_fields)]
    pub enum WrappedJdSceneConfig<'a> {
        #[serde(rename = "JD_MapSceneConfig")]
        Map(WrappedMapSceneConfig<'a>),
        #[serde(rename = "JD_SongDatabaseSceneConfig")]
        SongDatabase(WrappedSongDatabaseSceneConfig<'a>),
        #[serde(rename = "JD_TransitionSceneConfig")]
        Transition(WrappedTransitionSceneConfig<'a>),
        #[serde(rename = "JD_UIBannerSceneConfig")]
        UIBanner(WrappedUIBannerSceneConfig<'a>),
    }

    wrap!(WrappedMapSceneConfig, MapSceneConfig, "JD_MapSceneConfig", 'a);
    wrap!(WrappedSongDatabaseSceneConfig, SongDatabaseSceneConfig, "JD_SongDatabaseSceneConfig", 'a);
    wrap!(WrappedTransitionSceneConfig, TransitionSceneConfig, "JD_TransitionSceneConfig", 'a);
    wrap!(WrappedUIBannerSceneConfig, UIBannerSceneConfig, "JD_UIBannerSceneConfig", 'a);

    impl_deserialize_for_internally_tagged_enum! {
        WrappedJdSceneConfig<'a>, "@NAME",
        ("JD_MapSceneConfig" => Map(WrappedMapSceneConfig)),
        ("JD_SongDatabaseSceneConfig" => SongDatabase(WrappedSongDatabaseSceneConfig)),
        ("JD_TransitionSceneConfig" => Transition(WrappedTransitionSceneConfig)),
        ("JD_UIBannerSceneConfig" => UIBanner(WrappedUIBannerSceneConfig)),
    }
}

pub use wrapped_component::*;
mod wrapped_component {
    #![allow(
        clippy::wildcard_imports,
        clippy::module_name_repetitions,
        reason = "too many imports"
    )]
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(tag = "@NAME", deny_unknown_fields)]
    pub enum WrappedComponent<'a> {
        #[serde(rename = "AFXPostProcessComponent")]
        AFXPostProcess(WrappedAFXPostProcessComponent),
        #[serde(rename = "BezierBranchFxComponent")]
        BezierBranchFx(BezierBranchFx),
        #[serde(rename = "BezierTreeComponent")]
        BezierTree(WrappedBezierTreeComponent<'a>),
        #[serde(rename = "JD_BlockFlowComponent")]
        BlockFlowComponent(BlockFlowComponent),
        #[serde(rename = "BoxInterpolatorComponent")]
        BoxInterpolator(WrappedBoxInterpolatorComponent),
        #[serde(rename = "JD_Carousel")]
        Carousel(WrappedCarousel<'a>),
        #[serde(rename = "JD_CameraFeedComponent")]
        CameraFeedComponent(WrappedCameraFeedComponent<'a>),
        #[serde(rename = "CameraGraphicComponent")]
        CameraGraphic(WrappedCameraGraphicComponent<'a>),
        #[serde(rename = "ClearColorComponent")]
        ClearColor(WrappedClearColorComponent),
        #[serde(rename = "JD_CreditsComponent")]
        Credits(WrappedCreditsComponent<'a>),
        #[serde(rename = "ConvertedTmlTape_Component")]
        ConvertedTmlTape(WrappedConvertedTmlTapeComponent<'a>),
        #[serde(rename = "FxBankComponent")]
        FxBank(WrappedFxBankComponent<'a>),
        #[serde(rename = "FXControllerComponent")]
        FXController(WrappedFXControllerComponent),
        #[serde(rename = "JD_AutodanceComponent")]
        Autodance(Autodance),
        #[serde(rename = "JD_AvatarDescComponent")]
        AvatarDesc(AvatarDesc),
        #[serde(rename = "JD_CMU_GenericStage_Component")]
        CMUGenericStage(CMUGenericStage),
        #[serde(rename = "JD_FixedCameraComponent")]
        FixedCamera(WrappedFixedCameraComponent),
        #[serde(rename = "JD_GachaComponent")]
        Gacha(Gacha),
        #[serde(rename = "JD_GoldMoveComponent")]
        GoldMove(GoldMove),
        #[serde(rename = "JD_Grid_RegularPatterned")]
        GridRegularPatterned(WrappedGridRegularPatterned<'a>),
        #[serde(rename = "JD_Grid_CustomPatterned")]
        GridCustomPatterned(WrappedGridCustomPatterned<'a>),
        #[serde(rename = "JD_LineGrid")]
        LineGrid(WrappedLineGrid<'a>),
        #[serde(rename = "JD_UILineGrid")]
        UILineGrid(WrappedUILineGrid<'a>),
        #[serde(rename = "JD_UIGrid")]
        UIGrid(WrappedUIGrid<'a>),
        #[serde(rename = "JD_AnthologyGrid")]
        AnthologyGrid(WrappedAnthologyGrid<'a>),
        #[serde(rename = "JD_PleoInfoComponent")]
        PleoInfo(PleoInfo),
        #[serde(rename = "JD_WDFTeamBattleTransitionComponent")]
        WDFTeamBattleTransitionComponent(WrappedWDFTeamBattleTransitionComponent<'a>),
        #[serde(rename = "JD_BeatPulseComponent")]
        BeatPulseComponent(WrappedBeatPulseComponent<'a>),
        #[serde(rename = "JD_PictoTimeline")]
        PictoTimeline(WrappedPictoTimeline<'a>),
        #[serde(rename = "UIItemTextField")]
        UIItemTextField(WrappedUIItemTextField<'a>),
        #[serde(rename = "JD_NotificationBubble")]
        NotificationBubble(NotificationBubble),
        #[serde(rename = "JD_NotificationBubblesPile")]
        NotificationBubblesPile(NotificationBubblesPile),
        #[serde(rename = "JD_RegistrationComponent")]
        Registration(WrappedRegistrationComponent<'a>),
        #[serde(rename = "JD_ScrollingTextComponent")]
        ScrollingText(ScrollingText),
        #[serde(rename = "JD_SkinDescComponent")]
        SkinDesc(SkinDesc),
        #[serde(rename = "JD_SongDatabaseComponent")]
        SongDatabase(SongDatabase),
        #[serde(rename = "JD_SongDescComponent")]
        SongDesc(SongDesc),
        #[serde(rename = "JD_StickerGrid")]
        StickerGrid(WrappedStickerGrid<'a>),
        #[serde(rename = "JD_PictoComponent")]
        Picto(Picto),
        #[serde(rename = "JD_SubtitleComponent")]
        Subtitle(Subtitle),
        #[serde(rename = "JD_UIAvatarUnlockWidget")]
        UIAvatarUnlockWidget(UIAvatarUnlockWidget),
        #[serde(rename = "JD_UIHudCoopFeedbackComponent")]
        UIHudCoopFeedback(UIHudCoopFeedback),
        #[serde(rename = "JD_UIHudLyricsComponent")]
        UIHudLyrics(UIHudLyrics),
        #[serde(rename = "JD_UIHudPictoComponent")]
        UIHudPicto(UIHudPicto),
        #[serde(rename = "JD_UIHudPictolineComponent")]
        UIHudPictoline(UIHudPictoline),
        #[serde(rename = "JD_UIHudRacelineCoopComponent")]
        UIHudRacelineCoop(UIHudRacelineCoop),
        #[serde(rename = "JD_UIHudRacelineGaugeBarComponent")]
        UIHudRacelineGaugeBar(UIHudRacelineGaugeBar),
        #[serde(rename = "JD_UIHudRacelineGaugeComponent")]
        UIHudRacelineGauge(UIHudRacelineGauge),
        #[serde(rename = "JD_UIHudRacelineRivalBarComponent")]
        UIHudRacelineRivalBar(UIHudRacelineRivalBar),
        #[serde(rename = "JD_UIHudRacelineWDFBossComponent")]
        UIHudRacelineWDFBoss(UIHudRacelineWDFBoss),
        #[serde(rename = "JD_UIHudRacelineWDFRankComponent")]
        UIHudRacelineWDFRank(UIHudRacelineWDFRank),
        #[serde(rename = "JD_UIHudRacelineWDFSpotlightComponent")]
        UIHudRacelineWDFSpotlight(UIHudRacelineWDFSpotlight),
        #[serde(rename = "JD_UIHudRacelineWDFTeamBattleComponent")]
        UIHudRaceLineWDFTeamBattle(UIHudRaceLineWDFTeamBattle),
        #[serde(rename = "JD_UIHudShowtimePhotoFeedbackComponent")]
        UIHudShowtimePhotoFeedbackComponent(UIHudShowtimePhotoFeedbackComponent),
        #[serde(rename = "JD_UIHudStarvingComponent")]
        UIHudStarving(UIHudStarving),
        #[serde(rename = "JD_UIHudSweatTimer")]
        UIHudSweatTimer(UIHudSweatTimer),
        #[serde(rename = "JD_UIHudWDFIngameNotificationComponent")]
        UIHudWDFIngameNotification(UIHudWDFIngameNotification),
        #[serde(rename = "JD_UIJoyconWidget")]
        UIJoyconWidget(UIJoyconWidget),
        #[serde(rename = "JD_UIMojoWidget")]
        UIMojoWidget(UIMojoWidget),
        #[serde(rename = "JD_UISaveWidget")]
        UISaveWidget(UISaveWidget),
        #[serde(rename = "JD_UIScheduledQuestComponent")]
        UIScheduledQuest(UIScheduledQuest),
        #[serde(rename = "JD_WDFTransitionComponent")]
        WDFTransitionComponent(WDFTransitionComponent),
        #[serde(rename = "JD_WDFUnlimitedFeedbackComponent")]
        WDFUnlimitedFeedback(WDFUnlimitedFeedback),
        #[serde(rename = "JD_UIHudPlayerComponent")]
        UIHudPlayer(UIHudPlayer),
        #[serde(rename = "MasterTape")]
        MasterTape(MasterTape),
        #[serde(rename = "MaterialGraphicComponent")]
        MaterialGraphic(WrappedMaterialGraphicComponent<'a>),
        #[serde(rename = "MusicTrackComponent")]
        MusicTrack(MusicTrackComponent),
        #[serde(rename = "PleoComponent")]
        Pleo(WrappedPleoComponent<'a>),
        #[serde(rename = "PleoTextureGraphicComponent")]
        PleoTextureGraphic(WrappedPleoTextureGraphicComponent<'a>),
        #[serde(rename = "PropertyPatcher")]
        PropertyPatcher(WrappedPropertyPatcher<'a>),
        #[serde(rename = "JD_SceneSpawnerComponent")]
        SceneSpawner(WrappedSceneSpawnerComponent<'a>),
        #[serde(rename = "JD_ScrollBarComponent")]
        ScrollBar(WrappedScrollBarComponent),
        #[serde(rename = "SingleInstanceMesh3DComponent")]
        SingleInstanceMesh3D(WrappedSingleInstanceMesh3DComponent<'a>),
        #[serde(rename = "Mesh3DComponent")]
        Mesh3D(WrappedMesh3DComponent<'a>),
        #[serde(rename = "SoundComponent")]
        Sound(Sound),
        #[serde(rename = "TapeCase_Component")]
        TapeCase(TapeCase),
        #[serde(rename = "JD_UIUplayNotification")]
        UIUplayNotification(UIUplayNotification),
        #[serde(rename = "JD_UIHudSpotlightPlayerComponent")]
        UIHudSpotlightPlayerComponent(UIHudSpotlightPlayerComponent),
        #[serde(rename = "JD_UIHudLyricsFeedbackComponent")]
        UIHudLyricsFeedbackComponent(UIHudLyricsFeedbackComponent),
        #[serde(rename = "JD_UIHudCamerafeedComponent")]
        UIHudCamerafeedComponent(UIHudCamerafeedComponent),
        #[serde(rename = "JD_UIHudProgressComponent")]
        UIHudProgressComponent(UIHudProgressComponent),
        #[serde(rename = "JD_UIHudCommunityDancerCardComponent")]
        UIHudCommunityDancerCardComponent(UIHudCommunityDancerCardComponent),
        #[serde(rename = "JD_UIHudRacelineRivalComponent")]
        UIHudRacelineRivalComponent(UIHudRacelineRivalComponent),
        #[serde(rename = "JD_WDFOnlineRankTransitionComponent")]
        WDFOnlineRankTransitionComponent(WDFOnlineRankTransitionComponent),
        #[serde(rename = "JD_AliasUnlockNotification")]
        AliasUnlockNotification(AliasUnlockNotification),
        #[serde(rename = "JD_UIHudDoubleScoringPlayerComponent")]
        UIHudDoubleScoringPlayerComponent(UIHudDoubleScoringPlayerComponent),
        #[serde(rename = "JD_UIProfileStatWidget")]
        UIProfileStatWidget(UIProfileStatWidget),
        #[serde(rename = "JD_UIJDRankWidget")]
        UIJDRankWidget(UIJDRankWidget),
        #[serde(rename = "JD_ScrollingPopupComponent")]
        ScrollingPopupComponent(ScrollingPopupComponent),
        #[serde(rename = "JD_UISkinUnlockWidget")]
        UISkinUnlockWidget(UISkinUnlockWidget),
        #[serde(rename = "JD_UIHudVumeterComponent")]
        UIHudVumeterComponent(UIHudVumeterComponent),
        #[serde(rename = "TextureGraphicComponent")]
        TextureGraphic(WrappedTextureGraphicComponent<'a>),
        #[serde(rename = "TexturePatcherComponent")]
        TexturePatcher(WrappedTexturePatcherComponent<'a>),
        #[serde(rename = "UIComponent")]
        UI(UI),
        #[serde(rename = "UIAnchor")]
        UIAnchor(WrappedUIAnchor),
        #[serde(rename = "UICarousel")]
        UICarousel(WrappedUICarousel<'a>),
        #[serde(rename = "UIChangePage")]
        UIChangePage(WrappedUIChangePage<'a>),
        #[serde(rename = "UIControl")]
        UIControl(WrappedUIControl<'a>),
        #[serde(rename = "UICountdown")]
        UICountdown(WrappedUICountdown),
        #[serde(rename = "JD_UIHudAutodanceRecorderComponent")]
        UIHudAutodanceRecorder(WrappedUIHudAutodanceRecorderComponent),
        #[serde(rename = "JD_UIHudSweatCounter")]
        UIHudSweatCounter(WrappedUIHudSweatCounter<'a>),
        #[serde(rename = "UINineSliceComponent")]
        UINineSlice(WrappedUINineSliceComponent<'a>),
        #[serde(rename = "UIItemSlot")]
        UIItemSlot(WrappedUIItemSlot),
        #[serde(rename = "UINineSliceMaskComponent")]
        UINineSliceMask(WrappedUINineSliceMaskComponent<'a>),
        #[serde(rename = "UIPhoneData")]
        UIPhoneData(WrappedUIPhoneData<'a>),
        #[serde(rename = "UIRootComponent")]
        UIRoot(WrappedUIRootComponent),
        #[serde(rename = "UIScreenComponent")]
        UIScreen(WrappedUIScreenComponent<'a>),
        #[serde(rename = "UITextBox")]
        UITextBox(WrappedUITextBox<'a>),
        #[serde(rename = "JD_UIUploadIcon")]
        UIUploadIcon(WrappedUIUploadIcon),
        #[serde(rename = "JD_UIHudRacelineDM")]
        UIHudRacelineDM(WrappedUIHudRacelineDM),
        #[serde(rename = "JD_UIWidgetGroupHUD")]
        UIWidgetGroupHUD(WrappedUIWidgetGroupHUD<'a>),
        #[serde(rename = "JD_UIWidgetGroupHUD_AutodanceRecorder")]
        UIWidgetGroupHUDAutodanceRecorder(WrappedUIWidgetGroupHUDAutodanceRecorder<'a>),
        #[serde(rename = "JD_UIWidgetGroupHUD_Lyrics")]
        UIWidgetGroupHUDLyrics(WrappedUIWidgetGroupHUDLyrics<'a>),
        #[serde(rename = "JD_UIWidgetGroupHUD_PauseIcon")]
        UIWidgetGroupHUDPauseIcon(WrappedUIWidgetGroupHUDPauseIcon<'a>),
        #[serde(rename = "JD_UIHudVersusPlayerComponent")]
        UIHudVersusPlayer(WrappedUIHudVersusPlayerComponent<'a>),
        #[serde(rename = "ViewportUIComponent")]
        ViewportUI(WrappedViewportUIComponent<'a>),
        #[serde(rename = "JD_WDFBossSpawnerComponent")]
        WDFBossSpawner(WrappedWDFBossSpawnerComponent<'a>),
        #[serde(rename = "JD_WDFTeamBattlePresentationComponent")]
        WDFTeamBattlePresentation(WrappedWDFTeamBattlePresentationComponent<'a>),
        #[serde(rename = "JD_WDFThemePresentationComponent")]
        WDFThemePresentation(WrappedWDFThemePresentationComponent<'a>),
    }

    impl<'b> WrappedComponent<'b> {
        /// Convert this component to a `PleoComponent`.
        pub fn pleo_component<'a>(&'a self) -> Result<&'a PleoComponent<'b>, ParserError> {
            if let Self::Pleo(pleo_component) = self {
                Ok(&pleo_component.wrapped)
            } else {
                Err(ParserError::custom(format!(
                    "No PleoComponent in Component: {self:?}"
                )))
            }
        }

        /// Convert this component to a `MaterialGraphicComponent`.
        pub fn material_graphic_component<'a>(
            &'a self,
        ) -> Result<&'a MaterialGraphicComponent<'b>, ParserError> {
            if let Self::MaterialGraphic(material_graphic_component) = self {
                Ok(&material_graphic_component.wrapped)
            } else {
                Err(ParserError::custom(format!(
                    "No MaterialGraphicComponent in Component: {self:?}"
                )))
            }
        }
    }

    wrap!(
        WrappedAFXPostProcessComponent,
        AFXPostProcessComponent,
        "AFXPostProcessComponent"
    );
    wrap!(BezierBranchFx, "BezierBranchFxComponent");
    wrap!(WrappedBezierTreeComponent, BezierTreeComponent, "BezierTreeComponent", 'a);
    wrap!(BlockFlowComponent, "JD_BlockFlowComponent");
    wrap!(
        WrappedBoxInterpolatorComponent,
        BoxInterpolatorComponent,
        "BoxInterpolatorComponent"
    );
    wrap!(WrappedCarousel, Carousel, "JD_Carousel", 'a);
    wrap!(WrappedCameraFeedComponent, CameraFeedComponent, "JD_CameraFeedComponent", 'a);
    wrap!(WrappedCameraGraphicComponent, MaterialGraphicComponent, "CameraGraphicComponent", 'a);
    wrap!(
        WrappedClearColorComponent,
        ClearColorComponent,
        "ClearColorComponent"
    );
    wrap!(WrappedCreditsComponent, CreditsComponent, "JD_CreditsComponent", 'a);
    wrap!(WrappedConvertedTmlTapeComponent, ConvertedTmlTapeComponent, "ConvertedTmlTape_Component", 'a);
    wrap!(WrappedFxBankComponent, FxBankComponent, "FxBankComponent", 'a);
    wrap!(
        WrappedFXControllerComponent,
        FXControllerComponent,
        "FXControllerComponent"
    );
    wrap!(Autodance, "JD_AutodanceComponent");
    wrap!(AvatarDesc, "JD_AvatarDescComponent");
    wrap!(CMUGenericStage, "JD_CMU_GenericStage_Component");
    wrap!(
        WrappedFixedCameraComponent,
        FixedCameraComponent,
        "JD_FixedCameraComponent"
    );
    wrap!(Gacha, "JD_GachaComponent");
    wrap!(GoldMove, "JD_GoldMoveComponent");
    wrap!(WrappedGridRegularPatterned, Grid, "JD_Grid_RegularPatterned", 'a);
    wrap!(WrappedGridCustomPatterned, Grid, "JD_Grid_CustomPatterned", 'a);
    wrap!(WrappedLineGrid, LineGrid, "JD_LineGrid", 'a);
    wrap!(WrappedUILineGrid, UIGrid, "JD_UILineGrid", 'a);
    wrap!(WrappedUIGrid, UIGrid, "JD_UIGrid", 'a);
    wrap!(WrappedAnthologyGrid, AnthologyGrid, "JD_AnthologyGrid", 'a);
    wrap!(PleoInfo, "JD_PleoInfoComponent");
    wrap!(WrappedWDFTeamBattleTransitionComponent, WDFTeamBattleTransitionComponent, "JD_WDFTeamBattleTransitionComponent", 'a);
    wrap!(WrappedBeatPulseComponent, BeatPulseComponent, "JD_BeatPulseComponent", 'a);
    wrap!(WrappedPictoTimeline, PictoTimeline, "JD_PictoTimeline", 'a);
    wrap!(WrappedUIItemTextField, UIItemTextField, "UIItemTextField", 'a);
    wrap!(NotificationBubble, "JD_NotificationBubble");
    wrap!(NotificationBubblesPile, "JD_NotificationBubblesPile");
    wrap!(WrappedRegistrationComponent, RegistrationComponent, "JD_RegistrationComponent", 'a);
    wrap!(ScrollingText, "JD_ScrollingTextComponent");
    wrap!(SkinDesc, "JD_SkinDescComponent");
    wrap!(SongDatabase, "JD_SongDatabaseComponent");
    wrap!(SongDesc, "JD_SongDescComponent");
    wrap!(WrappedStickerGrid, Grid, "JD_StickerGrid", 'a);
    wrap!(Picto, "JD_PictoComponent");
    wrap!(Subtitle, "JD_SubtitleComponent");
    wrap!(UIAvatarUnlockWidget, "JD_UIAvatarUnlockWidget");
    wrap!(UIHudCoopFeedback, "JD_UIHudCoopFeedbackComponent");
    wrap!(UIHudLyrics, "JD_UIHudLyricsComponent");
    wrap!(UIHudPicto, "JD_UIHudPictoComponent");
    wrap!(UIHudPictoline, "JD_UIHudPictolineComponent");
    wrap!(UIHudRacelineCoop, "JD_UIHudRacelineCoopComponent");
    wrap!(UIHudRacelineGaugeBar, "JD_UIHudRacelineGaugeBarComponent");
    wrap!(UIHudRacelineGauge, "JD_UIHudRacelineGaugeComponent");
    wrap!(UIHudRacelineRivalBar, "JD_UIHudRacelineRivalBarComponent");
    wrap!(UIHudRacelineWDFBoss, "JD_UIHudRacelineWDFBossComponent");
    wrap!(UIHudRacelineWDFRank, "JD_UIHudRacelineWDFRankComponent");
    wrap!(
        UIHudRacelineWDFSpotlight,
        "JD_UIHudRacelineWDFSpotlightComponent"
    );
    wrap!(
        UIHudRaceLineWDFTeamBattle,
        "JD_UIHudRacelineWDFTeamBattleComponent"
    );
    wrap!(
        UIHudShowtimePhotoFeedbackComponent,
        "JD_UIHudShowtimePhotoFeedbackComponent"
    );
    wrap!(UIHudStarving, "JD_UIHudStarvingComponent");
    wrap!(UIHudSweatTimer, "JD_UIHudSweatTimer");
    wrap!(
        UIHudWDFIngameNotification,
        "JD_UIHudWDFIngameNotificationComponent"
    );
    wrap!(UIJoyconWidget, "JD_UIJoyconWidget");
    wrap!(UIMojoWidget, "JD_UIMojoWidget");
    wrap!(UISaveWidget, "JD_UISaveWidget");
    wrap!(UIScheduledQuest, "JD_UIScheduledQuestComponent");
    wrap!(WDFTransitionComponent, "JD_WDFTransitionComponent");
    wrap!(WDFUnlimitedFeedback, "JD_WDFUnlimitedFeedbackComponent");
    wrap!(UIHudPlayer, "JD_UIHudPlayerComponent");
    wrap!(MasterTape, "MasterTape");
    wrap!(WrappedMaterialGraphicComponent, MaterialGraphicComponent, "MaterialGraphicComponent", 'a);
    wrap!(MusicTrackComponent, "MusicTrackComponent");
    wrap!(WrappedPleoComponent, PleoComponent, "PleoComponent", 'a);
    wrap!(WrappedPleoTextureGraphicComponent, PleoTextureGraphicComponent, "PleoTextureGraphicComponent", 'a);
    wrap!(WrappedSceneSpawnerComponent, SceneSpawnerComponent, "JD_SceneSpawnerComponent", 'a);
    wrap!(
        WrappedScrollBarComponent,
        ScrollBarComponent,
        "JD_ScrollBarComponent"
    );
    wrap!(WrappedSingleInstanceMesh3DComponent, Mesh3DComponent, "SingleInstanceMesh3DComponent", 'a);
    wrap!(WrappedMesh3DComponent, Mesh3DComponent, "Mesh3DComponent", 'a);
    wrap!(Sound, "SoundComponent");
    wrap!(TapeCase, "TapeCase_Component");
    wrap!(UIUplayNotification, "JD_UIUplayNotification");
    wrap!(
        UIHudSpotlightPlayerComponent,
        "JD_UIHudSpotlightPlayerComponent"
    );
    wrap!(
        UIHudLyricsFeedbackComponent,
        "JD_UIHudLyricsFeedbackComponent"
    );
    wrap!(UIHudCamerafeedComponent, "JD_UIHudCamerafeedComponent");
    wrap!(UIHudProgressComponent, "JD_UIHudProgressComponent");
    wrap!(
        UIHudCommunityDancerCardComponent,
        "JD_UIHudCommunityDancerCardComponent"
    );
    wrap!(
        UIHudRacelineRivalComponent,
        "JD_UIHudRacelineRivalComponent"
    );
    wrap!(
        WDFOnlineRankTransitionComponent,
        "JD_WDFOnlineRankTransitionComponent"
    );
    wrap!(AliasUnlockNotification, "JD_AliasUnlockNotification");
    wrap!(
        UIHudDoubleScoringPlayerComponent,
        "JD_UIHudDoubleScoringPlayerComponent"
    );
    wrap!(UIProfileStatWidget, "JD_UIProfileStatWidget");
    wrap!(UIJDRankWidget, "JD_UIJDRankWidget");
    wrap!(ScrollingPopupComponent, "JD_ScrollingPopupComponent");
    wrap!(UISkinUnlockWidget, "JD_UISkinUnlockWidget");
    wrap!(UIHudVumeterComponent, "JD_UIHudVumeterComponent");
    wrap!(WrappedTextureGraphicComponent, TextureGraphicComponent, "TextureGraphicComponent", 'a);
    wrap!(WrappedTexturePatcherComponent, TexturePatcherComponent, "TexturePatcherComponent", 'a);
    wrap!(UI, "UIComponent");
    wrap!(WrappedUIAnchor, UIAnchor, "UIAnchor");
    wrap!(WrappedUICarousel, UICarousel, "UICarousel", 'a);
    wrap!(WrappedUIChangePage, UIChangePage, "UIChangePage", 'a);
    wrap!(WrappedUIControl, UIControl, "UIControl", 'a);
    wrap!(WrappedUICountdown, UICountdown, "UICountdown");
    wrap!(
        WrappedUIHudAutodanceRecorderComponent,
        UIHudAutodanceRecorderComponent,
        "JD_UIHudAutodanceRecorderComponent"
    );
    wrap!(WrappedUIHudSweatCounter, UIHudSweatCounter, "JD_UIHudSweatCounter", 'a);
    wrap!(WrappedUINineSliceComponent, UINineSliceComponent, "UINineSliceComponent", 'a);
    wrap!(WrappedUIItemSlot, UIItemSlot, "UIItemSlot");
    wrap!(WrappedUINineSliceMaskComponent, UINineSliceMaskComponent, "UINineSliceMaskComponent", 'a);
    wrap!(WrappedUIPhoneData, UIPhoneData, "UIPhoneData", 'a);
    wrap!(WrappedUIRootComponent, UIRootComponent, "UIRootComponent");
    wrap!(WrappedUIScreenComponent, UIScreenComponent, "UIScreenComponent", 'a);
    wrap!(WrappedUITextBox, UITextBox, "UITextBox", 'a);
    wrap!(WrappedUIUploadIcon, UIUploadIcon, "JD_UIUploadIcon");
    wrap!(
        WrappedUIHudRacelineDM,
        UIHudRacelineDM,
        "JD_UIHudRacelineDM"
    );
    wrap!(WrappedUIWidgetGroupHUD, UIWidgetGroupHUD, "JD_UIWidgetGroupHUD", 'a);
    wrap!(WrappedUIWidgetGroupHUDAutodanceRecorder, UIWidgetGroupHUDAutodanceRecorder, "JD_UIWidgetGroupHUD_AutodanceRecorder", 'a);
    wrap!(WrappedUIWidgetGroupHUDLyrics, UIWidgetGroupHUDLyrics, "JD_UIWidgetGroupHUD_Lyrics", 'a);
    wrap!(WrappedUIWidgetGroupHUDPauseIcon, UIWidgetGroupHUDPauseIcon, "JD_UIWidgetGroupHUD_PauseIcon", 'a);
    wrap!(WrappedUIHudVersusPlayerComponent, UIHudVersusPlayerComponent, "JD_UIHudVersusPlayerComponent", 'a);
    wrap!(WrappedViewportUIComponent, ViewportUIComponent, "ViewportUIComponent", 'a);
    wrap!(WrappedWDFBossSpawnerComponent, WDFBossSpawnerComponent, "JD_WDFBossSpawnerComponent", 'a);
    wrap!(WrappedWDFTeamBattlePresentationComponent, WDFTeamBattlePresentationComponent, "JD_WDFTeamBattlePresentationComponent", 'a);
    wrap!(WrappedWDFThemePresentationComponent, WDFThemePresentationComponent, "JD_WDFThemePresentationComponent", 'a);

    impl_deserialize_for_internally_tagged_enum! {
        WrappedComponent<'a>, "@NAME",
        ("AFXPostProcessComponent" => AFXPostProcess(WrappedAFXPostProcessComponent)),
        ("BezierBranchFxComponent" => BezierBranchFx(BezierBranchFx)),
        ("BezierTreeComponent" => BezierTree(WrappedBezierTreeComponent)),
        ("BoxInterpolatorComponent" => BoxInterpolator(WrappedBoxInterpolatorComponent)),
        ("CameraGraphicComponent" => CameraGraphic(WrappedCameraGraphicComponent)),
        ("ClearColorComponent" => ClearColor(WrappedClearColorComponent)),
        ("ConvertedTmlTape_Component" => ConvertedTmlTape(WrappedConvertedTmlTapeComponent)),
        ("FxBankComponent" => FxBank(WrappedFxBankComponent)),
        ("FXControllerComponent" => FXController(WrappedFXControllerComponent)),
        ("JD_AutodanceComponent" => Autodance(Autodance)),
        ("JD_AvatarDescComponent" => AvatarDesc(AvatarDesc)),
        ("JD_BlockFlowComponent" => BlockFlowComponent(BlockFlowComponent)),
        ("JD_Carousel" => Carousel(WrappedCarousel)),
        ("JD_CameraFeedComponent" => CameraFeedComponent(WrappedCameraFeedComponent)),
        ("JD_CMU_GenericStage_Component" => CMUGenericStage(CMUGenericStage)),
        ("JD_CreditsComponent" => Credits(WrappedCreditsComponent)),
        ("JD_FixedCameraComponent" => FixedCamera(WrappedFixedCameraComponent)),
        ("JD_GachaComponent" => Gacha(Gacha)),
        ("JD_GoldMoveComponent" => GoldMove(GoldMove)),
        ("JD_Grid_CustomPatterned" => GridCustomPatterned(WrappedGridCustomPatterned)),
        ("JD_Grid_RegularPatterned" => GridRegularPatterned(WrappedGridRegularPatterned)),
        ("JD_LineGrid" => LineGrid(WrappedLineGrid)),
        ("JD_UILineGrid" => UILineGrid(WrappedUILineGrid)),
        ("JD_UIGrid" => UIGrid(WrappedUIGrid)),
        ("JD_AnthologyGrid" => AnthologyGrid(WrappedAnthologyGrid)),
        ("JD_NotificationBubble" => NotificationBubble(NotificationBubble)),
        ("JD_NotificationBubblesPile" => NotificationBubblesPile(NotificationBubblesPile)),
        ("JD_PictoComponent" => Picto(Picto)),
        ("JD_PleoInfoComponent" => PleoInfo(PleoInfo)),
        ("JD_RegistrationComponent" => Registration(WrappedRegistrationComponent)),
        ("JD_WDFTeamBattleTransitionComponent" => WDFTeamBattleTransitionComponent(WrappedWDFTeamBattleTransitionComponent)),
        ("JD_BeatPulseComponent" => BeatPulseComponent(WrappedBeatPulseComponent)),
        ("JD_PictoTimeline" => PictoTimeline(WrappedPictoTimeline)),
        ("UIItemTextField" => UIItemTextField(WrappedUIItemTextField)),
        ("JD_SceneSpawnerComponent" => SceneSpawner(WrappedSceneSpawnerComponent)),
        ("JD_ScrollBarComponent" => ScrollBar(WrappedScrollBarComponent)),
        ("JD_ScrollingTextComponent" => ScrollingText(ScrollingText)),
        ("JD_SkinDescComponent" => SkinDesc(SkinDesc)),
        ("JD_SongDatabaseComponent" => SongDatabase(SongDatabase)),
        ("JD_SongDescComponent" => SongDesc(SongDesc)),
        ("JD_StickerGrid" => StickerGrid(WrappedStickerGrid)),
        ("JD_SubtitleComponent" => Subtitle(Subtitle)),
        ("JD_UIAvatarUnlockWidget" => UIAvatarUnlockWidget(UIAvatarUnlockWidget)),
        ("JD_UIHudAutodanceRecorderComponent" => UIHudAutodanceRecorder(WrappedUIHudAutodanceRecorderComponent)),
        ("JD_UIHudCoopFeedbackComponent" => UIHudCoopFeedback(UIHudCoopFeedback)),
        ("JD_UIHudLyricsComponent" => UIHudLyrics(UIHudLyrics)),
        ("JD_UIHudPictoComponent" => UIHudPicto(UIHudPicto)),
        ("JD_UIHudPictolineComponent" => UIHudPictoline(UIHudPictoline)),
        ("JD_UIHudPlayerComponent" => UIHudPlayer(UIHudPlayer)),
        ("JD_UIHudRacelineCoopComponent" => UIHudRacelineCoop(UIHudRacelineCoop)),
        ("JD_UIHudRacelineGaugeBarComponent" => UIHudRacelineGaugeBar(UIHudRacelineGaugeBar)),
        ("JD_UIHudRacelineRivalBarComponent" => UIHudRacelineRivalBar(UIHudRacelineRivalBar)),
        ("JD_UIHudRacelineGaugeComponent" => UIHudRacelineGauge(UIHudRacelineGauge)),
        ("JD_UIHudRacelineWDFBossComponent" => UIHudRacelineWDFBoss(UIHudRacelineWDFBoss)),
        ("JD_UIHudRacelineWDFRankComponent" => UIHudRacelineWDFRank(UIHudRacelineWDFRank)),
        ("JD_UIHudRacelineWDFSpotlightComponent" => UIHudRacelineWDFSpotlight(UIHudRacelineWDFSpotlight)),
        ("JD_UIHudRacelineWDFTeamBattleComponent" => UIHudRaceLineWDFTeamBattle(UIHudRaceLineWDFTeamBattle)),
        ("JD_UIHudShowtimePhotoFeedbackComponent" => UIHudShowtimePhotoFeedbackComponent(UIHudShowtimePhotoFeedbackComponent)),
        ("JD_UIHudStarvingComponent" => UIHudStarving(UIHudStarving)),
        ("JD_UIHudSweatCounter" => UIHudSweatCounter(WrappedUIHudSweatCounter)),
        ("JD_UIHudSweatTimer" => UIHudSweatTimer(UIHudSweatTimer)),
        ("JD_UIHudVersusPlayerComponent" => UIHudVersusPlayer(WrappedUIHudVersusPlayerComponent)),
        ("JD_UIHudWDFIngameNotificationComponent" => UIHudWDFIngameNotification(UIHudWDFIngameNotification)),
        ("JD_UIJoyconWidget" => UIJoyconWidget(UIJoyconWidget)),
        ("JD_UIMojoWidget" => UIMojoWidget(UIMojoWidget)),
        ("JD_UISaveWidget" => UISaveWidget(UISaveWidget)),
        ("JD_UIScheduledQuestComponent" => UIScheduledQuest(UIScheduledQuest)),
        ("JD_UIUploadIcon" => UIUploadIcon(WrappedUIUploadIcon)),
        ("JD_UIWidgetGroupHUD" => UIWidgetGroupHUD(WrappedUIWidgetGroupHUD)),
        ("JD_UIWidgetGroupHUD_AutodanceRecorder" => UIWidgetGroupHUDAutodanceRecorder(WrappedUIWidgetGroupHUDAutodanceRecorder)),
        ("JD_UIWidgetGroupHUD_Lyrics" => UIWidgetGroupHUDLyrics(WrappedUIWidgetGroupHUDLyrics)),
        ("JD_UIWidgetGroupHUD_PauseIcon" => UIWidgetGroupHUDPauseIcon(WrappedUIWidgetGroupHUDPauseIcon)),
        ("JD_WDFBossSpawnerComponent" => WDFBossSpawner(WrappedWDFBossSpawnerComponent)),
        ("JD_WDFTeamBattlePresentationComponent" => WDFTeamBattlePresentation(WrappedWDFTeamBattlePresentationComponent)),
        ("JD_WDFThemePresentationComponent" => WDFThemePresentation(WrappedWDFThemePresentationComponent)),
        ("JD_WDFTransitionComponent" => WDFTransitionComponent(WDFTransitionComponent)),
        ("JD_WDFUnlimitedFeedbackComponent" => WDFUnlimitedFeedback(WDFUnlimitedFeedback)),
        ("SoundComponent" => Sound(Sound)),
        ("MasterTape" => MasterTape(MasterTape)),
        ("JD_UIUplayNotification" => UIUplayNotification(UIUplayNotification)),
        ("JD_UIHudSpotlightPlayerComponent" => UIHudSpotlightPlayerComponent(UIHudSpotlightPlayerComponent)),
        ("JD_UIHudLyricsFeedbackComponent" => UIHudLyricsFeedbackComponent(UIHudLyricsFeedbackComponent)),
        ("JD_UIHudCamerafeedComponent" => UIHudCamerafeedComponent(UIHudCamerafeedComponent)),
        ("JD_UIHudProgressComponent" => UIHudProgressComponent(UIHudProgressComponent)),
        ("JD_UIHudCommunityDancerCardComponent" => UIHudCommunityDancerCardComponent(UIHudCommunityDancerCardComponent)),
        ("JD_UIHudRacelineRivalComponent" => UIHudRacelineRivalComponent(UIHudRacelineRivalComponent)),
        ("JD_WDFOnlineRankTransitionComponent" => WDFOnlineRankTransitionComponent(WDFOnlineRankTransitionComponent)),
        ("JD_AliasUnlockNotification" => AliasUnlockNotification(AliasUnlockNotification)),
        ("JD_UIHudDoubleScoringPlayerComponent" => UIHudDoubleScoringPlayerComponent(UIHudDoubleScoringPlayerComponent)),
        ("JD_UIProfileStatWidget" => UIProfileStatWidget(UIProfileStatWidget)),
        ("JD_UIJDRankWidget" => UIJDRankWidget(UIJDRankWidget)),
        ("JD_ScrollingPopupComponent" => ScrollingPopupComponent(ScrollingPopupComponent)),
        ("JD_UIHudVumeterComponent" => UIHudVumeterComponent(UIHudVumeterComponent)),
        ("JD_UISkinUnlockWidget" => UISkinUnlockWidget(UISkinUnlockWidget)),
        ("MaterialGraphicComponent" => MaterialGraphic(WrappedMaterialGraphicComponent)),
        ("JD_UIHudRacelineDM" => UIHudRacelineDM(WrappedUIHudRacelineDM)),
        ("MusicTrackComponent" => MusicTrack(MusicTrackComponent)),
        ("PleoComponent" => Pleo(WrappedPleoComponent)),
        ("PleoTextureGraphicComponent" => PleoTextureGraphic(WrappedPleoTextureGraphicComponent)),
        ("PropertyPatcher" => PropertyPatcher(WrappedPropertyPatcher)),
        ("SingleInstanceMesh3DComponent" => SingleInstanceMesh3D(WrappedSingleInstanceMesh3DComponent)),
        ("Mesh3DComponent" => Mesh3D(WrappedMesh3DComponent)),
        ("TapeCase_Component" => TapeCase(TapeCase)),
        ("TextureGraphicComponent" => TextureGraphic(WrappedTextureGraphicComponent)),
        ("TexturePatcherComponent" => TexturePatcher(WrappedTexturePatcherComponent)),
        ("UIAnchor" => UIAnchor(WrappedUIAnchor)),
        ("UICarousel" => UICarousel(WrappedUICarousel)),
        ("UIChangePage" => UIChangePage(WrappedUIChangePage)),
        ("UIControl" => UIControl(WrappedUIControl)),
        ("UIComponent" => UI(UI)),
        ("UICountdown" => UICountdown(WrappedUICountdown)),
        ("UIItemSlot" => UIItemSlot(WrappedUIItemSlot)),
        ("UINineSliceComponent" => UINineSlice(WrappedUINineSliceComponent)),
        ("UINineSliceMaskComponent" => UINineSliceMask(WrappedUINineSliceMaskComponent)),
        ("UIPhoneData" => UIPhoneData(WrappedUIPhoneData)),
        ("UIRootComponent" => UIRoot(WrappedUIRootComponent)),
        ("UIScreenComponent" => UIScreen(WrappedUIScreenComponent)),
        ("UITextBox" => UITextBox(WrappedUITextBox)),
        ("ViewportUIComponent" => ViewportUI(WrappedViewportUIComponent)),
    }
}

use ubiart_toolkit_shared_types::errors::ParserError;
pub use wrapped_carousel_behaviour::*;
mod wrapped_carousel_behaviour {
    #![allow(
        clippy::wildcard_imports,
        clippy::module_name_repetitions,
        reason = "too many imports"
    )]
    use super::*;

    #[derive(Debug, Clone, Serialize)]
    #[serde(tag = "@NAME", deny_unknown_fields)]
    pub enum ValWrappedCarouselBehaviour<'a> {
        #[serde(rename = "CarouselBehaviour_Navigation")]
        Navigation(ValWrappedCarouselBehaviourNavigation<'a>),
        #[serde(rename = "CarouselBehaviour_NavigationGrid")]
        NavigationGrid(ValWrappedCarouselBehaviourNavigationGrid<'a>),
        #[serde(rename = "CarouselBehaviour_GoToElement")]
        GoToElement(ValWrappedCarouselBehaviourGoToElement<'a>),
        #[serde(rename = "CarouselBehaviour_AutoScroll")]
        AutoScroll(ValWrappedCarouselBehaviourAutoScroll<'a>),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ValWrappedCarouselBehaviourNavigation<'a> {
        #[serde(rename = "@KEY")]
        pub key: HipStr<'a>,
        #[serde(borrow, rename = "VAL")]
        pub val: WrappedCarouselBehaviourNavigation<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct WrappedCarouselBehaviourNavigation<'a> {
        #[serde(borrow, rename = "CarouselBehaviour_Navigation")]
        pub carousel_behaviour_navigation: CarouselBehaviourNavigation<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ValWrappedCarouselBehaviourNavigationGrid<'a> {
        #[serde(rename = "@KEY")]
        pub key: HipStr<'a>,
        #[serde(borrow, rename = "VAL")]
        pub val: WrappedCarouselBehaviourNavigationGrid<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct WrappedCarouselBehaviourNavigationGrid<'a> {
        #[serde(borrow, rename = "CarouselBehaviour_NavigationGrid")]
        pub carousel_behaviour_navigation_grid: CarouselBehaviourNavigation<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ValWrappedCarouselBehaviourGoToElement<'a> {
        #[serde(rename = "@KEY")]
        pub key: HipStr<'a>,
        #[serde(borrow, rename = "VAL")]
        pub val: WrappedCarouselBehaviourGoToElement<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct WrappedCarouselBehaviourGoToElement<'a> {
        #[serde(borrow, rename = "CarouselBehaviour_GoToElement")]
        pub carousel_behaviour_go_to_element: CarouselBehaviourNavigationGoToElement<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ValWrappedCarouselBehaviourAutoScroll<'a> {
        #[serde(rename = "@KEY")]
        pub key: HipStr<'a>,
        #[serde(borrow, rename = "VAL")]
        pub val: WrappedCarouselBehaviourAutoScroll<'a>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct WrappedCarouselBehaviourAutoScroll<'a> {
        #[serde(borrow, rename = "CarouselBehaviour_AutoScroll")]
        pub carousel_behaviour_go_to_element: CarouselBehaviourNavigationAutoScroll<'a>,
    }

    impl_deserialize_for_internally_tagged_enum! {
        ValWrappedCarouselBehaviour<'a>, "@NAME",
        ("CarouselBehaviour_Navigation" => Navigation(ValWrappedCarouselBehaviourNavigation)),
        ("CarouselBehaviour_NavigationGrid" => NavigationGrid(ValWrappedCarouselBehaviourNavigationGrid)),
        ("CarouselBehaviour_GoToElement" => GoToElement(ValWrappedCarouselBehaviourGoToElement)),
        ("CarouselBehaviour_AutoScroll" => AutoScroll(ValWrappedCarouselBehaviourAutoScroll))
    }
}
