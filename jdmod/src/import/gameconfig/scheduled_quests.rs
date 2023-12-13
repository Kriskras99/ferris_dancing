//! # Scheduled Quests
//! Imports all scheduled quests
use std::fs::File;

use anyhow::{anyhow, Context, Error};
use ubiart_toolkit::{
    cooked,
    json_types::{ScheduledQuestDesc1819, ScheduledQuestSetup},
};

use super::objectives::{load_objectives, save_objectives};
use crate::{
    types::{
        gameconfig::scheduled_quests::{QuestDescription, ScheduledQuests},
        ImportState,
    },
    utils::cook_path,
};

/// Import scheduled quests for Just Dance 2020-2022
pub fn import_v20v22(
    is: &ImportState<'_>,
    setup: ScheduledQuestSetup<'_>,
    path: &str,
) -> Result<(), Error> {
    println!("Importing scheduled quests...");

    let scheduled_quests_file = is.vfs.open(cook_path(path, is.platform)?.as_ref())?;
    let parsed_json = cooked::json::parse_v22(&scheduled_quests_file, is.lax)?;
    let scheduled_quests = parsed_json.scheduled_quests_database()?;
    let quest_descriptions = scheduled_quests.scheduled_quests;

    let quest_config_path = is.dirs.config().join("quests.json");
    let scheduled_quests = if quest_config_path.exists() {
        let quest_config_file = File::open(&quest_config_path)?;
        let mut scheduled_quests: ScheduledQuests<'_> =
            serde_json::from_reader(&quest_config_file)?;
        scheduled_quests
            .quests
            .extend(quest_descriptions.into_iter().map(QuestDescription::from));
        scheduled_quests
    } else {
        let first_discovery_quest = quest_descriptions
            .iter()
            .find(|q| q.id == setup.first_discovery_quest_id)
            .map(|q| QuestDescription::from(q.clone()))
            .ok_or_else(|| anyhow!("Could not find quest matching discovery quest id!"))?;
        let quests = quest_descriptions
            .into_iter()
            .map(QuestDescription::from)
            .collect();
        ScheduledQuests {
            minimum_score: setup.minimum_score,
            session_count_until_discovery_kill: setup.session_count_until_discovery_kill,
            session_count_until_quest_kill: setup.session_count_until_quest_kill,
            session_count_until_first_discovery_kill: setup
                .session_count_until_first_discovery_kill,
            session_count_until_normal_quest_setting: setup
                .session_count_until_normal_quest_setting,
            first_discovery_quest,
            push_song_probability: setup.push_song_probability,
            update_timings: setup.update_timings,
            time_cap_in_hours_to_renew: setup.time_cap_in_hours_to_renew,
            exclude_from_algorithm_quest_tags: setup.exclude_from_algorithm_quest_tags,
            quests,
        }
    };

    let quest_config_file = File::create(&quest_config_path)?;
    serde_json::to_writer_pretty(quest_config_file, &scheduled_quests)?;

    Ok(())
}

/// Import scheduled quests for Just Dance 2018-2019
pub fn import_v18v19(
    is: &ImportState<'_>,
    quest_descriptions: Vec<ScheduledQuestDesc1819<'_>>,
) -> Result<(), Error> {
    println!("Importing scheduled quests...");

    let mut objectives = load_objectives(is)?;

    let quest_config_path = is.dirs.config().join("quests.json");
    let mut scheduled_quests: ScheduledQuests<'_> = {
        let quest_config_file =
            File::open(&quest_config_path).context("config/quests.json not found!")?;
        serde_json::from_reader(&quest_config_file)?
    };
    let quests = quest_descriptions
        .into_iter()
        .map(|q| {
            QuestDescription::from_scheduled_quest_desc_1819(q, &mut objectives, &is.locale_id_map)
        })
        .collect::<Result<Vec<_>, _>>()?;
    scheduled_quests.quests.extend(quests);

    save_objectives(is, &objectives)?;

    let quest_config_file = File::create(&quest_config_path)?;
    serde_json::to_writer_pretty(quest_config_file, &scheduled_quests)?;
    Ok(())
}
