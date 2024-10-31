use std::collections::HashMap;

use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use ubiart_toolkit_shared_types::{Color, LocaleId};

use super::{v1819::ObjectiveDesc1819, DifficultyColors, Empty};
use crate::cooked::tpl::types::{
    AutodancePropData, FxEvent, PlaybackEvent, PropEvent, PropPlayerConfig,
};

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementsDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub achievements: Vec<AchievementDescriptor<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AchievementDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub platform_id: u32,
    pub uplay_id: u32,
    #[serde(borrow)]
    pub unlock_objective_desc_id: HipStr<'a>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocalAliases<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub locked_color: HipStr<'a>,
    #[serde(borrow)]
    pub difficulty_colors: DifficultyColors<'a>,
    #[serde(borrow)]
    pub aliases: Vec<UnlockableAliasDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct UnlockableAliasDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    #[serde(rename = "StringLocID")]
    pub string_loc_id: LocaleId,
    #[serde(rename = "StringLocIDFemale")]
    pub string_loc_id_female: LocaleId,
    #[serde(borrow)]
    pub string_online_localized: HipStr<'a>,
    #[serde(borrow)]
    pub string_online_localized_female: HipStr<'a>,
    #[serde(borrow)]
    pub string_placeholder: HipStr<'a>,
    pub unlocked_by_default: bool,
    #[serde(rename = "DescriptionLocID")]
    pub description_loc_id: LocaleId,
    #[serde(borrow)]
    pub description_localized: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub unlock_objective: Option<UnlockObjectiveOnlineInfo<'a>>,
    pub difficulty_color: Rarity,
    pub visibility: u32,
}

impl UnlockableAliasDescriptor<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_UnlockableAliasDescriptor");
}

impl Default for UnlockableAliasDescriptor<'static> {
    fn default() -> Self {
        Self {
            class: Some(UnlockableAliasDescriptor::CLASS),
            id: Default::default(),
            string_loc_id: LocaleId::default(),
            string_loc_id_female: LocaleId::default(),
            string_online_localized: HipStr::default(),
            string_online_localized_female: HipStr::default(),
            string_placeholder: HipStr::default(),
            unlocked_by_default: Default::default(),
            description_loc_id: LocaleId::default(),
            description_localized: HipStr::default(),
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

        impl serde::de::Visitor<'_> for RarityVisitor {
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub unlock_objective_desc_id: HipStr<'a>,
}

impl UnlockObjectiveOnlineInfo<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_UnlockObjectiveOnlineInfo");
}

impl Default for UnlockObjectiveOnlineInfo<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            unlock_objective_desc_id: HipStr::borrowed(""),
        }
    }
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub shakes: Vec<CameraShake<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShake<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub intensity: f32,
    pub duration: f32,
    pub ease_in_duration: f32,
    pub ease_out_duration: f32,
    #[serde(borrow)]
    pub shake_x: CameraShakeCurveParams<'a>,
    #[serde(borrow)]
    pub shake_y: CameraShakeCurveParams<'a>,
    #[serde(borrow)]
    pub shake_z: CameraShakeCurveParams<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CameraShakeCurveParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemPortraitBorder<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub portrait_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleGachaItemAlias<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub alias_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutSetup1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub contexts: HashMap<HipStr<'a>, ContextSetup1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextSetup1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub platforms: HashMap<HipStr<'a>, ShortcutDescriptorList1719<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDescriptorList1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    #[serde(borrow)]
    pub behaviour_name: HipStr<'a>,
    pub show_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ShortcutDesc1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    // Not in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_on_phone: Option<bool>,
    #[serde(borrow)]
    pub behaviour_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PopupConfigList<'a> {
    // In nx2017 this is not a class, but a regular hashmap
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<HipStr<'a>, PopupContentConfig<'a>>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub navigation: Option<HashMap<HipStr<'a>, PopupNavigationConfig<'a>>>,
    // Not used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub popup_description: Option<Vec<PopupParams<'a>>>,
    /// Retired after NX2019
    #[serde(rename = "menuDebugErrorList", default)]
    pub menu_debug_error_list: Option<Vec<u32>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub club_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub renew_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017, all caps on WiiU
    #[serde(
        borrow,
        default,
        skip_serializing_if = "Option::is_none",
        alias = "DEFAULT"
    )]
    pub default: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub check_cross: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub cross_check: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub none: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub overwrite_nosave: Option<PopupConfig<'a>>,
    // Only used in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub retry_nosave: Option<PopupConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupContentConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub content_scene_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupNavigationConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub left_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub middle_item: HipStr<'a>,
    #[serde(borrow)]
    pub right_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub up_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub bottom_item: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(default)]
    pub start_button_index: u32,
    #[serde(borrow)]
    pub phone_button_image: HipStr<'a>,
    pub phone_button_loc_id: u32,
    pub button_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PopupParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub popup_id: HipStr<'a>,
    #[serde(borrow)]
    pub content_key: HipStr<'a>,
    #[serde(borrow)]
    pub navigation_key: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(default)]
    pub full_screen_display: bool,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub popup_overriding_sound_context: HipStr<'a>,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub grid_overriding_sound_context: HipStr<'a>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub left_item: HipStr<'a>,
    #[serde(borrow)]
    pub right_item: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ClubRewardConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "locIdCR")]
    pub loc_id_cr: u32,
    #[serde(borrow, rename = "imgUrlCR")]
    pub img_url_cr: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScoringParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub asset_type: HipStr<'a>,
    pub max_assets: u32,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub default_assets: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub asset_path_fmts: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MenuMusicParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    pub prefetch: u32,
    pub fadein: u32,
    /// Not in 2016
    #[serde(borrow, default)]
    pub stinger: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub jingle: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RemoteSoundParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sound_id_for_phone: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMultiTrackItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub menu_music_path: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub stinger: HipStr<'a>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub jingle: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MenuMusicConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub loop_cross_fade_duration: f32,
    pub start_fade_duration: f32,
    pub multi_track_transition_beat_count: u32,
    pub end_of_loop_soundwich_notif_time_offset: u32,
    /// Only in 2016
    #[serde(borrow, default)]
    pub menu_music_multi_tracks: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct RankDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "maxRank")]
    pub max_rank: u32,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub color_look_up: HashMap<u32, u32>,
    pub rank_limits: Vec<u32>,
    // Not in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub gain_types: Option<HashMap<HipStr<'a>, u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestEntry1617<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub quest_id: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub locked: u32,
    pub trigger_end: u32,
    #[serde(borrow)]
    pub phone_image: HipStr<'a>,
    #[serde(borrow)]
    pub playlist: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub cover_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo_shaded_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSongList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(borrow)]
    pub artist: HipStr<'a>,
    #[serde(borrow, default)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct SystemDescriptor18<'a> {
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
    pub button_scene_id: HipStr<'a>,
    #[serde(borrow)]
    pub visual_scene_id: HipStr<'a>,
    pub boss_id: u32,
    #[serde(borrow)]
    pub planets: Vec<PlanetDescriptor18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetDescriptor18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub map_list: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub play_mode: HipStr<'a>,
    pub is_boss_planet: bool,
    pub is_surprise: bool,
    #[serde(borrow)]
    pub planet_objectives: Vec<PlanetObjectiveDesc18<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PlanetObjectiveDesc18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub objective_id: u32,
    pub mandatory: bool,
    pub rewards: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureBossDesc18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub avatar_id: u32,
    pub skin_id: u32,
    pub final_score: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AdventureModeSetup18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub video_paths: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestConfig1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "rankingScenePathID")]
    pub ranking_scene_path_id: HipStr<'a>,
    #[serde(borrow, rename = "rankingActorPathID")]
    pub ranking_actor_path_id: HipStr<'a>,
    pub difficulty_final_scores: Vec<(u32, u32)>,
    pub threshold_rank: u32,
    pub nb_challengers: u32,
    pub ranking_points_gain: Vec<u32>,
    pub mojo_per_star: u32,
    pub mojo_per_rank: Vec<(u32, u32, u32)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct QuestChallengerEntry1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub avatar_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlimitedUpsellSubtitles<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub subtitles: HipStr<'a>,
    pub subtitles_loc_ids: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CustomizableItemConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub minimum_score: u32,
    pub session_count_until_discovery_kill: u32,
    pub session_count_until_quest_kill: u32,
    pub session_count_until_first_discovery_kill: u32,
    pub session_count_until_normal_quest_setting: u32,
    pub first_discovery_quest_id: u32,
    /// Not used before NX2020
    #[serde(borrow, rename = "MapProbabilities", default)]
    pub map_probabilities: MapChoosingProbabilities<'a>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        borrow,
        rename = "MapProbabilitiesNX",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_nx: Option<MapChoosingProbabilities<'a>>,
    /// Superseded by `map_probabilities` in NX2020 and later
    #[serde(
        borrow,
        rename = "MapProbabilitiesOther",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub map_probabilities_other: Option<MapChoosingProbabilities<'a>>,
    #[serde(rename = "PushSongProbability")]
    pub push_song_probability: u32,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub selection_pre_conditions: Option<HashMap<HipStr<'a>, Vec<u32>>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub update_timings: HashMap<u32, f32>,
    pub time_cap_in_hours_to_renew: u32,
    /// Introduced in NX2020
    #[serde(borrow, default)]
    pub exclude_from_algorithm_quest_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct MapChoosingProbabilities<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub increase_priority_tag: HashMap<HipStr<'a>, u32>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub special_priority: HashMap<u32, DanceMachinePriority17<'a>>,
    pub bonus_stage_min_value: u32,
    pub bonus_stage_max_value: u32,
    pub default_increase_priority_value: u32,
    pub bonus_priority_for_lowest_played_blocks: u32,
    pub delta_for_unlock_unlimited: u32,
    #[serde(borrow)]
    pub reward_tag_order: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachinePriority17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub priority_map: HashMap<HipStr<'a>, i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct DanceMachineGlobalConfig1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub video_path: Option<HashMap<HipStr<'a>, Vec<HipStr<'a>>>>,
    // Only in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub anim_syncrho: Option<HashMap<HipStr<'a>, HipStr<'a>>>,
    // Only in nx2017
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub first_experience: Option<Vec<HipStr<'a>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SweatRandomizeConfig1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub weights: HashMap<HipStr<'a>, Vec<f32>>,
    #[serde(borrow)]
    pub excluded_tags: Vec<HipStr<'a>>,
    pub seed_range: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SearchConfig1719<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
pub struct ChallengerScoreEvolutionTemplate1619<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub template_name: HipStr<'a>,
    #[serde(borrow)]
    pub template_descriptor: HashMap<HipStr<'a>, f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CountryEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub loc_id: u32,
    #[serde(borrow)]
    pub code: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub region: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChatMessagesParams1618<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
#[serde(deny_unknown_fields, untagged)]
pub enum AutoDanceEffectData<'a> {
    #[serde(borrow)]
    JD1722(AutoDanceEffectData1722<'a>),
    #[serde(borrow)]
    JD16(AutoDanceEffectData16<'a>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoDanceEffectData1722<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "video_structure")]
    pub video_structure: Box<AutodanceVideoStructure<'a>>,
    pub effect_type: u32,
    #[serde(borrow)]
    pub effect_id: HipStr<'a>,
    /// Only in 2016
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub effect_map_name: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AutoDanceEffectData16<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub effect_type: u32,
    #[serde(borrow)]
    pub effect_map_name: HipStr<'a>,
    #[serde(borrow)]
    pub effect_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AutodanceVideoStructure<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    /// Only in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<u32>,
    pub song_start_position: f32,
    pub duration: f32,
    pub thumbnail_time: u32,
    pub fade_out_duration: f32,
    /// Only in 2016
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub animated_frame_path: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub ground_plane_path: HipStr<'a>,
    #[serde(borrow)]
    pub first_layer_triple_background_path: HipStr<'a>,
    #[serde(borrow)]
    pub second_layer_triple_background_path: HipStr<'a>,
    #[serde(borrow)]
    pub third_layer_triple_background_path: HipStr<'a>,
    #[serde(
        borrow,
        rename = "playback_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub playback_events: Vec<PlaybackEvent<'a>>,
    #[serde(borrow, rename = "background_effect")]
    pub background_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        borrow,
        rename = "background_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub background_effect_events: Vec<FxEvent<'a>>,
    #[serde(borrow, rename = "player_effect")]
    pub player_effect: Box<AutoDanceFxDesc<'a>>,
    #[serde(
        borrow,
        rename = "player_effect_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub player_effect_events: Vec<FxEvent<'a>>,
    #[serde(
        borrow,
        rename = "prop_events",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub prop_events: Vec<PropEvent<'a>>,
    #[serde(
        borrow,
        rename = "props",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub props: Vec<AutodancePropData<'a>>,
    #[serde(
        borrow,
        rename = "props_players_config",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub props_players_config: Vec<PropPlayerConfig<'a>>,
}

impl AutodanceVideoStructure<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_AutodanceVideoStructure");
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AutoDanceFxDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub opacity: f32,
    #[serde(borrow)]
    pub color_low: GFXVector4<'a>,
    #[serde(borrow)]
    pub color_mid: GFXVector4<'a>,
    #[serde(borrow)]
    pub color_high: GFXVector4<'a>,
    pub low_to_mid: f32,
    pub low_to_mid_width: f32,
    pub mid_to_high: f32,
    pub mid_to_high_width: f32,
    #[serde(borrow)]
    pub sob_color: GFXVector4<'a>,
    #[serde(borrow)]
    pub out_color: GFXVector4<'a>,
    pub thick_middle: f32,
    pub thick_inner: f32,
    pub thick_smooth: f32,
    pub shv_nb_frames: u32,
    pub parts_scale: Vec<u32>,
    pub halftone_factor: u32,
    pub halftone_cutout_levels: f32,
    #[serde(rename = "UVBlackoutFactor")]
    pub uv_blackout_factor: u32,
    #[serde(rename = "UVBlackoutDesaturation")]
    pub uv_blackout_desaturation: f32,
    #[serde(rename = "UVBlackoutContrast")]
    pub uv_blackout_contrast: f32,
    #[serde(rename = "UVBlackoutBrightness")]
    pub uv_blackout_brightness: u32,
    #[serde(borrow, rename = "UVBlackoutColor")]
    pub uv_blackout_color: GFXVector4<'a>,
    pub toon_factor: u32,
    pub toon_cutout_levels: f32,
    pub refraction_factor: u32,
    #[serde(borrow)]
    pub refraction_tint: GFXVector4<'a>,
    #[serde(borrow)]
    pub refraction_scale: GFXVector4<'a>,
    pub refraction_opacity: f32,
    #[serde(borrow)]
    pub colored_shiva_thresholds: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_0: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_1: GFXVector4<'a>,
    #[serde(borrow)]
    pub colored_shiva_color_2: GFXVector4<'a>,
    pub saturation_modifier: f32,
    pub slime_factor: f32,
    #[serde(borrow)]
    pub slime_color: GFXVector4<'a>,
    pub slime_opacity: f32,
    pub slime_ambient: f32,
    pub slime_normal_tiling: f32,
    pub slime_light_angle: f32,
    pub slime_refraction: f32,
    pub slime_refraction_index: f32,
    pub slime_specular: f32,
    pub slime_specular_power: f32,
    pub overlay_blend_factor: f32,
    #[serde(borrow)]
    pub overlay_blend_color: GFXVector4<'a>,
    pub background_sobel_factor: f32,
    #[serde(borrow)]
    pub background_sobel_color: GFXVector4<'a>,
    pub player_glow_factor: f32,
    #[serde(borrow)]
    pub player_glow_color: GFXVector4<'a>,
    pub swap_head_with_player: Vec<u32>,
    pub animate_player_head: Vec<u32>,
    pub animated_head_total_time: f32,
    pub animated_head_rest_time: f32,
    pub animated_head_frame_time: f32,
    pub animated_head_max_distance: f32,
    pub animated_head_max_angle: f32,
    pub screen_blend_inverse_alpha_factor: u32,
    pub screen_blend_inverse_alpha_scale_x: f32,
    pub screen_blend_inverse_alpha_scale_y: f32,
    pub screen_blend_inverse_alpha_trans_x: u32,
    pub screen_blend_inverse_alpha_trans_y: u32,
    pub tint_mul_color_factor: u32,
    #[serde(borrow)]
    pub tint_mul_color: GFXVector4<'a>,
    pub floor_plane_factor: f32,
    #[serde(borrow)]
    pub floor_plane_tiles: GFXVector4<'a>,
    pub floor_speed_x: f32,
    pub floor_speed_y: f32,
    pub floor_wave_speed: f32,
    pub floor_blend_mode: u32,
    #[serde(rename = "FloorPlaneImageID")]
    pub floor_plane_image_id: u32,
    pub start_radius: f32,
    pub end_radius: f32,
    pub radius_variance: f32,
    pub radius_noise_rate: u32,
    pub radius_noise_amp: f32,
    pub min_spin: f32,
    pub max_spin: f32,
    pub dir_angle: f32,
    pub min_wander_rate: f32,
    pub max_wander_rate: f32,
    pub min_wander_amp: f32,
    pub max_wander_amp: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub motion_power: f32,
    pub amount: f32,
    #[serde(rename = "ImageID")]
    pub image_id: u32,
    pub start_r: f32,
    pub start_g: f32,
    pub start_b: f32,
    pub end_r: f32,
    pub end_g: f32,
    pub end_b: f32,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub textured_outline_factor: u32,
    pub textured_outline_tiling: f32,
    pub triple_layer_background_factor: u32,
    #[serde(borrow)]
    pub triple_layer_background_tint_color: GFXVector4<'a>,
    pub triple_layer_background_speed_x: u32,
    pub triple_layer_background_speed_y: u32,
    pub trail_effect_id: u32,
}

impl AutoDanceFxDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("AutoDanceFxDesc");
}

impl Default for AutoDanceFxDesc<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            opacity: 1.0,
            color_low: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            color_mid: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            color_high: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            low_to_mid: 0.333,
            low_to_mid_width: 0.15,
            mid_to_high: 0.666,
            mid_to_high_width: 0.15,
            sob_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            out_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            thick_middle: 0.4,
            thick_inner: 0.1,
            thick_smooth: 0.1,
            shv_nb_frames: 0,
            parts_scale: vec![0, 0, 0, 0, 0],
            halftone_factor: 0,
            halftone_cutout_levels: 256.0,
            uv_blackout_factor: 0,
            uv_blackout_desaturation: 0.2,
            uv_blackout_contrast: 4.0,
            uv_blackout_brightness: 0,
            uv_blackout_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.54902,
                y: 0.54902,
                z: 1.0,
                w: 1.0,
            },
            toon_factor: 0,
            toon_cutout_levels: 256.0,
            refraction_factor: 0,
            refraction_tint: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            refraction_scale: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.03,
                y: 0.03,
                z: 0.03,
                w: 0.03,
            },
            refraction_opacity: 0.2,
            colored_shiva_thresholds: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.1,
                y: 0.3,
                z: 0.6,
                w: 0.95,
            },
            colored_shiva_color_0: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            colored_shiva_color_1: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            colored_shiva_color_2: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 1.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            saturation_modifier: 0.0,
            slime_factor: 0.0,
            slime_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.49902,
                y: 0.629_176,
                z: 0.136_039,
                w: 1.0,
            },
            slime_opacity: 0.2,
            slime_ambient: 0.2,
            slime_normal_tiling: 7.0,
            slime_light_angle: 0.0,
            slime_refraction: 0.0913,
            slime_refraction_index: 1.0837,
            slime_specular: 1.0,
            slime_specular_power: 10.0,
            overlay_blend_factor: 0.0,
            overlay_blend_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.721_569,
                y: 0.639_216,
                z: 0.756_863,
                w: 1.0,
            },
            background_sobel_factor: 0.0,
            background_sobel_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            player_glow_factor: 0.0,
            player_glow_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 1.0,
                z: 1.0,
                w: 1.0,
            },
            swap_head_with_player: vec![0, 1, 2, 3, 4, 5],
            animate_player_head: vec![0, 0, 0, 0, 0, 0],
            animated_head_total_time: 20.0,
            animated_head_rest_time: 16.0,
            animated_head_frame_time: 0.6,
            animated_head_max_distance: 1.25,
            animated_head_max_angle: 1.2,
            screen_blend_inverse_alpha_factor: 0,
            screen_blend_inverse_alpha_scale_x: 0.0,
            screen_blend_inverse_alpha_scale_y: 0.0,
            screen_blend_inverse_alpha_trans_x: 0,
            screen_blend_inverse_alpha_trans_y: 0,
            tint_mul_color_factor: 0,
            tint_mul_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            floor_plane_factor: 0.0,
            floor_plane_tiles: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 8.0,
                y: 8.0,
                z: 0.0,
                w: 0.0,
            },
            floor_speed_x: 0.0,
            floor_speed_y: 0.0,
            floor_wave_speed: 0.0,
            floor_blend_mode: 0,
            floor_plane_image_id: 0,
            start_radius: 3.0,
            end_radius: 2.0,
            radius_variance: 0.5,
            radius_noise_rate: 0,
            radius_noise_amp: 0.0,
            min_spin: -4.0,
            max_spin: 4.0,
            dir_angle: 0.0,
            min_wander_rate: 2.0,
            max_wander_rate: 3.0,
            min_wander_amp: 0.1,
            max_wander_amp: 0.2,
            min_speed: 0.2,
            max_speed: 0.4,
            motion_power: 1.5,
            amount: 0.0,
            image_id: 7,
            start_r: 1.0,
            start_g: 0.1,
            start_b: 0.1,
            end_r: 0.1,
            end_g: 0.2,
            end_b: 1.0,
            start_alpha: 1.0,
            end_alpha: 1.0,
            textured_outline_factor: 0,
            textured_outline_tiling: 1.0,
            triple_layer_background_factor: 0,
            triple_layer_background_tint_color: GFXVector4 {
                class: Some(GFXVector4::CLASS),
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            triple_layer_background_speed_x: 0,
            triple_layer_background_speed_y: 0,
            trail_effect_id: 0,
        }
    }
}

/// For serde to set a value to default to `u32::MAX`
const fn u32_max() -> u32 {
    u32::MAX
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GFXVector4<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl GFXVector4<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("GFX_Vector4");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CoopTweakedText17<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub min_score: f32,
    pub title: u32,
    #[serde(default = "u32_max")]
    pub title_one_player: u32,
    pub desc: u32,
    #[serde(default = "u32_max")]
    pub desc_one_player: u32,
    #[serde(borrow)]
    pub sound_notification: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialContent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub popup: u32,
    pub browsable: u32,
    pub slide_delay: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<HipStr<'a>>,
    #[serde(borrow)]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub message_id: u32,
    #[serde(borrow)]
    pub image_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MessageFocusDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub focus_id: u32,
    pub loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct TutorialDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tutorial_context: HipStr<'a>,
    #[serde(borrow)]
    pub game_context: HipStr<'a>,
    // Named Messages before nx2019
    #[serde(borrow, alias = "Messages")]
    pub contents: Vec<HipStr<'a>>,
    pub priority: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub mandatory_song_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub optional_song_tags: Vec<HipStr<'a>>,
    pub max_display: i32,
    pub max_display_per_session: i32,
    /// Not used after NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_ftue_step: Option<i32>,
    /// Introduced in NX2019
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_dependant: Option<bool>,
    /// Introduced in NX2020
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_string: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UplayReward<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub reward_id: u32,
    #[serde(borrow)]
    pub reward_name: HipStr<'a>,
    pub reward_type: u32,
    pub amount_to_unlock: u32,
    #[serde(borrow)]
    pub reward_string_on_server: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFBossEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub boss_id: HipStr<'a>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    #[serde(borrow)]
    pub logo: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct AdventureObjective18<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(borrow)]
    pub objective: ObjectiveDesc1819<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorLookUp<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub descriptor: HashMap<HipStr<'a>, ItemColorMap<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemColorMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub colors: HashMap<HipStr<'a>, Color>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoLoopSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "videoFPS")]
    pub video_fps: u32,
    #[serde(borrow)]
    pub descriptors: HashMap<HipStr<'a>, VideoBrickDescriptor<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoBrickDescriptor<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub local_start_frame: u32,
    pub local_end_frame: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HueConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub menu_color: Color,
    pub gold_effect_color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbum<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_page_delay: Option<f32>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub bonus_page_unlock_objective_id: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub pages: Vec<CollectibleAlbumPage<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumPage<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CollectibleAlbumItem<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    // Not present in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub carousel_item_scene_id: HipStr<'a>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    pub sticker_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemCustomizable<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    pub customizable_item_id: u32,
    pub customizable_item_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemPostcard<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub postcard_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemMap<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CollectibleAlbumItemJDM<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub rarity: u32,
    #[serde(borrow)]
    pub episode_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StickerEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sticker_id: u32,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub objective_id: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<HipStr<'a>>,
    // Not used in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub scene_path: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub price: u32,
    pub nb_max_history_pickup_reward: u32,
    #[serde(borrow)]
    pub reward_unlock_scenes: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub rarity_pickup_percentage: HashMap<u32, u32>,
    pub force_high_rarity_reward_count: u32,
    pub force_mojo_reward_count: u32,
    // Not used in nx2019 or later
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub mojo_reward_list: Option<Vec<GachaMojoRewardConfig<'a>>>,
    #[serde(borrow)]
    pub puzzle_map_reward: HipStr<'a>,
    pub nb_maps_threshold_before_push_gacha_screen: (u32, u32),
    // Not used in nx2018 or before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_play_count_between_map_rewards: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GachaMojoRewardConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub mojo_amount: u32,
    pub number_of_packs: u32,
    pub rarity: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not in nx2018 or before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ordered_steps: Option<Vec<StepInfo<'a>>>,
    #[serde(borrow)]
    pub songs_to_be_kept_unlocked: Vec<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub step: u32,
    pub map_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RumbleConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sleep_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub parent_marker: HipStr<'a>,
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
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GridActorsToPreload<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actors_to_preload: Vec<ActorsToPreload<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorsToPreload<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scene_path: HipStr<'a>,
    #[serde(borrow)]
    pub actor_path: HipStr<'a>,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LayoutTabbedGrids<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tabbed_grid_descs: Vec<TabbedGridDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TabbedGridDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tab_name: HipStr<'a>,
    #[serde(borrow)]
    pub tab_hover_tutorial_context: HipStr<'a>,
    #[serde(borrow)]
    pub tab_content_tutorial_context: HipStr<'a>,
    pub start_index: u32,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub grid_actors_to_preload_id: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner_trigger_time: Option<f32>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_preview_time: Option<f32>,
    // Not used in nx2020 and later
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub grid_desc_id: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub elements_list_by_visual_type: HashMap<HipStr<'a>, [HipStr<'a>; 5]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeDataTipEntry<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tip_id: HipStr<'a>,
    pub catch_phrase: u32,
    pub content: u32,
    #[serde(borrow)]
    pub thumbnail: HipStr<'a>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<HipStr<'a>>,
    // Not used in nx2020 and later
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_by_first_time_layout: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HomeVideoDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub video_thumbnail_path: HipStr<'a>,
    #[serde(borrow)]
    pub video_path: HipStr<'a>,
    pub video_tile_title_loc_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongsSearchTags<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub maps: HashMap<HipStr<'a>, SongSearchTags<'a>>,
}

impl SongsSearchTags<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongsSearchTags");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTags<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub tags: Vec<SongSearchTag<'a>>,
}

impl SongSearchTags<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongSearchTags");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongSearchTag<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub tag_loc_id: LocaleId,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
}

impl SongSearchTag<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_SongSearchTag");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct GroupsSoundNotificationConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "delay_before_sendingEvent_enterNewGroupStartTimer")]
    pub delay_before_sending_event_enter_new_group_start_timer: f32,
    #[serde(rename = "delay_before_sendingEvent_changeGroup")]
    pub delay_before_sending_event_change_group: f32,
    #[serde(borrow)]
    pub items_group: Vec<ItemsGroup<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ItemsGroup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub group_name: HipStr<'a>,
    pub items_indexes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OnFlyNotificationTypeParams<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub default_form_and_timing: FormAndTiming<'a>,
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub specific_cases_form_and_timing: HashMap<HipStr<'a>, FormAndTiming<'a>>,
    pub bubble_title_loc_id: u32,
    pub reward_screen_title_loc_id: u32,
    pub specific_content_loc_id: u32,
    pub forbid_reward_screen_flow_jump_button: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FormAndTiming<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub form: u32,
    pub timing: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RecapConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub gauge_number_of_stars_per_beat: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfigs<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub configs: Vec<WhatsNewConfig<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WhatsNewConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub config_name: HipStr<'a>,
    #[serde(borrow)]
    pub ui_display: HipStr<'a>,
    pub max_views: u32,
    pub session_interval: u32,
    #[serde(borrow)]
    pub related_song_tags: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub subscribed_grid_desc: HipStr<'a>,
    #[serde(borrow)]
    pub unsubscribed_grid_desc: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    /// Not in 2016
    #[serde(borrow, default)]
    pub anim_setups: HashMap<HipStr<'a>, CarouselAnimSetup<'a>>,
    #[serde(borrow)]
    pub carousel_descs: HashMap<HipStr<'a>, CarouselDesc<'a>>,
    #[serde(borrow)]
    pub item_object: HashMap<HipStr<'a>, HipStr<'a>>,
    #[serde(borrow)]
    pub item_logic: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselAnimSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub start_index: u32,
    pub is_loop: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescAction<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionActivateConnection<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub connection: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePage<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionSelectChallengeMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionRematchChallenge<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageFromHomeTile<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    // Not present in nx2019 and before
    #[serde(
        rename = "needToSetJDUDestination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub need_to_set_jdu_destination: Option<bool>,
    // Not present in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_type: Option<HipStr<'a>>,
    // Not present in nx2020 and after
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub tracking_tile_sub_type: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangePageWithContext<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionChangeCluster<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionEnterGameMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGotoGacha<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    pub gacha_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionGoto<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescActionLaunchJDTV<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    #[serde(rename = "locID")]
    pub loc_id: u32,
    #[serde(borrow)]
    pub banner_setup: BannerSetup<'a>,
    #[serde(borrow)]
    pub tag: HipStr<'a>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub destination: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct BannerSetup<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "Type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub theme: HipStr<'a>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescBase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    // Not used in nx2019 and before
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_x: Option<u32>,
    // Not used in nx2020 and after
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub half_size_y: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CarouselElementDesc<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescTabItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<CarouselElementDescComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselElementDescCarousel<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not used in nx2019 and before
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub autotest_friendly_name: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_object: HipStr<'a>,
    #[serde(borrow)]
    pub item_logic: HipStr<'a>,
    pub enabled: u32,
    #[serde(borrow, rename = "carouselDescID")]
    pub carousel_desc_id: HipStr<'a>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub catage: u32,
    #[serde(borrow)]
    pub phone_image_path: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentAmiibo<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub avatar_id: u32,
    pub character_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCluster<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
    #[serde(borrow)]
    pub transition: HipStr<'a>,
    #[serde(borrow)]
    pub news_placement: HipStr<'a>,
    pub availability_check: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGender<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub gender: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSweatMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sweat_mode: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentQuestDifficulty<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub difficulty: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentTauntCategory<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub messages: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub patch_marker: HipStr<'a>,
    #[serde(borrow)]
    pub indicator_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentNewMarkerTab<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub patch_marker: HipStr<'a>,
    #[serde(borrow)]
    pub tab_name: HipStr<'a>,
    pub clean_rule: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentGameMode<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub context: HipStr<'a>,
    #[serde(borrow)]
    pub transition: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentDevice<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub scoring_type: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentWDFVoteChoice<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub vote: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentSoundNotification<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_context: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub sound_notification_prefix: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub module_name: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub focus_notification: Option<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct JdCarouselElementDescComponentCompletionDisplay<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub marker: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffectList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub effects: Vec<FontEffect<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FontEffect<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub ftue_steps_descs: Vec<FTUEStepDesc<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FTUEStepDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub step_name: HipStr<'a>,
    #[serde(borrow)]
    pub step_done_objective_id: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRules<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_lists: HashMap<HipStr<'a>, ActionList<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub song_item_lists: Option<HashMap<HipStr<'a>, SongItemList<'a>>>,
    #[serde(borrow)]
    pub rules: HashMap<HipStr<'a>, CarouselRule<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_type: HipStr<'a>,
    #[serde(borrow)]
    pub actions: Vec<Action<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub title_id: u32,
    pub online_title_id: u32,
    #[serde(borrow)]
    pub target: HipStr<'a>,
    #[serde(borrow)]
    pub banner_type: HipStr<'a>,
    #[serde(borrow)]
    pub banner_theme: HipStr<'a>,
    #[serde(borrow)]
    pub banner_context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItemList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(borrow)]
    pub list: Vec<SongItem<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRule<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub categories: Vec<CategoryRule<'a>>,
    pub online_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CategoryRule<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub title_id: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<CarouselRequestDesc<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

impl CategoryRule<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("CategoryRule");
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
    #[serde(borrow, rename = "JD_CarouselGalaxyRequestDesc")]
    Galaxy(CarouselGalaxyRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselPlaylistRequestDesc")]
    Playlist(CarouselPlaylistRequestDesc<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselGalaxyRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub sub_type: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselCustomizableItemRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub uploading: bool,
    pub offline: bool,
    pub most_liked: bool,
    pub most_viewed: bool,
    pub featured: bool,
    pub query_pid: bool,
    pub player_pid: bool,
    pub friend_pids: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselMapRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(rename = "originalJDVersion")]
    pub original_jd_version: u32,
    pub coach_count: u32,
    #[serde(borrow)]
    pub order: HipStr<'a>,
    pub subscribed: bool,
    /// Not in 2016
    #[serde(default)]
    pub favorites: bool,
    /// Only included in nx2019
    pub sweat_toggle_item: Option<bool>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub included_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub custom_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub optional_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselDancerCardRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub main: bool,
    pub create: bool,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_save_item: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselItemRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default)]
    pub item_list: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPlaylistsRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow, rename = "playlistID")]
    pub playlist_id: HipStr<'a>,
}

impl Default for CarouselPlaylistsRequestDesc<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            isc: HipStr::borrowed("grp_row"),
            act: HipStr::borrowed("ui_carousel"),
            type_it: HipStr::borrowed("edito-pinned"),
            playlist_id: HipStr::borrowed(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPhotoRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPlaylistRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub create: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselQuestRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub start_action: HipStr<'a>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselSkuFilter<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub game_version: HipStr<'a>,
    #[serde(borrow)]
    pub platform: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct TRCLocalisation<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "TRCLocalisationList")]
    pub trc_localisation_list: Vec<TRCLocalisationDetail<'a>>,
    #[serde(borrow)]
    pub popups_illustrations: HashMap<HipStr<'a>, HipStr<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TRCLocalisationDetail<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "TRCError")]
    pub trc_error: u32,
    #[serde(borrow)]
    pub illustration_id: HipStr<'a>,
    #[serde(borrow)]
    pub title: SmartLocId<'a>,
    #[serde(borrow)]
    pub message: SmartLocId<'a>,
    #[serde(borrow)]
    pub button_left: SmartLocId<'a>,
    #[serde(borrow)]
    pub button_middle: SmartLocId<'a>,
    #[serde(borrow)]
    pub button_right: SmartLocId<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SmartLocId<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub loc_id: u32,
    #[serde(borrow)]
    pub default_text: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ObjectivesDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub objective_descs: HashMap<HipStr<'a>, ObjectiveDescriptor<'a>>,
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
    pub fn description_raw(&self) -> HipStr<'a> {
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAccumulateXCal<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub moves_count: u32,
    pub categories_to_count: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorAddXSongsToAPlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub songs_added_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorChangeCustoItemXTimes<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub custo_item_changes_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorCompleteXQuests<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub quests_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorDanceXSeconds<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub dance_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorFinishXPlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub playlists_play_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorGatherXStars<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub stars_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayDailyQuestsForXDays<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub maps_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorPlayXWDFTournamentRounds<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub rounds_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorReachRankX<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<ObjectiveDescriptorComponent<'a>>,
    #[serde(rename = "Static")]
    pub is_static: bool,
    pub exclude_from_upload: bool,
    pub portrait_border_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorUnlockXStickers<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub description: LocaleId,
    #[serde(borrow)]
    pub description_raw: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentCustoItemTypeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_custo_item_types: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentGachaItemTypeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_gacha_item_types: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapCoachCountRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_coach_counts: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapLaunchLocationRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_contexts: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_launch_subcontexts: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapMovesRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub exact_moves_count: u32,
    pub min_moves_count: u32,
    pub max_moves_count: u32,
    pub all_map_moves_count: bool,
    pub only_map_last_move: bool,
    pub moves_in_a_row: bool,
    pub acceptable_categories: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapNameRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_names: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapPlaymodeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub score: u32,
    pub better_than_dancer_of_the_week: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentMapTagsRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub acceptable_map_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub unacceptable_map_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentPlaylistIdRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    #[serde(borrow)]
    pub acceptable_playlist_ids: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentScoringModeRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_scoring_modes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentSearchLabelsRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_label_loc_ids: Vec<LocaleId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forbidden_label_loc_ids: Vec<LocaleId>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDescriptorComponentStickerIdRequirement<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub only_diff_values: bool,
    pub acceptable_sticker_ids: Vec<u32>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumbleManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rumbles: Vec<PadRumble<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PadRumble<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub intensity: f32,
    pub duration: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlaylistDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub playlists: HashMap<HipStr<'a>, OfflinePlaylist<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OfflinePlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub title_id: LocaleId,
    pub description_id: LocaleId,
    #[serde(borrow)]
    pub cover_path: HipStr<'a>,
    #[serde(borrow)]
    pub maps: Vec<HipStr<'a>>,
}

impl OfflinePlaylist<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("OfflinePlaylist");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBordersDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub portrait_borders: Vec<PortraitBorderDesc<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PortraitBorderDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub portrait_border_id: u32,
    #[serde(borrow)]
    pub background_texture_path: HipStr<'a>,
    #[serde(borrow)]
    pub foreground_texture_path: HipStr<'a>,
    #[serde(borrow)]
    pub background_phone_path: HipStr<'a>,
    #[serde(borrow)]
    pub foreground_phone_path: HipStr<'a>,
    pub original_lock_status: u32,
    pub visibility: u32,
}

impl PortraitBorderDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_PortraitBorderDesc");
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRules<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rules_by_priority_order: Vec<QuickplayRule<'a>>,
    #[serde(
        borrow,
        rename = "DEBUG_TEST_LIST",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub debug_test_list: Vec<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub max_number_of_previous_maps_to_consider: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct QuickplayRuleEnforceEasyMapAtStart<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub max_number_of_previous_maps_to_consider: u32,
    #[serde(borrow, rename = "acceptableTags")]
    pub acceptable_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScheduledQuestDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub scheduled_quests: Vec<ScheduledQuestDesc<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ScheduledQuestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Type")]
    pub type_it: u8,
    pub unlimited_only: bool,
    pub mojo_reward: u32,
    pub probability_weight: u32,
    pub objective_id: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions_objectives_id: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<HipStr<'a>>,
}

impl ScheduledQuestDesc<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_ScheduledQuestDesc");
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SoundConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub busses: Vec<BusDef<'a>>,
    #[serde(borrow)]
    pub limiters: Vec<LimiterDef<'a>>,
    pub pause_fade_in: f32,
    pub pause_fade_out: f32,
    #[serde(borrow)]
    pub headphone_bus_mix: BusMix<'a>,
    #[serde(borrow)]
    pub music_tracks: Vec<MusicTrackDef<'a>>,
    #[serde(borrow, rename = "PCMSources")]
    pub pcm_sources: Vec<PCMSourceDef<'a>>,
    #[serde(borrow)]
    pub metronome_debug_sound_bar: HipStr<'a>,
    #[serde(borrow)]
    pub metronome_debug_sound_beat: HipStr<'a>,
    #[serde(borrow, rename = "TestSounds")]
    pub test_sounds: Vec<HipStr<'a>>,
    #[serde(borrow)]
    pub bus_mix_list: Vec<EventBusMix<'a>>,
    #[serde(borrow)]
    pub bus_fade_list: Vec<EventBusFade<'a>>,
    #[serde(borrow)]
    pub soundwich_synth: HipStr<'a>,
    #[serde(borrow)]
    pub soundwich_modules: Vec<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub project_fade_curves: Option<Vec<ProjectFadeCurve<'a>>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<HipStr<'a>>,
    pub volume: i32,
    pub filter_frequency: u32,
    pub filter_type: u32,
    pub out_devices: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LimiterDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub max_instances: u32,
    pub mode: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BusMix<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub priority: u32,
    pub duration: f32,
    pub fade_in: f32,
    pub fade_out: f32,
    // Not on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_mix: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub bus_defs: Vec<BusDef<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MusicTrackDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub track_name: HipStr<'a>,
    #[serde(borrow)]
    pub bus_name: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PCMSourceDef<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub nb_channels: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusMix<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sender: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub activate: u32,
    #[serde(borrow)]
    pub bus_mix: BusMix<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EventBusFade<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub sender: u32,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub bus: HipStr<'a>,
    pub time: f32,
    pub fade_in: bool,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProjectFadeCurve<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub fade_type: u32,
    pub decibel_volume_for_curve_at_start: i32,
    pub decibel_volume_for_curve_at_middle: i32,
    pub decibel_volume_for_curve_at_end: i32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UITextManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons: HashMap<HipStr<'a>, TextIcon<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "HashMap::is_empty")]
    pub text_icons_phone: HashMap<HipStr<'a>, HipStr<'a>>,
    // Only on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
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
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub data: IconActorData<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TextIconShortcut<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub datas: HashMap<HipStr<'a>, IconActorData<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct IconActorData<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub path: HipStr<'a>,
    pub font_size: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActorIcon<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub icon_name: HipStr<'a>,
    #[serde(borrow)]
    pub icon_path: HipStr<'a>,
    pub font_size: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VibrationManager<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub vib_files_paths: Vec<HipStr<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFLinearRewards<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rewards: Vec<WDFReward<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFReward<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub id: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub inputs: Vec<HipStr<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZInputManager<'a> {
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
    pub config: HipStr<'a>,
    pub category: u32,
    #[serde(borrow)]
    pub actions: Vec<ZAction<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ZAction<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub inverted: u32,
    pub scale: u32,
    #[serde(borrow)]
    pub input: Vec<ZInput<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ZInput<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub control: HipStr<'a>,
    #[serde(borrow)]
    pub query: HipStr<'a>,
    pub axis_range: (f32, f32),
    pub threshold: u32,
    pub delay: f32,
}

#[cfg(feature = "full_json_types")]
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AnthologyConfig<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub sku_map_name: HipStr<'a>,
    pub planets_and_rocket_translation_speed: f32,
    pub ribbon_discovering_speed: f32,
    pub background_squares_translation_speed: f32,
    #[serde(borrow)]
    pub intro_video_path: HipStr<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub progression_videos_paths: HashMap<u32, HipStr<'a>>,
    #[serde(borrow)]
    pub outro_video_path: HipStr<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub opus_textures_paths: HashMap<u32, HipStr<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsContainer<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub stats_events: Vec<StatsEvent<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatsEvent<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub stats_event_name: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_parameters: Vec<StatParameter<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub stats_event_user_stats: Vec<UserStat<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StatParameter<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub parameter_name: HipStr<'a>,
    pub parameter_type: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub parameter_value: Vec<VarType<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UserStat<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub user_stat_name: HipStr<'a>,
    pub user_stat_behaviour: u32,
    pub user_stat_used_on_xbox_one: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub user_stat_parameters: Vec<StatParameter<'a>>,
    #[serde(borrow)]
    pub parameter_used_to_update_value: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardContainer<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rewards: Vec<RewardDetail<'a>>,
    pub is_silent: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardDetail<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub id: HipStr<'a>,
    #[serde(borrow)]
    pub name: HipStr<'a>,
    pub platform_id: u32,
    #[serde(borrow)]
    pub event_trigger: HipStr<'a>,
    pub has_to_be_checked_in_game: u32,
    pub uplay_id: i32,
    /// Not in 2016
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uplay_tag: Option<u32>,
    #[serde(rename = "uplayXP", default, skip_serializing_if = "Option::is_none")]
    pub uplay_xp: Option<u32>,
    pub uplay_points_value: u32,
    #[serde(rename = "uplayLocID")]
    pub uplay_loc_id: u32,
    pub has_no_reward: u32,
    #[serde(
        borrow,
        rename = "REWARD_TRIGGER",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reward_trigger: Vec<RewardTriggerSum<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RewardTriggerSum<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub user_stat_name: HipStr<'a>,
    pub amount_to_get: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub values_to_check: Vec<StatParameter<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct VarType<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub var: u32,
}
