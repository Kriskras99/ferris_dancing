use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::LocaleId;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct AvatarDescription1819<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<&'a str>,
    pub jd_version: u16,
    pub relative_song_name: Cow<'a, str>,
    #[serde(rename = "RelativeQuestID")]
    pub relative_quest_id: Cow<'a, str>,
    #[serde(rename = "RelativeWDFBossName")]
    pub relative_wdf_boss_name: Cow<'a, str>,
    #[serde(rename = "RelativeWDFTournamentName")]
    pub relative_wdf_tournament_name: Cow<'a, str>,
    #[serde(rename = "RelativeJDRank")]
    pub relative_jd_rank: Cow<'a, str>,
    pub relative_game_mode_name: Cow<'a, str>,
    pub sound_family: Cow<'a, str>,
    pub status: u8,
    pub unlock_type: u8,
    pub mojo_price: u16,
    pub wdf_level: u8,
    pub count_in_progression: u8,
    pub actor_path: Cow<'a, str>,
    pub phone_image: Cow<'a, str>,
    pub avatar_id: u16,
    #[serde(rename = "UsedAsCoach_MapName")]
    pub used_as_coach_map_name: Cow<'a, str>,
    #[serde(rename = "UsedAsCoach_CoachId")]
    pub used_as_coach_coach_id: u8,
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
