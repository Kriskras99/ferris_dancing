//! # Scheduled Quests Building
//! Build the scheduled quests
use std::fs::File;

use anyhow::Error;
use ubiart_toolkit::{
    cooked,
    json_types::{self, v22::GameManagerConfig22},
};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::scheduled_quests::ScheduledQuests,
    utils::cook_path,
};

/// Build the scheduled quests
pub fn build(
    bs: &BuildState,
    bf: &mut BuildFiles,
    gameconfig: &mut GameManagerConfig22<'_>,
) -> Result<(), Error> {
    let quest_config: ScheduledQuests =
        serde_json::from_reader(File::open(bs.dirs.config().join("quests.json"))?)?;

    let mut scheduled_quests = Vec::new();

    for quest in quest_config.quests {
        scheduled_quests.push(quest.into());
    }

    let discovery_quest = json_types::ScheduledQuestDesc::from(quest_config.first_discovery_quest);
    let first_discovery_quest_id = discovery_quest.id;
    scheduled_quests.push(discovery_quest);

    let setup = &mut gameconfig.scheduled_quest_setup;
    setup.minimum_score = quest_config.minimum_score;
    setup.session_count_until_discovery_kill = quest_config.session_count_until_discovery_kill;
    setup.session_count_until_first_discovery_kill =
        quest_config.session_count_until_first_discovery_kill;
    setup.session_count_until_normal_quest_setting =
        quest_config.session_count_until_normal_quest_setting;
    setup.session_count_until_quest_kill = quest_config.session_count_until_quest_kill;
    setup.push_song_probability = quest_config.push_song_probability;
    setup.update_timings = quest_config.update_timings;
    setup.time_cap_in_hours_to_renew = quest_config.time_cap_in_hours_to_renew;
    setup.exclude_from_algorithm_quest_tags = quest_config.exclude_from_algorithm_quest_tags;
    setup.first_discovery_quest_id = first_discovery_quest_id;

    let quest_database =
        json_types::v22::Template22::ScheduledQuestDatabase(json_types::ScheduledQuestDatabase {
            class: None,
            scheduled_quests,
        });

    let quest_database_vec = cooked::json::create_vec(&quest_database)?;
    bf.generated_files.add_file(
        cook_path("enginedata/gameconfig/scheduledquests.isg", bs.platform)?,
        quest_database_vec,
    );

    Ok(())
}
