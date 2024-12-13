//! # Quests
//! Describes the daily quests
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{AtomicU32, Ordering},
};

use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::cooked;

use super::objectives::{Objective, Objectives};
use crate::types::localisation::LocaleIdMap;

/// Configuration for the daily quests
#[derive(Debug, Clone, Serialize, Deserialize, IntoOwned)]
pub struct ScheduledQuests<'a> {
    /// Unknown
    pub minimum_score: u32,
    /// Unknown
    pub session_count_until_discovery_kill: u32,
    /// Unknown
    pub session_count_until_quest_kill: u32,
    /// Unknown
    pub session_count_until_first_discovery_kill: u32,
    /// Unknown
    pub session_count_until_normal_quest_setting: u32,
    /// First quest for users without a save file
    #[serde(borrow)]
    pub first_discovery_quest: QuestDescription<'a>,
    /// Unknown
    pub push_song_probability: u32,
    /// Unknown
    pub update_timings: HashMap<u32, f32>,
    /// How long until there are new quests
    pub time_cap_in_hours_to_renew: u32,
    /// Unknown
    #[serde(borrow)]
    pub exclude_from_algorithm_quest_tags: Vec<HipStr<'a>>,
    /// The quests
    #[serde(borrow)]
    pub quests: HashSet<QuestDescription<'a>>,
}

/// Describes a quest
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, IntoOwned)]
pub struct QuestDescription<'a> {
    /// Quest type, values unknown
    pub quest_type: u8,
    /// Reward for completing the quest
    pub mojo_reward: u32,
    /// Probability it will be served as a daily quest
    pub probability_weight: u32,
    /// The objective of the quest
    #[serde(borrow)]
    pub objective: HipStr<'a>,
    /// Is it only possible with JD Unlimited
    pub unlimited_only: bool,
    /// Tags for the quest?
    #[serde(borrow)]
    pub tags: Vec<HipStr<'a>>,
    /// Conditions that need to be completed before the quest is shown
    #[serde(borrow)]
    pub preconditions: Vec<HipStr<'a>>,
}

/// Contains the last id used for quests
static QUEST_ID: AtomicU32 = AtomicU32::new(1);

/// Generate a new id for a quest
///
/// # Panics
/// Will panic if if incrementing the id would overflow
fn generate_quest_id() -> u32 {
    // SAFETY: The atomic u16 will make sure every call gets a different value
    let id = QUEST_ID.fetch_add(1, Ordering::SeqCst);
    assert_ne!(id, u32::MAX, "Ran out of IDs for quests!");
    id
}

impl<'a> From<cooked::isg::ScheduledQuestDesc<'a>> for QuestDescription<'a> {
    fn from(value: cooked::isg::ScheduledQuestDesc<'a>) -> Self {
        Self {
            quest_type: value.type_it,
            mojo_reward: value.mojo_reward,
            probability_weight: value.probability_weight,
            unlimited_only: value.unlimited_only,
            objective: value.objective_id,
            tags: value.tags,
            preconditions: value.preconditions_objectives_id,
        }
    }
}

impl<'a> From<QuestDescription<'a>> for cooked::isg::ScheduledQuestDesc<'a> {
    fn from(value: QuestDescription<'a>) -> Self {
        cooked::isg::ScheduledQuestDesc {
            class: Some(cooked::isg::ScheduledQuestDesc::CLASS),
            id: generate_quest_id(),
            type_it: value.quest_type,
            unlimited_only: value.unlimited_only,
            mojo_reward: value.mojo_reward,
            probability_weight: value.probability_weight,
            objective_id: value.objective,
            preconditions_objectives_id: value.preconditions,
            tags: value.tags,
        }
    }
}

impl<'a> QuestDescription<'a> {
    /// Convert an old quest description into the modern format
    pub fn from_scheduled_quest_desc_1819(
        description: cooked::isg::ScheduledQuestDesc1819<'a>,
        objectives: &mut Objectives<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Self {
        let objective = HipStr::from(objectives.add_objective(Objective::from_old_descriptor(
            &description.objective,
            description.unlimited_only,
            locale_id_map,
        )));

        Self {
            quest_type: description.type_it,
            mojo_reward: description.mojo_reward,
            probability_weight: description.objective.probability_weight(),
            unlimited_only: description.unlimited_only,
            objective,
            tags: description.tags,
            preconditions: description.preconditions_objectives_id,
        }
    }
}
