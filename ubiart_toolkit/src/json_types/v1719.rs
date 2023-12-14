use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use super::{isg::Rarity, DifficultyColors};
use crate::utils::LocaleId;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LocalAliases1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub locked_color: Cow<'a, str>,
    pub difficulty_colors: DifficultyColors<'a>,
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub aliases: HashMap<u16, UnlockableAliasDescriptor1719<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct UnlockableAliasDescriptor1719<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "StringLocID")]
    pub string_loc_id: LocaleId,
    pub string_online_localized: Cow<'a, str>,
    pub string_placeholder: Cow<'a, str>,
    pub difficulty_color: Rarity,
    pub restricted_to_unlimited_songs: bool,
    pub unlock_objective: ObjectiveDesc1819<'a>,
}

impl UnlockableAliasDescriptor1719<'_> {
    pub const CLASS: &'static str = "JD_UnlockableAliasDescriptor";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ScheduledQuestDatabase1819<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub scheduled_quests: Vec<ScheduledQuestDesc1819<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ScheduledQuestDesc1819<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    #[serde(rename = "ID")]
    pub id: u32,
    #[serde(rename = "Type")]
    pub type_it: u8,
    pub unlimited_only: bool,
    pub mojo_reward: u32,
    pub objective: ObjectiveDesc1819<'a>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preconditions_objectives_id: Vec<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Cow<'a, str>>,
}

impl ScheduledQuestDesc1819<'_> {
    pub const CLASS: &'static str = "JD_ScheduledQuestDesc";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum ObjectiveDesc1819<'a> {
    #[serde(borrow, rename = "JD_ObjectiveDesc")]
    Base(ObjectiveDesc1819Base<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_MinStarsReachedSongCount")]
    MinStarsReachedSongCount(ObjectiveDesc1819MinStarsReachedSongCount<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_SweatSongCount")]
    SweatSongCount(ObjectiveDesc1819Base<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_WDFSongCount")]
    WDFSongCount(ObjectiveDesc1819Base<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_GatherStarsWDF")]
    GatherStarsWDF(ObjectiveDesc1819Base<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_PlaySpecificMap")]
    PlaySpecificMap(ObjectiveDesc1819PlaySpecificMap<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_RecommendedSongCount")]
    RecommendSongCount(ObjectiveDesc1819Base<'a>),
    #[serde(borrow, rename = "JD_ObjectiveDesc_ClassicTournamentRank")]
    ClassicTournamentRank(ObjectiveDesc1819Base<'a>),
}

impl ObjectiveDesc1819<'_> {
    #[must_use]
    pub const fn probability_weight(&self) -> u32 {
        match self {
            Self::Base(data)
            | Self::SweatSongCount(data)
            | Self::RecommendSongCount(data)
            | Self::WDFSongCount(data)
            | Self::GatherStarsWDF(data)
            | Self::ClassicTournamentRank(data) => data.probability_weight,
            Self::MinStarsReachedSongCount(data) => data.probability_weight,
            Self::PlaySpecificMap(data) => data.probability_weight,
        }
    }

    #[must_use]
    pub const fn description(&self) -> LocaleId {
        match self {
            Self::Base(data)
            | Self::SweatSongCount(data)
            | Self::RecommendSongCount(data)
            | Self::WDFSongCount(data)
            | Self::ClassicTournamentRank(data)
            | Self::GatherStarsWDF(data) => data.description,
            Self::MinStarsReachedSongCount(data) => data.description,
            Self::PlaySpecificMap(data) => data.description,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDesc1819Base<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDesc1819MinStarsReachedSongCount<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
    pub min_star_to_reach: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ObjectiveDesc1819PlaySpecificMap<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub description: LocaleId,
    pub objective_type: u8,
    pub minimum_value: u32,
    pub probability_weight: u32,
    pub map_name: Cow<'a, str>,
}
