use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::{
    just_dance::{AutoDanceFxDesc, PlaybackEvent},
    v1819::ObjectiveDesc1819,
    DifficultyColors, Empty,
};
use ubiart_toolkit_shared_types::{Color, LocaleId};

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementsDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub achievements: Vec<AchievementDescriptor<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub platform_id: u32,
    pub uplay_id: u32,
    pub unlock_objective_desc_id: Cow<'a, str>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocalAliases<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub locked_color: Cow<'a, str>,
    pub difficulty_colors: DifficultyColors<'a>,
    pub aliases: Vec<UnlockableAliasDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct UnlockableAliasDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u16,
    #[serde(rename = "StringLocID")]
    pub string_loc_id: LocaleId,
    #[serde(rename = "StringLocIDFemale")]
    pub string_loc_id_female: LocaleId,
    pub string_online_localized: Cow<'a, str>,
    pub string_online_localized_female: Cow<'a, str>,
    pub string_placeholder: Cow<'a, str>,
    pub unlocked_by_default: bool,
    #[serde(rename = "DescriptionLocID")]
    pub description_loc_id: LocaleId,
    pub description_localized: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlock_objective: Option<UnlockObjectiveOnlineInfo<'a>>,
    pub difficulty_color: Rarity,
    pub visibility: u8,
}

impl UnlockableAliasDescriptor<'_> {
    pub const CLASS: &'static str = "JD_UnlockableAliasDescriptor";
}

impl Default for UnlockableAliasDescriptor<'static> {
    fn default() -> Self {
        Self {
            class: Some(UnlockableAliasDescriptor::CLASS),
            id: Default::default(),
            string_loc_id: LocaleId::default(),
            string_loc_id_female: LocaleId::default(),
            string_online_localized: Cow::default(),
            string_online_localized_female: Cow::default(),
            string_placeholder: Cow::default(),
            unlocked_by_default: Default::default(),
            description_loc_id: LocaleId::default(),
            description_localized: Cow::default(),
            unlock_objective: Some(UnlockObjectiveOnlineInfo::default()),
            difficulty_color: Rarity::Common,
            visibility: 0,
        }
    }
}

/// How rare is the alias
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Rarity {
    /// Common
    Common = 0,
    /// Uncommon
    Uncommon = 1,
    /// Rare
    Rare = 2,
    /// Epic
    Epic = 3,
    /// Legendary
    Legendary = 4,
    /// Exotic
    Exotic = 5,
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RarityVisitor;

        impl<'de> serde::de::Visitor<'de> for RarityVisitor {
            type Value = Rarity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 5")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(Rarity::Common),
                    1 => Ok(Rarity::Uncommon),
                    2 => Ok(Rarity::Rare),
                    3 => Ok(Rarity::Epic),
                    4 => Ok(Rarity::Legendary),
                    5 => Ok(Rarity::Exotic),
                    _ => Err(E::custom(format!("Rarity is unknown: {v}"))),
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "0" => Ok(Rarity::Common),
                    "1" => Ok(Rarity::Uncommon),
                    "2" => Ok(Rarity::Rare),
                    "3" => Ok(Rarity::Epic),
                    "4" => Ok(Rarity::Legendary),
                    "5" => Ok(Rarity::Exotic),
                    _ => Err(E::custom(format!("Rarity is unknown: {v}"))),
                }
            }

            // Similar for other methods:
            //   - visit_i16
            //   - visit_u8
            //   - visit_u16
            //   - visit_u32
            //   - visit_u64
        }

        deserializer.deserialize_any(RarityVisitor)
    }
}

impl Serialize for Rarity {
    #![allow(
        clippy::as_conversions,
        reason = "Rarity is repr(u8) and thus this is safe"
    )]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", *self as u8))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlockObjectiveOnlineInfo<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub unlock_objective_desc_id: Cow<'a, str>,
}

impl UnlockObjectiveOnlineInfo<'_> {
    pub const CLASS: &'static str = "JD_UnlockObjectiveOnlineInfo";
}

impl Default for UnlockObjectiveOnlineInfo<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            unlock_objective_desc_id: Cow::Borrowed(""),
        }
    }
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub shakes: Vec<CameraShake<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShake<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub intensity: f32,
    pub duration: f32,
    pub ease_in_duration: f32,
    pub ease_out_duration: f32,
    pub shake_x: CameraShakeCurveParams<'a>,
    pub shake_y: CameraShakeCurveParams<'a>,
    pub shake_z: CameraShakeCurveParams<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeCurveParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub xofs: u32,
    pub yofs: u32,
    pub x_scale: u32,
    pub y_scale: u32,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    #[serde(rename = "loop")]
    pub loop_it: u32,
    pub frequency: u32,
    pub amplitude: f32,
    pub offset: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaContentDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub collectibles: Vec<CollectibleGachaItem<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CollectibleGachaItem<'a> {
    #[serde(borrow, rename = "JD_CollectibleGachaItemAvatar")]
    Avatar(CollectibleGachaItemAvatar<'a>),
    #[serde(borrow, rename = "JD_CollectibleGachaItemPortraitBorder")]
    PortraitBorder(CollectibleGachaItemPortraitBorder<'a>),
    #[serde(borrow, rename = "JD_CollectibleGachaItemAlias")]
    Alias(CollectibleGachaItemAlias<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemAvatar<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub avatar_id: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemPortraitBorder<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub portrait_id: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemAlias<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub alias_id: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutSetup1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub contexts: HashMap<Cow<'a, str>, ContextSetup1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextSetup1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub platforms: HashMap<Cow<'a, str>, ShortcutDescriptorList1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDescriptorList1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub descriptor_list: Vec<ShortcutDescriptor1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ShortcutDescriptor1719<'a> {
    #[serde(borrow, rename = "JD_DancerProfileShortcutDescriptor")]
    DancerProfile(DancerProfileShortcutDescriptor1719<'a>),
    #[serde(borrow, rename = "JD_ShortcutDescriptor")]
    Base(ShortcutDesc1719<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DancerProfileShortcutDescriptor1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    pub behaviour_name: Cow<'a, str>,
    pub show_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDesc1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    pub behaviour_name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PopupConfigList<'a> {
    // In nx2017 this is not a class, but a regular hashmap
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<Cow<'a, str>, PopupContentConfig<'a>>>,
    // Not used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation: Option<HashMap<Cow<'a, str>, PopupNavigationConfig<'a>>>,
    // Not used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup_description: Option<Vec<PopupParams<'a>>>,
    /// Retired after NX2019
    #[serde(rename = "menuDebugErrorList", default)]
    pub menu_debug_error_list: Option<Vec<u32>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub club_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017, all caps on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "DEFAULT")]
    pub default: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cross_check: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub none: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite_nosave: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_nosave: Option<PopupConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupContentConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub content_scene_path: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupNavigationConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub left_item: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub middle_item: Cow<'a, str>,
    pub right_item: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub up_item: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub bottom_item: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub start_button_index: u32,
    pub phone_button_image: Cow<'a, str>,
    pub phone_button_loc_id: u32,
    pub button_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub popup_id: Cow<'a, str>,
    pub content_key: Cow<'a, str>,
    pub navigation_key: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub full_screen_display: bool,
    /// Introduced in NX2020
    #[serde(default)]
    pub popup_overriding_sound_context: Cow<'a, str>,
    /// Introduced in NX2020
    #[serde(default)]
    pub grid_overriding_sound_context: Cow<'a, str>,
    pub display_wait_screen_on_phone_during_enter: u32,
    /// Introduced in NX2020
    #[serde(default)]
    pub message_left_alignment: bool,
    /// Introduced in NX2020
    #[serde(default)]
    pub message_area_width: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub associate_error_list: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub left_item: Cow<'a, str>,
    pub right_item: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClubRewardConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "locIdCR")]
    pub loc_id_cr: u32,
    #[serde(rename = "imgUrlCR")]
    pub img_url_cr: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub on_fire_default: f32,
    #[serde(rename = "on_fire_X360")]
    pub on_fire_x360: f32,
    #[serde(rename = "on_fire_Durango")]
    pub on_fire_durango: f32,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_charity_OK_no_move_energy_amount_factor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_charity_ok_no_move_energy_amount_factor: Option<u32>,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_charity_OK_score_ratio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_charity_ok_score_ratio: Option<f32>,
    // Not in nx2017
    #[serde(
        rename = "KidsMode_decreasingScoreRatioBoost",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kids_mode_decreasing_score_ratio_boost: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_distance_low_threshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_distance_high_threshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_auto_correlation_theshold: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_direction_impact_factor: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_move_penalty_if_energy_amount_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charity_bonus_if_energy_factor_above: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perfect_malus_if_energy_factor_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_no_move_penalty_if_energy_amount_under: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_shake_detected_max_score_ratio: Option<f32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_direction_malus_multiplier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "force_use_WiiU_classifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_use_wiiu_classifiers: Option<u32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_energy_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_energy_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_low_distance_threshold_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_low_distance_threshold_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_high_distance_threshold_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_high_distance_threshold_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_shake_sensitivity_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_shake_sensitivity_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "PSMove_direction_sensibility_modifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub psmove_direction_sensibility_modifier: Option<f32>,
    // Only in nx2017
    #[serde(
        rename = "SwitchJoyCon_direction_malus_multiplier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_joycon_direction_malus_multiplier: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringCameraParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "maxPlayerCountScoreBoostPower")]
    pub max_player_count_score_boost_power: f32,
    // Not in nx2018 or earlier
    #[serde(
        rename = "no_move_penalty_if_energy_amount_under_XOne",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_move_penalty_if_energy_amount_under_xone: Option<u32>,
    // Not in nx2018 or earlier
    #[serde(
        rename = "no_move_penalty_if_energy_amount_under_PS4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_move_penalty_if_energy_amount_under_ps4: Option<u32>,
    // Not in nx2018 or earlier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charity_bonus_if_energy_factor_above: Option<f32>,
    // Not in nx2018 or earlier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perfect_malus_if_energy_factor_under: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringMovespaceParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub default_distance_low_threshold: f32,
    pub default_distance_high_threshold: f32,
    pub default_auto_correlation_theshold: f32,
    pub default_direction_impact_factor: u32,
    pub no_move_penalty_if_energy_amount_under: f32,
    pub charity_bonus_if_energy_factor_above: f32,
    pub perfect_malus_if_energy_factor_under: f32,
    pub phone_no_move_penalty_if_energy_amount_under: f32,
    pub phone_shake_detected_max_score_ratio: f32,
    pub phone_direction_malus_multiplier: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_move_score_boost: Option<u32>,
    #[serde(
        rename = "force_use_WiiU_classifiers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_use_wiiu_classifiers: Option<u32>,
    #[serde(rename = "PSMove_energy_modifier")]
    pub psmove_energy_modifier: u32,
    #[serde(rename = "PSMove_low_distance_threshold_modifier")]
    pub psmove_low_distance_threshold_modifier: f32,
    #[serde(rename = "PSMove_high_distance_threshold_modifier")]
    pub psmove_high_distance_threshold_modifier: f32,
    #[serde(rename = "PSMove_shake_sensitivity_modifier")]
    pub psmove_shake_sensitivity_modifier: u32,
    #[serde(rename = "PSMove_direction_sensibility_modifier")]
    pub psmove_direction_sensibility_modifier: i32,
    #[serde(rename = "SwitchJoyCon_direction_malus_multiplier")]
    pub switch_joycon_direction_malus_multiplier: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MenuAssetsCacheParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub asset_type: Cow<'a, str>,
    pub max_assets: u32,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub default_assets: HashMap<Cow<'a, str>, Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub asset_path_fmts: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MenuMusicParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub scene_path: Cow<'a, str>,
    pub prefetch: u32,
    pub fadein: u32,
    pub stinger: Cow<'a, str>,
    pub jingle: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RemoteSoundParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sound_id_for_phone: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMultiTrackItem<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub menu_music_path: Cow<'a, str>,
    pub stinger: Cow<'a, str>,
    pub jingle: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMusicConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub loop_cross_fade_duration: f32,
    pub start_fade_duration: f32,
    pub multi_track_transition_beat_count: u32,
    pub end_of_loop_soundwich_notif_time_offset: u32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct RankDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "maxRank")]
    pub max_rank: u32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub color_look_up: HashMap<u32, u32>,
    pub rank_limits: Vec<u32>,
    // Not in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gain_types: Option<HashMap<Cow<'a, str>, u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestEntry17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub quest_id: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub locked: u32,
    pub trigger_end: u32,
    pub phone_image: Cow<'a, str>,
    pub playlist: Vec<Cow<'a, str>>,
    pub cover_path: Cow<'a, str>,
    pub logo_path: Cow<'a, str>,
    pub logo_shaded_path: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSongList<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub title: Cow<'a, str>,
    pub artist: Cow<'a, str>,
    #[serde(default)]
    pub map_name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct SystemDescriptor18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub button_scene_id: Cow<'a, str>,
    pub visual_scene_id: Cow<'a, str>,
    pub boss_id: u32,
    pub planets: Vec<PlanetDescriptor18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetDescriptor18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub map_list: Vec<Cow<'a, str>>,
    pub play_mode: Cow<'a, str>,
    pub is_boss_planet: bool,
    pub is_surprise: bool,
    pub planet_objectives: Vec<PlanetObjectiveDesc18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetObjectiveDesc18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub objective_id: u32,
    pub mandatory: bool,
    pub rewards: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureBossDesc18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub name: Cow<'a, str>,
    pub avatar_id: u32,
    pub skin_id: u32,
    pub final_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AdventureModeSetup18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub video_paths: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestConfig1718<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "rankingScenePathID")]
    pub ranking_scene_path_id: Cow<'a, str>,
    #[serde(rename = "rankingActorPathID")]
    pub ranking_actor_path_id: Cow<'a, str>,
    pub difficulty_final_scores: Vec<(u32, u32)>,
    pub threshold_rank: u32,
    pub nb_challengers: u32,
    pub ranking_points_gain: Vec<u32>,
    pub mojo_per_star: u32,
    pub mojo_per_rank: Vec<(u32, u32, u32)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestChallengerEntry1718<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: u32,
    pub name: Cow<'a, str>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSubtitles<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub subtitles: Cow<'a, str>,
    pub subtitles_loc_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CustomizableItemConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub default_avatar_id: u32,
    pub default_skin_id: u32,
    /// Introduced in NX2020
    #[serde(default)]
    pub default_portrait_border_id: u32,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ScheduledQuestSetup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub minimum_score: u32,
    pub session_count_until_discovery_kill: u32,
    pub session_count_until_quest_kill: u32,
    pub session_count_until_first_discovery_kill: u32,
    pub session_count_until_normal_quest_setting: u32,
    pub first_discovery_quest_id: u32,
    /// Not used before NX2020
    #[serde(rename = "MapProbabilities", default)]
    pub map_probabilities: MapChoosingProbabilities<'a>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        rename = "MapProbabilitiesNX",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_nx: Option<MapChoosingProbabilities<'a>>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        rename = "MapProbabilitiesOther",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_other: Option<MapChoosingProbabilities<'a>>,
    #[serde(rename = "PushSongProbability")]
    pub push_song_probability: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_pre_conditions: Option<HashMap<Cow<'a, str>, Vec<u32>>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub update_timings: HashMap<u32, f32>,
    pub time_cap_in_hours_to_renew: u32,
    /// Introduced in NX2020
    #[serde(default)]
    pub exclude_from_algorithm_quest_tags: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct MapChoosingProbabilities<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub ubisoft_club: u32,
    pub normal_sku: u32,
    /// Not used in NX2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_scoring: Option<u32>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachineRandomSetup17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub increase_priority_tag: HashMap<Cow<'a, str>, u32>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub special_priority: HashMap<u32, DanceMachinePriority17<'a>>,
    pub bonus_stage_min_value: u32,
    pub bonus_stage_max_value: u32,
    pub default_increase_priority_value: u32,
    pub bonus_priority_for_lowest_played_blocks: u32,
    pub delta_for_unlock_unlimited: u32,
    pub reward_tag_order: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachinePriority17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub priority_map: HashMap<Cow<'a, str>, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachineGlobalConfig1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub value_for_win_move: f32,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_for_unlock_rewards: Option<Vec<f32>>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_blocks: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_score_for_global_battery: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_animation_recap_global: Option<u32>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_path: Option<HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anim_syncrho: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>,
    // Only in nx2017
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_experience: Option<Vec<Cow<'a, str>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SweatRandomizeConfig1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub weights: HashMap<Cow<'a, str>, Vec<f32>>,
    pub excluded_tags: Vec<Cow<'a, str>>,
    pub seed_range: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchConfig1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub num_of_search_history: u32,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_rows: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tries_artist_match: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_num_artist_matches: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tries_tag_match: Option<u32>,
    // Not in nx2019?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tag_matches: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChallengerScoreEvolutionTemplate1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub template_name: Cow<'a, str>,
    pub template_descriptor: HashMap<Cow<'a, str>, f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CountryEntry<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub loc_id: u32,
    pub code: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChatMessagesParams1718<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "AllMessages_MinDelayBetweenMessages")]
    pub all_messages_min_delay_between_messages: f32,
    #[serde(rename = "AutoMessages_MinDelayBeforeFirstMessage")]
    pub auto_messages_min_delay_before_first_message: f32,
    #[serde(rename = "AutoMessages_MaxDelayBeforeFirstMessage")]
    pub auto_messages_max_delay_before_first_message: f32,
    #[serde(rename = "AutoMessages_MessagesAverageDelay")]
    pub auto_messages_messages_average_delay: f32,
    #[serde(rename = "AutoMessages_MessagesAverageDelayRandomVariation")]
    pub auto_messages_messages_average_delay_random_variation: f32,
    #[serde(rename = "AutoMessages_DelaysBoostForEachAdditionalAvatar")]
    pub auto_messages_delays_boost_for_each_additional_avatar: f32,
    #[serde(rename = "AutoMessages_GenericMessagesOccurrenceFactor")]
    pub auto_messages_generic_messages_occurrence_factor: f32,
    #[serde(rename = "AutoMessages_LevelBasedMessagesOccurrenceFactor")]
    pub auto_messages_level_based_messages_occurrence_factor: f32,
    #[serde(rename = "AutoMessages_CountryBasedMessagesOccurrenceFactor")]
    pub auto_messages_country_based_messages_occurrence_factor: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_ReadyToPlay_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_ready_to_play_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_ReadyToPlay_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_ready_to_play_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPosition_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_new_position_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPosition_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_new_position_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_NewPositionUpOrDown_TriggerProbability")]
    pub instant_messages_raceline_in_game_new_position_up_or_down_trigger_probability: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_GoldMove_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_gold_move_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_GoldMove_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_gold_move_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_Emote_MinDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_emote_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_InGame_Emote_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_in_game_emote_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_Vote_MinDelayBeforeMessage")]
    pub instant_messages_raceline_vote_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Raceline_Vote_MaxDelayBeforeMessage")]
    pub instant_messages_raceline_vote_max_delay_before_message: f32,
    #[serde(rename = "InstantMessages_RecapResult_MinDelayBeforeMessage")]
    pub instant_messages_recap_result_min_delay_before_message: f32,
    #[serde(rename = "InstantMessages_Recapresult_MaxDelayBeforeMessage")]
    pub instant_messages_recapresult_max_delay_before_message: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoDanceEffectData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "video_structure")]
    pub video_structure: AutodanceVideoStructure<'a>,
    pub effect_id: Cow<'a, str>,
    pub effect_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AutodanceVideoStructure<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub song_start_position: i32,
    pub duration: f32,
    pub thumbnail_time: u32,
    pub fade_out_duration: f32,
    pub ground_plane_path: Cow<'a, str>,
    pub first_layer_triple_background_path: Cow<'a, str>,
    pub second_layer_triple_background_path: Cow<'a, str>,
    pub third_layer_triple_background_path: Cow<'a, str>,
    #[serde(
        rename = "playback_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playback_events: Vec<PlaybackEvent<'a>>,
    #[serde(rename = "background_effect")]
    pub background_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        rename = "background_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub background_effect_events: Vec<FxEvent<'a>>,
    #[serde(rename = "player_effect")]
    pub player_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        rename = "player_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub player_effect_events: Vec<FxEvent<'a>>,
    #[serde(rename = "prop_events", default, skip_serializing_if = "Vec::is_empty")]
    pub prop_events: Vec<PropEvent<'a>>,
    #[serde(rename = "props", default, skip_serializing_if = "Vec::is_empty")]
    pub props: Vec<AutodancePropData<'a>>,
    #[serde(
        rename = "props_players_config",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub props_players_config: Vec<PropPlayerConfig<'a>>,
}

impl AutodanceVideoStructure<'_> {
    pub const CLASS: &'static str = "JD_AutodanceVideoStructure";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct FxEvent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub start_time: u32,
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropEvent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub start_time: u32,
    pub duration: u32,
    pub associated_props: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AutodancePropData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub index: u32,
    pub pivot_x: f32,
    pub pivot_y: f32,
    pub size: f32,
    #[serde(rename = "fx_assetID")]
    pub fx_asset_id: Cow<'a, str>,
    pub prop_part: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropPlayerConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub index: u32,
    pub active_props: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CoopTweakedText17<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub min_score: f32,
    pub title: u32,
    pub title_one_player: u32,
    pub desc: u32,
    pub desc_one_player: u32,
    pub sound_notification: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialContent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub popup: u32,
    pub browsable: u32,
    pub slide_delay: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<Cow<'a, str>>,
    pub message_descs: Vec<MessageDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum MessageDesc<'a> {
    #[serde(borrow, rename = "MessageSlideDesc")]
    Slide(MessageSlideDesc<'a>),
    #[serde(borrow, rename = "MessageFocusDesc")]
    Focus(MessageFocusDesc<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct MessageSlideDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub message_id: u32,
    pub image_path: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MessageFocusDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub focus_id: u32,
    pub loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub tutorial_context: Cow<'a, str>,
    pub game_context: Cow<'a, str>,
    // Named Messages before nx2019
    #[serde(alias = "Messages")]
    pub contents: Vec<Cow<'a, str>>,
    pub priority: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mandatory_song_tags: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional_song_tags: Vec<Cow<'a, str>>,
    pub max_display: i32,
    pub max_display_per_session: i32,
    /// Not used after NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_ftue_step: Option<i32>,
    /// Introduced in NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_dependant: Option<bool>,
    /// Introduced in NX2020
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_string: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UplayReward<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub reward_id: u32,
    pub reward_name: Cow<'a, str>,
    pub reward_type: u32,
    pub amount_to_unlock: u32,
    pub reward_string_on_server: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFBossEntry<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub boss_id: Cow<'a, str>,
    pub scene_path: Cow<'a, str>,
    pub logo: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureObjective18<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "ID")]
    pub id: u32,
    pub objective: ObjectiveDesc1819<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorLookUp<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub descriptor: HashMap<Cow<'a, str>, ItemColorMap<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorMap<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub colors: HashMap<Cow<'a, str>, Color>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoLoopSetup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "videoFPS")]
    pub video_fps: u32,
    pub descriptors: HashMap<Cow<'a, str>, VideoBrickDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoBrickDescriptor<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub local_start_frame: u32,
    pub local_end_frame: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HueConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub menu_color: Color,
    pub gold_effect_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbum<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_page_delay: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_page_unlock_objective_id: Option<Cow<'a, str>>,
    pub pages: Vec<CollectibleAlbumPage<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumPage<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CollectibleAlbumItem<'a>>,
    pub scene_path: Cow<'a, str>,
    // Not present in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<Cow<'a, str>>,
    pub carousel_item_scene_id: Cow<'a, str>,
    #[serde(default)]
    pub is_bonus_page: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CollectibleAlbumItem<'a> {
    #[serde(borrow, rename = "JD_CollectibleAlbumItemSticker")]
    Sticker(CollectibleAlbumItemSticker<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemCustomizable")]
    Customizable(CollectibleAlbumItemCustomizable<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemPostcard")]
    Postcard(CollectibleAlbumItemPostcard<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemMap")]
    Map(CollectibleAlbumItemMap<'a>),
    #[serde(borrow, rename = "JD_CollectibleAlbumItemJDM")]
    JDM(CollectibleAlbumItemJDM<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemSticker<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rarity: u32,
    pub sticker_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemCustomizable<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rarity: u32,
    pub customizable_item_id: u32,
    pub customizable_item_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemPostcard<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub postcard_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemMap<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rarity: u32,
    pub map_name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemJDM<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rarity: u32,
    pub episode_id: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StickerEntry<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sticker_id: u32,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<Cow<'a, str>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<Cow<'a, str>>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene_path: Option<Cow<'a, str>>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_emblem: Option<bool>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_sound: Option<bool>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub price: u32,
    pub nb_max_history_pickup_reward: u32,
    pub reward_unlock_scenes: HashMap<Cow<'a, str>, Cow<'a, str>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub rarity_pickup_percentage: HashMap<u32, u32>,
    pub force_high_rarity_reward_count: u32,
    pub force_mojo_reward_count: u32,
    // Not used in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mojo_reward_list: Option<Vec<GachaMojoRewardConfig<'a>>>,
    pub puzzle_map_reward: Cow<'a, str>,
    pub nb_maps_threshold_before_push_gacha_screen: (u32, u32),
    // Not used in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_play_count_between_map_rewards: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaMojoRewardConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub mojo_amount: u32,
    pub number_of_packs: u32,
    pub rarity: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordered_steps: Option<Vec<StepInfo<'a>>>,
    pub songs_to_be_kept_unlocked: Vec<Cow<'a, str>>,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps_helper_version_mismatch_indicator: Option<u32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_available_on_main_carousel: Option<i32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_to_unlock_dance_card: Option<i32>,
    // Not in nx2019 or later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub songs_to_push_gatcha: Option<i32>,
    // Not in nx2019 or later
    #[serde(
        rename = "songsToUnlockJDMainCarousel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub songs_to_unlock_jd_main_carousel: Option<i32>,
    // Not in nx2019 or later
    #[serde(
        rename = "songsToUnlockJDU",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub songs_to_unlock_jdu: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StepInfo<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub step: u32,
    pub map_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RumbleConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sleep_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub parent_marker: Cow<'a, str>,
    pub delta_x: u32,
    pub delta_y: u32,
    pub rows_count: u32,
    pub columns_count: u32,
    pub start_line: u32,
    pub start_line_offset: u32,
    pub start_column: u32,
    pub line_scroll_condition: u32,
    pub no_scroll_height: u32,
    pub element_count_per_line: u32,
    pub visible_element_count: i32,
    pub banner_trigger_time: f32,
    pub audio_preview_time: f32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridActorsToPreload<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorsToPreload<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub scene_path: Cow<'a, str>,
    pub actor_path: Cow<'a, str>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LayoutTabbedGrids<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tabbed_grid_descs: Vec<TabbedGridDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TabbedGridDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub tab_name: Cow<'a, str>,
    pub tab_hover_tutorial_context: Cow<'a, str>,
    pub tab_content_tutorial_context: Cow<'a, str>,
    pub start_index: u32,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid_actors_to_preload_id: Option<Cow<'a, str>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_trigger_time: Option<f32>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_preview_time: Option<f32>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grid_desc_id: Option<Cow<'a, str>>,
    // Not used in nx2020 and later
    #[serde(
        rename = "RequiredFtueSteps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_ftue_steps: Option<u32>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_full_install: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeDataConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub elements_list_by_visual_type: HashMap<Cow<'a, str>, [Cow<'a, str>; 5]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeDataTipEntry<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub tip_id: Cow<'a, str>,
    pub catch_phrase: u32,
    pub content: u32,
    pub thumbnail: Cow<'a, str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<Cow<'a, str>>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by_first_time_layout: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeVideoDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub video_thumbnail_path: Cow<'a, str>,
    pub video_path: Cow<'a, str>,
    pub video_tile_title_loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongsSearchTags<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub maps: HashMap<Cow<'a, str>, SongSearchTags<'a>>,
}

impl SongsSearchTags<'_> {
    pub const CLASS: &'static str = "JD_SongsSearchTags";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTags<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub tags: Vec<SongSearchTag<'a>>,
}

impl SongSearchTags<'_> {
    pub const CLASS: &'static str = "JD_SongSearchTags";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTag<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub tag_loc_id: LocaleId,
    pub tag: Cow<'a, str>,
}

impl SongSearchTag<'_> {
    pub const CLASS: &'static str = "JD_SongSearchTag";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GroupsSoundNotificationConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "delay_before_sendingEvent_enterNewGroupStartTimer")]
    pub delay_before_sending_event_enter_new_group_start_timer: f32,
    #[serde(rename = "delay_before_sendingEvent_changeGroup")]
    pub delay_before_sending_event_change_group: f32,
    pub items_group: Vec<ItemsGroup<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemsGroup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub group_name: Cow<'a, str>,
    pub items_indexes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OnFlyNotificationTypeParams<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub default_form_and_timing: FormAndTiming<'a>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub specific_cases_form_and_timing: HashMap<Cow<'a, str>, FormAndTiming<'a>>,
    pub bubble_title_loc_id: u32,
    pub reward_screen_title_loc_id: u32,
    pub specific_content_loc_id: u32,
    pub forbid_reward_screen_flow_jump_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FormAndTiming<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub form: u32,
    pub timing: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RecapConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub gauge_number_of_stars_per_beat: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfigs<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub configs: Vec<WhatsNewConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub config_name: Cow<'a, str>,
    pub ui_display: Cow<'a, str>,
    pub max_views: u32,
    pub session_interval: u32,
    pub related_song_tags: Vec<Cow<'a, str>>,
    pub subscribed_grid_desc: Cow<'a, str>,
    pub unsubscribed_grid_desc: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub anim_setups: HashMap<Cow<'a, str>, CarouselAnimSetup<'a>>,
    pub carousel_descs: HashMap<Cow<'a, str>, CarouselDesc<'a>>,
    pub item_object: HashMap<Cow<'a, str>, Cow<'a, str>>,
    pub item_logic: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselAnimSetup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub acceleration: u32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_deceleration_start_ratio: f32,
    pub max_deceleration_start_ratio: f32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub title: Cow<'a, str>,
    pub start_index: u32,
    pub is_loop: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<CarouselElementDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselElementDesc<'a> {
    #[serde(borrow, rename = "CarouselElementDesc_Base")]
    CarouselElementDescBase(CarouselElementDescBase<'a>),
    #[serde(borrow, rename = "CarouselElementDesc_Carousel")]
    CarouselElementDescCarousel(CarouselElementDescCarousel<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc")]
    Base(JdCarouselElementDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action")]
    Action(JdCarouselElementDescAction<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ActivateConnection")]
    ActionActivateConnection(JdCarouselElementDescActionActivateConnection<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Age")]
    ActionAge(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Amiibo")]
    ActionAmiibo(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangeCluster_InstallCheck"
    )]
    ActionChangeClusterInstallCheck(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangeCluster_VideoChallenge"
    )]
    ActionChangeClusterVideoChallenge(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangeCluster_WDF")]
    ActionChangeClusterWDF(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangeCluster")]
    ActionChangeCluster(JdCarouselElementDescActionChangeCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangePage")]
    ActionChangePage(JdCarouselElementDescActionChangePage<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_ChangePageFromHomeTile"
    )]
    ActionChangePageFromHomeTile(JdCarouselElementDescActionChangePageFromHomeTile<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ChangePageWithContext")]
    ActionChangePageWithContext(JdCarouselElementDescActionChangePageWithContext<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_CheckBox")]
    ActionCheckBox(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_DeviceSelection")]
    ActionDeviceSelection(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_EditFocusedDancerCardNickname"
    )]
    ActionEditFocusedDancerCardNickname(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EnterGameMode")]
    ActionEnterGameMode(JdCarouselElementDescActionEnterGameMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipAlias")]
    ActionEquipAlias(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipAvatar")]
    ActionEquipAvatar(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_EquipPortraitBorder")]
    ActionEquipPortraitBorder(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gacha_Cancel")]
    ActionGachaCancel(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gacha_Play")]
    ActionGachaPlay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Gender")]
    ActionGender(JdCarouselElementDescActionBase<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDesc_Action_Goto_DancerCardCreation"
    )]
    ActionGotoDancerCardCreation(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_Gacha")]
    ActionGotoGacha(JdCarouselElementDescActionGotoGacha<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_JDU")]
    ActionGotoJDU(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_Goto_Postcards")]
    ActionGotoPostcards(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_LaunchJDTV")]
    ActionLaunchJDTV(JdCarouselElementDescActionLaunchJDTV<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_LaunchWDF")]
    ActionLaunchWDF(JdCarouselElementDescActionGoto<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenHomeArticle")]
    ActionOpenHomeArticle(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenKeyboard")]
    ActionOpenKeyboard(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_OpenUplay")]
    ActionOpenUplay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PlaySong")]
    ActionPlaySong(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PlaySongBanner")]
    ActionPlaySongBanner(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_PostcardFullscreen")]
    ActionPostcardFullscreen(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_QuestDifficulty")]
    ActionQuestDifficulty(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_RematchChallenge")]
    ActionRematchChallenge(JdCarouselElementDescActionRematchChallenge<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_ReportChallengeVideo")]
    ActionReportChallengeVideo(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SavePlaylist")]
    ActionSavePlaylist(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SelectChallengeMode")]
    ActionSelectChallengeMode(JdCarouselElementDescActionSelectChallengeMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SendTauntMessage")]
    ActionSendTauntMessage(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_StartPlaylist")]
    ActionStartPlaylist(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_StartQuickplay")]
    ActionStartQuickplay(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_SweatHome")]
    ActionSweatHome(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_Action_WDF_VoteChoice")]
    ActionWDFVoteChoice(JdCarouselElementDescActionBase<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDesc_TabItem")]
    TabItem(JdCarouselElementDescTabItem<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescAction<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionActivateConnection<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub connection: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePage<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionSelectChallengeMode<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionRematchChallenge<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageFromHomeTile<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    // Not present in nx2019 and before
    #[serde(
        rename = "needToSetJDUDestination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub need_to_set_jdu_destination: Option<bool>,
    // Not present in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_type: Option<Cow<'a, str>>,
    // Not present in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_sub_type: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageWithContext<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    pub context: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangeCluster<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionEnterGameMode<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGotoGacha<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    pub gacha_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGoto<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionLaunchJDTV<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    pub title: Cow<'a, str>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    pub banner_setup: BannerSetup<'a>,
    pub tag: Cow<'a, str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    pub destination: Cow<'a, str>,
    pub context: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct BannerSetup<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "Type")]
    pub type_it: Cow<'a, str>,
    pub theme: Cow<'a, str>,
    pub context: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_x: Option<u32>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_y: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CarouselElementDesc<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescTabItem<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescCarousel<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<Cow<'a, str>>,
    pub item_object: Cow<'a, str>,
    pub item_logic: Cow<'a, str>,
    pub enabled: u32,
    #[serde(rename = "carouselDescID")]
    pub carousel_desc_id: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselElementDescComponent<'a> {
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_MojoDisplay")]
    MojoDisplay(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Device")]
    Device(JdCarouselElementDescComponentDevice<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_GameMode")]
    GameMode(JdCarouselElementDescComponentGameMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_NewMarker_Item")]
    NewMarkerItem(JdCarouselElementDescComponentNewMarkerItem<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_NewMarker_Tab")]
    NewMarkerTab(JdCarouselElementDescComponentNewMarkerTab<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_PostcardsCompletionDisplay"
    )]
    PostcardsCompletionDisplay(JdCarouselElementDescComponentCompletionDisplay<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_SoundNotification")]
    SoundNotification(JdCarouselElementDescComponentSoundNotification<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_SoundNotification_ForHomeTileMap"
    )]
    SoundNotificationForHomeTileMap(JdCarouselElementDescComponentSoundNotification<'a>),
    #[serde(
        borrow,
        rename = "JD_CarouselElementDescComponent_StickerAlbumCompletionDisplay"
    )]
    StickerAlbumCompletionDisplay(JdCarouselElementDescComponentCompletionDisplay<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_WDF_VoteChoice")]
    WDFVoteChoice(JdCarouselElementDescComponentWDFVoteChoice<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Age")]
    Age(JdCarouselElementDescComponentAge<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Amiibo")]
    Amiibo(JdCarouselElementDescComponentAmiibo<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Cluster")]
    Cluster(JdCarouselElementDescComponentCluster<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Gender")]
    Gender(JdCarouselElementDescComponentGender<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Rival_Mode")]
    RivalMode(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_Coop_Mode")]
    CoopMode(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_SweatMode")]
    SweatMode(JdCarouselElementDescComponentSweatMode<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_QuestDifficulty")]
    QuestDifficulty(JdCarouselElementDescComponentQuestDifficulty<'a>),
    #[serde(borrow, rename = "JD_CarouselElementDescComponent_TauntCategory")]
    TauntCategory(JdCarouselElementDescComponentTauntCategory<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentAge<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub catage: u32,
    pub phone_image_path: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentAmiibo<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub avatar_id: u32,
    pub character_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCluster<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub context: Cow<'a, str>,
    pub transition: Cow<'a, str>,
    pub news_placement: Cow<'a, str>,
    pub availability_check: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGender<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub gender: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSweatMode<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sweat_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentQuestDifficulty<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub difficulty: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentTauntCategory<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub messages: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerItem<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub patch_marker: Cow<'a, str>,
    pub indicator_id: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerTab<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub patch_marker: Cow<'a, str>,
    pub tab_name: Cow<'a, str>,
    pub clean_rule: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGameMode<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub context: Cow<'a, str>,
    pub transition: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentDevice<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub scoring_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentWDFVoteChoice<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub vote: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSoundNotification<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<Cow<'a, str>>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_notification_prefix: Option<Cow<'a, str>>,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<Cow<'a, str>>,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<Cow<'a, str>>,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focus_notification: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCompletionDisplay<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub marker: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffectList<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub effects: Vec<FontEffect<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffect<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub fadein_start: u32,
    pub fadein_end: u32,
    pub fadeout_start: i32,
    pub fadeout_end: i32,
    pub speed_min: u32,
    pub speed_max: u32,
    #[serde(rename = "static")]
    pub static_it: u32,
    pub static_seed: u32,
    pub min: u32,
    pub max: u32,
    pub limit: u32,
    pub value: u32,
    pub alpha_left: u32,
    pub alpha_mid_left: u32,
    pub length_left: u32,
    pub alpha_right: u32,
    pub alpha_mid_right: u32,
    pub length_right: u32,
    pub alpha_top: f32,
    pub alpha_mid_top: u32,
    pub length_top: u32,
    pub alpha_bottom: f32,
    pub alpha_mid_bottom: u32,
    pub length_bottom: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUESteps<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub ftue_steps_descs: Vec<FTUEStepDesc<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEStepDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub step_name: Cow<'a, str>,
    pub step_done_objective_id: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRules<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub action_lists: HashMap<Cow<'a, str>, ActionList<'a>>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub song_item_lists: Option<HashMap<Cow<'a, str>, SongItemList<'a>>>,
    pub rules: HashMap<Cow<'a, str>, CarouselRule<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionList<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub item_type: Cow<'a, str>,
    pub actions: Vec<Action<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "type")]
    pub type_it: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub title_id: u32,
    pub online_title_id: u32,
    pub target: Cow<'a, str>,
    pub banner_type: Cow<'a, str>,
    pub banner_theme: Cow<'a, str>,
    pub banner_context: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItemList<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub action_list_name: Cow<'a, str>,
    pub list: Vec<SongItem<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItem<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub act: Cow<'a, str>,
    pub action_list_name: Cow<'a, str>,
    pub isc: Cow<'a, str>,
    pub map_name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRule<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub categories: Vec<CategoryRule<'a>>,
    pub online_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CategoryRule<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub act: Cow<'a, str>,
    pub isc: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub title_id: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<CarouselRequestDesc<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

impl CategoryRule<'_> {
    pub const CLASS: &'static str = "CategoryRule";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselRequestDesc<'a> {
    #[serde(borrow, rename = "JD_CarouselCustomizableItemRequestDesc")]
    CustomizableItem(CarouselCustomizableItemRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselUgcRequestDesc")]
    Ugc(CarouselUgcRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselMapRequestDesc")]
    Map(CarouselMapRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselDancerCardRequestDesc")]
    DancerCard(CarouselDancerCardRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselFriendRequestDesc")]
    Friend(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselItemRequestDesc")]
    Item(CarouselItemRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselPlaylistsRequestDesc")]
    Playlists(CarouselPlaylistsRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselComVideoRequestDesc")]
    ComVideo(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselLatestChallengesRequestDesc")]
    LatestChallenges(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselFriendChallengesRequestDesc")]
    FriendChallenges(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselPhotoRequestDesc")]
    PhotoRequest(CarouselPhotoRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselQuestRequestDesc")]
    QuestRequest(CarouselQuestRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselMapSearchRequestDesc")]
    MapSearch(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselSearchRequestDesc")]
    Search(Empty<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselCustomizableItemRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub jdversion: i32,
    pub lock_type: u32,
    pub sort_by: u32,
    pub item_type: u32,
    pub unlocked_only: bool,
    pub first_time_creation: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselUgcRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "type")]
    pub type_it: Cow<'a, str>,
    pub action_list_name: Cow<'a, str>,
    pub uploading: bool,
    pub offline: bool,
    pub most_liked: bool,
    pub most_viewed: bool,
    pub featured: bool,
    pub query_pid: bool,
    pub player_pid: bool,
    pub friend_pids: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselMapRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub action_list_name: Cow<'a, str>,
    #[serde(rename = "originalJDVersion")]
    pub original_jd_version: u32,
    pub coach_count: u32,
    pub order: Cow<'a, str>,
    pub subscribed: bool,
    pub favorites: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub included_tags: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_tags: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_tags: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselDancerCardRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub action_list_name: Cow<'a, str>,
    pub main: bool,
    pub create: bool,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_save_item: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselItemRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not on WiiU
    #[serde(default)]
    pub item_list: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPlaylistsRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub isc: Cow<'a, str>,
    pub act: Cow<'a, str>,
    #[serde(rename = "type")]
    pub type_it: Cow<'a, str>,
    #[serde(rename = "playlistID")]
    pub playlist_id: Cow<'a, str>,
}

impl Default for CarouselPlaylistsRequestDesc<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            isc: Cow::Borrowed("grp_row"),
            act: Cow::Borrowed("ui_carousel"),
            type_it: Cow::Borrowed("edito-pinned"),
            playlist_id: Cow::Borrowed(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPhotoRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub action_list_name: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselQuestRequestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub start_action: Cow<'a, str>,
    pub offline: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselFilter<'a> {
    #[serde(borrow, rename = "JD_CarouselFilter")]
    Empty(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselSubscriptionFilter")]
    Subscription(CarouselSubscriptionFilter<'a>),
    #[serde(borrow, rename = "JD_CarouselSkuFilter")]
    Sku(CarouselSkuFilter<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselSubscriptionFilter<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselSkuFilter<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub game_version: Cow<'a, str>,
    pub platform: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct TRCLocalisation<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "TRCLocalisationList")]
    pub trc_localisation_list: Vec<TRCLocalisationDetail<'a>>,
    pub popups_illustrations: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TRCLocalisationDetail<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "TRCError")]
    pub trc_error: u32,
    pub illustration_id: Cow<'a, str>,
    pub title: SmartLocId<'a>,
    pub message: SmartLocId<'a>,
    pub button_left: SmartLocId<'a>,
    pub button_middle: SmartLocId<'a>,
    pub button_right: SmartLocId<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SmartLocId<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub loc_id: u32,
    pub default_text: Cow<'a, str>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ObjectivesDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub objective_descs: HashMap<Cow<'a, str>, ObjectiveDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ObjectiveDescriptor<'a> {
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AccumulateXCal")]
    AccumulateXCal(ObjectiveDescriptorAccumulateXCal<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AccumulateXMoves")]
    AccumulateXMoves(ObjectiveDescriptorAccumulateXMoves<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ActivateCoopMode")]
    ActivateCoopMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_AddXSongsToAPlaylist")]
    AddXSongsToAPlaylist(ObjectiveDescriptorAddXSongsToAPlaylist<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_BeatWDFBoss")]
    BeatWDFBoss(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ChangeCustoItemXTimes")]
    ChangeCustoItemXTimes(ObjectiveDescriptorChangeCustoItemXTimes<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_CompleteXQuests")]
    CompleteXQuests(ObjectiveDescriptorCompleteXQuests<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_DanceXSeconds")]
    DanceXSeconds(ObjectiveDescriptorDanceXSeconds<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_FinishXPlaylist")]
    FinishXPlaylist(ObjectiveDescriptorFinishXPlaylist<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_GatherXStars")]
    GatherXStars(ObjectiveDescriptorGatherXStars<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_LinkedToUplay")]
    LinkedToUplay(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenAnthologyMode")]
    OpenAnthologyMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenPostcardsGallery")]
    OpenPostcardsGallery(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenStickerAlbum")]
    OpenStickerAlbum(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_OpenVideoGallery")]
    OpenVideoGallery(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayDailyQuestsForXDays")]
    PlayDailyQuestsForXDays(ObjectiveDescriptorPlayDailyQuestsForXDays<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayGachaXTimes")]
    PlayGachaXTimes(ObjectiveDescriptorPlayGachaXTimes<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayPreviousJD")]
    PlayPreviousJD(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayWDFTournament")]
    PlayWDFTournament(ObjectiveDescriptorPlayWDFTournament<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayXMaps")]
    PlayXMaps(ObjectiveDescriptorPlayXMaps<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_PlayXWDFTournamentRounds")]
    PlayXWDFTournamentRounds(ObjectiveDescriptorPlayXWDFTournamentRounds<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_ReachRankX")]
    ReachRankX(ObjectiveDescriptorReachRankX<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_RenewJDUSub")]
    RenewJDUSub(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_SwitchSweatMode")]
    SwitchSweatMode(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockUplayRewardAliasPack1")]
    UnlockUplayRewardAliasPack1(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockUplayRewardAliasPack2")]
    UnlockUplayRewardAliasPack2(ObjectiveDescriptorBase<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockXPortraitBorders")]
    UnlockXPortraitBorders(ObjectiveDescriptorUnlockXPortraitBorders<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_UnlockXStickers")]
    UnlockXStickers(ObjectiveDescriptorUnlockXStickers<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptor_WinWDFTeamBattle")]
    WinWDFTeamBattle(ObjectiveDescriptorBase<'a>),
}

impl ObjectiveDescriptor<'_> {
    #[must_use]
    pub const fn description(&self) -> LocaleId {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::OpenAnthologyMode(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.description,
            Self::AccumulateXCal(data) => data.description,
            Self::AccumulateXMoves(data) => data.description,
            Self::AddXSongsToAPlaylist(data) => data.description,
            Self::ChangeCustoItemXTimes(data) => data.description,
            Self::CompleteXQuests(data) => data.description,
            Self::DanceXSeconds(data) => data.description,
            Self::FinishXPlaylist(data) => data.description,
            Self::GatherXStars(data) => data.description,
            Self::PlayDailyQuestsForXDays(data) => data.description,
            Self::PlayGachaXTimes(data) => data.description,
            Self::PlayWDFTournament(data) => data.description,
            Self::PlayXMaps(data) => data.description,
            Self::PlayXWDFTournamentRounds(data) => data.description,
            Self::ReachRankX(data) => data.description,
            Self::UnlockXPortraitBorders(data) => data.description,
            Self::UnlockXStickers(data) => data.description,
        }
    }

    #[must_use]
    pub const fn is_static(&self) -> bool {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenAnthologyMode(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.is_static,
            Self::AccumulateXCal(data) => data.is_static,
            Self::AccumulateXMoves(data) => data.is_static,
            Self::AddXSongsToAPlaylist(data) => data.is_static,
            Self::ChangeCustoItemXTimes(data) => data.is_static,
            Self::CompleteXQuests(data) => data.is_static,
            Self::DanceXSeconds(data) => data.is_static,
            Self::FinishXPlaylist(data) => data.is_static,
            Self::GatherXStars(data) => data.is_static,
            Self::PlayDailyQuestsForXDays(data) => data.is_static,
            Self::PlayGachaXTimes(data) => data.is_static,
            Self::PlayWDFTournament(data) => data.is_static,
            Self::PlayXMaps(data) => data.is_static,
            Self::PlayXWDFTournamentRounds(data) => data.is_static,
            Self::ReachRankX(data) => data.is_static,
            Self::UnlockXPortraitBorders(data) => data.is_static,
            Self::UnlockXStickers(data) => data.is_static,
        }
    }

    #[must_use]
    pub const fn exclude_from_upload(&self) -> bool {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenAnthologyMode(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.exclude_from_upload,
            Self::AccumulateXCal(data) => data.exclude_from_upload,
            Self::AccumulateXMoves(data) => data.exclude_from_upload,
            Self::AddXSongsToAPlaylist(data) => data.exclude_from_upload,
            Self::ChangeCustoItemXTimes(data) => data.exclude_from_upload,
            Self::CompleteXQuests(data) => data.exclude_from_upload,
            Self::DanceXSeconds(data) => data.exclude_from_upload,
            Self::FinishXPlaylist(data) => data.exclude_from_upload,
            Self::GatherXStars(data) => data.exclude_from_upload,
            Self::PlayDailyQuestsForXDays(data) => data.exclude_from_upload,
            Self::PlayGachaXTimes(data) => data.exclude_from_upload,
            Self::PlayWDFTournament(data) => data.exclude_from_upload,
            Self::PlayXMaps(data) => data.exclude_from_upload,
            Self::PlayXWDFTournamentRounds(data) => data.exclude_from_upload,
            Self::ReachRankX(data) => data.exclude_from_upload,
            Self::UnlockXPortraitBorders(data) => data.exclude_from_upload,
            Self::UnlockXStickers(data) => data.exclude_from_upload,
        }
    }
}

impl<'a> ObjectiveDescriptor<'a> {
    #[must_use]
    pub fn description_raw(&self) -> Cow<'a, str> {
        match self {
            Self::ActivateCoopMode(data)
            | Self::BeatWDFBoss(data)
            | Self::LinkedToUplay(data)
            | Self::OpenPostcardsGallery(data)
            | Self::OpenStickerAlbum(data)
            | Self::OpenVideoGallery(data)
            | Self::OpenAnthologyMode(data)
            | Self::PlayPreviousJD(data)
            | Self::RenewJDUSub(data)
            | Self::SwitchSweatMode(data)
            | Self::UnlockUplayRewardAliasPack2(data)
            | Self::WinWDFTeamBattle(data)
            | Self::UnlockUplayRewardAliasPack1(data) => data.description_raw.clone(),
            Self::AccumulateXCal(data) => data.description_raw.clone(),
            Self::AccumulateXMoves(data) => data.description_raw.clone(),
            Self::AddXSongsToAPlaylist(data) => data.description_raw.clone(),
            Self::ChangeCustoItemXTimes(data) => data.description_raw.clone(),
            Self::CompleteXQuests(data) => data.description_raw.clone(),
            Self::DanceXSeconds(data) => data.description_raw.clone(),
            Self::FinishXPlaylist(data) => data.description_raw.clone(),
            Self::GatherXStars(data) => data.description_raw.clone(),
            Self::PlayDailyQuestsForXDays(data) => data.description_raw.clone(),
            Self::PlayGachaXTimes(data) => data.description_raw.clone(),
            Self::PlayWDFTournament(data) => data.description_raw.clone(),
            Self::PlayXMaps(data) => data.description_raw.clone(),
            Self::PlayXWDFTournamentRounds(data) => data.description_raw.clone(),
            Self::ReachRankX(data) => data.description_raw.clone(),
            Self::UnlockXPortraitBorders(data) => data.description_raw.clone(),
            Self::UnlockXStickers(data) => data.description_raw.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAccumulateXCal<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub calories_amount: u32,
    pub in_one_session: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAccumulateXMoves<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub moves_count: u32,
    pub categories_to_count: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAddXSongsToAPlaylist<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub songs_added_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorChangeCustoItemXTimes<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub custo_item_changes_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorCompleteXQuests<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub quests_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorDanceXSeconds<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub dance_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorFinishXPlaylist<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub playlists_play_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorGatherXStars<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub stars_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayDailyQuestsForXDays<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    #[serde(rename = "consecutiveDays")]
    pub consecutive_days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayGachaXTimes<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub gacha_plays_count: u32,
    pub unlock_all_acceptable_gacha_items: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayWDFTournament<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    // Not in nx2020
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tournament_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayXMaps<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub maps_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayXWDFTournamentRounds<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub rounds_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorReachRankX<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    #[serde(rename = "rankToReach")]
    pub rank_to_reach: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorUnlockXPortraitBorders<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub portrait_border_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorUnlockXStickers<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub description_raw: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub stickers_count: u32,
    // Not in nx2020
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_stickers: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum ObjectiveDescriptorComponent<'a> {
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_CustoItemTypeRequirement"
    )]
    CustoItemTypeRequirement(ObjectiveDescriptorComponentCustoItemTypeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_GachaItemTypeRequirement"
    )]
    GachaItemTypeRequirement(ObjectiveDescriptorComponentGachaItemTypeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapCoachCountRequirement"
    )]
    MapCoachCountRequirement(ObjectiveDescriptorComponentMapCoachCountRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapLaunchLocationRequirement"
    )]
    MapLaunchLocationRequirement(ObjectiveDescriptorComponentMapLaunchLocationRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapMovesRequirement")]
    MapMovesRequirement(ObjectiveDescriptorComponentMapMovesRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapNameRequirement")]
    MapNameRequirement(ObjectiveDescriptorComponentMapNameRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_MapPlaymodeRequirement"
    )]
    MapPlaymodeRequirement(ObjectiveDescriptorComponentMapPlaymodeRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapScoreRequirement")]
    MapScoreRequirement(ObjectiveDescriptorComponentMapScoreRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_MapTagsRequirement")]
    MapTagsRequirement(ObjectiveDescriptorComponentMapTagsRequirement<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDescriptorComponent_OnlyOnline")]
    OnlyOnline(ObjectiveDescriptorComponentBase<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_OnlyOnUnlimitedSongs"
    )]
    OnlyOnUnlimitedSongs(ObjectiveDescriptorComponentBase<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_PlaylistIdRequirement"
    )]
    PlaylistIdRequirement(ObjectiveDescriptorComponentPlaylistIdRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_ScoringModeRequirement"
    )]
    ScoringModeRequirement(ObjectiveDescriptorComponentScoringModeRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_SearchLabelsRequirement"
    )]
    SearchLabelsRequirement(ObjectiveDescriptorComponentSearchLabelsRequirement<'a>),
    #[serde(
        borrow,
        rename = "JD_ObjectiveDescriptorComponent_StickerIdRequirement"
    )]
    StickerIdRequirement(ObjectiveDescriptorComponentStickerIdRequirement<'a>),
}

impl ObjectiveDescriptorComponent<'_> {
    #[must_use]
    pub const fn only_diff_values(&self) -> bool {
        match self {
            Self::CustoItemTypeRequirement(data) => data.only_diff_values,
            Self::GachaItemTypeRequirement(data) => data.only_diff_values,
            Self::MapCoachCountRequirement(data) => data.only_diff_values,
            Self::MapLaunchLocationRequirement(data) => data.only_diff_values,
            Self::MapMovesRequirement(data) => data.only_diff_values,
            Self::MapNameRequirement(data) => data.only_diff_values,
            Self::MapPlaymodeRequirement(data) => data.only_diff_values,
            Self::MapScoreRequirement(data) => data.only_diff_values,
            Self::MapTagsRequirement(data) => data.only_diff_values,
            Self::OnlyOnline(data) | Self::OnlyOnUnlimitedSongs(data) => data.only_diff_values,
            Self::PlaylistIdRequirement(data) => data.only_diff_values,
            Self::ScoringModeRequirement(data) => data.only_diff_values,
            Self::SearchLabelsRequirement(data) => data.only_diff_values,
            Self::StickerIdRequirement(data) => data.only_diff_values,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentCustoItemTypeRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_custo_item_types: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentGachaItemTypeRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_gacha_item_types: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapCoachCountRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_coach_counts: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapLaunchLocationRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_contexts: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_subcontexts: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapMovesRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub exact_moves_count: u32,
    pub min_moves_count: u32,
    pub max_moves_count: u32,
    pub all_map_moves_count: bool,
    pub only_map_last_move: bool,
    pub moves_in_a_row: bool,
    pub acceptable_categories: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapNameRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_names: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapPlaymodeRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub classic: bool,
    pub coop: bool,
    pub sweat: bool,
    pub playlist: bool,
    #[serde(rename = "WDF")]
    pub wdf: bool,
    pub kids: bool,
    // Not present in nx2021 or nx2022
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anthology: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapScoreRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub score: u32,
    pub better_than_dancer_of_the_week: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapTagsRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_tags: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unacceptable_map_tags: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentPlaylistIdRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_playlist_ids: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentScoringModeRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_scoring_modes: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentSearchLabelsRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_label_loc_ids: Vec<LocaleId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_label_loc_ids: Vec<LocaleId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentStickerIdRequirement<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub only_diff_values: bool,
    pub acceptable_sticker_ids: Vec<u32>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumbleManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rumbles: Vec<PadRumble<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumble<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub intensity: f32,
    pub duration: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlaylistDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub playlists: HashMap<Cow<'a, str>, OfflinePlaylist<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OfflinePlaylist<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub title_id: LocaleId,
    pub description_id: LocaleId,
    pub cover_path: Cow<'a, str>,
    pub maps: Vec<Cow<'a, str>>,
}

impl OfflinePlaylist<'_> {
    pub const CLASS: &'static str = "OfflinePlaylist";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBordersDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub portrait_borders: Vec<PortraitBorderDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBorderDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub portrait_border_id: u16,
    pub background_texture_path: Cow<'a, str>,
    pub foreground_texture_path: Cow<'a, str>,
    pub background_phone_path: Cow<'a, str>,
    pub foreground_phone_path: Cow<'a, str>,
    pub original_lock_status: u8,
    pub visibility: u8,
}

impl PortraitBorderDesc<'_> {
    pub const CLASS: &'static str = "JD_PortraitBorderDesc";
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRules<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rules_by_priority_order: Vec<QuickplayRule<'a>>,
    #[serde(
        rename = "DEBUG_TEST_LIST",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub debug_test_list: Vec<Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum QuickplayRule<'a> {
    #[serde(
        borrow,
        rename = "JD_QuickplayRule_EnforceEasyMapAfterModerateMapsSequence"
    )]
    EnforceEasyMapAfterModerateMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_EnforceEasyMapAtStart")]
    EnforceEasyMapAtStart(QuickplayRuleEnforceEasyMapAtStart<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_ForbidExtremeMapsSequence")]
    ForbidExtremeMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_ForbidIntenseMapsSequence")]
    ForbidIntenseMapsSequence(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_PromoteNeverPlayedMaps")]
    PromoteNeverPlayedMaps(QuickplayRuleBase<'a>),
    #[serde(borrow, rename = "JD_QuickplayRule_PromoteNeverPlayedTaggedMaps")]
    PromoteNeverPlayedTaggedMaps(QuickplayRulePromoteNeverPlayedTaggedMaps<'a>),
    #[serde(
        borrow,
        rename = "JD_QuickplayRule_PromoteSameNumberOfCoachesAsNumberOfPlayers"
    )]
    PromoteSameNumberOfCoachesAsNumberOfPlayers(QuickplayRuleBase<'a>),
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRuleBase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub max_number_of_previous_maps_to_consider: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRuleEnforceEasyMapAtStart<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub max_number_of_previous_maps_to_consider: u32,
    #[serde(rename = "acceptableSweatDifficulties")]
    pub acceptable_sweat_difficulties: Vec<u32>,
    #[serde(rename = "acceptableSongDifficulties")]
    pub acceptable_song_difficulties: Vec<u32>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRulePromoteNeverPlayedTaggedMaps<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub max_number_of_previous_maps_to_consider: u32,
    #[serde(rename = "acceptableTags")]
    pub acceptable_tags: Vec<Cow<'a, str>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScheduledQuestDatabase<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub scheduled_quests: Vec<ScheduledQuestDesc<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ScheduledQuestDesc<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Type")]
    pub type_it: u8,
    pub unlimited_only: bool,
    pub mojo_reward: u32,
    pub probability_weight: u32,
    pub objective_id: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions_objectives_id: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Cow<'a, str>>,
}

impl ScheduledQuestDesc<'_> {
    pub const CLASS: &'static str = "JD_ScheduledQuestDesc";
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SoundConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub busses: Vec<BusDef<'a>>,
    pub limiters: Vec<LimiterDef<'a>>,
    pub pause_fade_in: f32,
    pub pause_fade_out: f32,
    pub headphone_bus_mix: BusMix<'a>,
    pub music_tracks: Vec<MusicTrackDef<'a>>,
    #[serde(rename = "PCMSources")]
    pub pcm_sources: Vec<PCMSourceDef<'a>>,
    pub metronome_debug_sound_bar: Cow<'a, str>,
    pub metronome_debug_sound_beat: Cow<'a, str>,
    #[serde(rename = "TestSounds")]
    pub test_sounds: Vec<Cow<'a, str>>,
    pub bus_mix_list: Vec<EventBusMix<'a>>,
    pub bus_fade_list: Vec<EventBusFade<'a>>,
    pub soundwich_synth: Cow<'a, str>,
    pub soundwich_modules: Vec<Cow<'a, str>>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_fade_curves: Option<Vec<ProjectFadeCurve<'a>>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusDef<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<Cow<'a, str>>,
    pub volume: i32,
    pub filter_frequency: u32,
    pub filter_type: u32,
    pub out_devices: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LimiterDef<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub max_instances: u32,
    pub mode: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusMix<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub priority: u32,
    pub duration: f32,
    pub fade_in: f32,
    pub fade_out: f32,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_mix: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bus_defs: Vec<BusDef<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MusicTrackDef<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub track_name: Cow<'a, str>,
    pub bus_name: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PCMSourceDef<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub nb_channels: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusMix<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sender: u32,
    pub name: Cow<'a, str>,
    pub activate: u32,
    pub bus_mix: BusMix<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusFade<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sender: u32,
    pub name: Cow<'a, str>,
    pub bus: Cow<'a, str>,
    pub time: f32,
    pub fade_in: bool,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProjectFadeCurve<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub fade_type: u32,
    pub decibel_volume_for_curve_at_start: i32,
    pub decibel_volume_for_curve_at_middle: i32,
    pub decibel_volume_for_curve_at_end: i32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UITextManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons: HashMap<Cow<'a, str>, TextIcon<'a>>,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons_phone: HashMap<Cow<'a, str>, Cow<'a, str>>,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_icons: Option<Vec<ActorIcon<'a>>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum TextIcon<'a> {
    #[serde(borrow, rename = "TextIcon_Default")]
    TextIconDefault(TextIconDefault<'a>),
    #[serde(borrow, rename = "TextIcon_Shortcut")]
    TextIconShortcut(TextIconShortcut<'a>),
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextIconDefault<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub data: IconActorData<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextIconShortcut<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub datas: HashMap<Cow<'a, str>, IconActorData<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IconActorData<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub path: Cow<'a, str>,
    pub font_size: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorIcon<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub icon_name: Cow<'a, str>,
    pub icon_path: Cow<'a, str>,
    pub font_size: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VibrationManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub vib_files_paths: Vec<Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFLinearRewards<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rewards: Vec<WDFReward<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFReward<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub id: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub inputs: Vec<Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub config: Cow<'a, str>,
    pub category: u32,
    pub actions: Vec<ZAction<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZAction<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub name: Cow<'a, str>,
    pub inverted: u32,
    pub scale: u32,
    pub input: Vec<ZInput<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ZInput<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub control: Cow<'a, str>,
    pub query: Cow<'a, str>,
    pub axis_range: (f32, f32),
    pub threshold: u32,
    pub delay: f32,
}

#[cfg(feature = "full_json_types")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AnthologyConfig<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub sku_map_name: Cow<'a, str>,
    pub planets_and_rocket_translation_speed: f32,
    pub ribbon_discovering_speed: f32,
    pub background_squares_translation_speed: f32,
    pub intro_video_path: Cow<'a, str>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub progression_videos_paths: HashMap<u32, Cow<'a, str>>,
    pub outro_video_path: Cow<'a, str>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub opus_textures_paths: HashMap<u32, Cow<'a, str>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsContainer<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub stats_events: Vec<StatsEvent<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsEvent<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub stats_event_name: Cow<'a, str>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_parameters: Vec<StatParameter<'a>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_user_stats: Vec<UserStat<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatParameter<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub parameter_name: Cow<'a, str>,
    pub parameter_type: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_value: Vec<VarType<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserStat<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub user_stat_name: Cow<'a, str>,
    pub user_stat_behaviour: u32,
    pub user_stat_used_on_xbox_one: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub user_stat_parameters: Vec<StatParameter<'a>>,
    pub parameter_used_to_update_value: Cow<'a, str>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardContainer<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub rewards: Vec<RewardDetail<'a>>,
    pub is_silent: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardDetail<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub platform_id: u32,
    pub event_trigger: Cow<'a, str>,
    pub has_to_be_checked_in_game: u32,
    pub uplay_id: i32,
    pub uplay_tag: u32,
    #[serde(rename = "uplayXP")]
    pub uplay_xp: u32,
    pub uplay_points_value: u32,
    #[serde(rename = "uplayLocID")]
    pub uplay_loc_id: u32,
    pub has_no_reward: u32,
    #[serde(rename = "REWARD_TRIGGER")]
    pub reward_trigger: Vec<RewardTriggerSum<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardTriggerSum<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub user_stat_name: Cow<'a, str>,
    pub amount_to_get: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values_to_check: Vec<StatParameter<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct VarType<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub var: u32,
}
