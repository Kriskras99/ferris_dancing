//! # Objectives
//! Import all objectives.
//!
//! For Just Dance 2018-2019 this is done in Scheduled Quests and other places where quests are used.
//! This is because the objectives are part of the quest in these games.
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use ubiart_toolkit::cooked;

use crate::{
    types::{
        gameconfig::objectives::{Objective, Objectives},
        ImportState,
    },
    utils::cook_path,
};

/// Import all objectives.
pub fn import_v20v22(is: &ImportState<'_>, objectives_path: &str) -> Result<(), Error> {
    let mut objectives = load_objectives(is)?;

    let objectives_file = is
        .vfs
        .open(cook_path(objectives_path, is.platform)?.as_ref())?;
    let parsed_json = cooked::json::parse_v22(&objectives_file)?;
    let objective_database = parsed_json.objectives_database()?;
    for (name, descriptor) in &objective_database.objective_descs {
        objectives.add_objective_with_name(
            Objective::from_descriptor(descriptor, &is.locale_id_map)?,
            name.to_string(),
        )?;
    }

    save_objectives(is, &objectives)?;

    Ok(())
}

/// Load existing objectives in the mod
pub fn load_objectives(is: &ImportState<'_>) -> Result<Objectives<'static>, Error> {
    if let Ok(file) = File::open(is.dirs.config().join("objectives.json")) {
        let name_map: HashMap<String, Objective> = serde_json::from_reader(file)?;
        let mut objective_map = HashMap::with_capacity(name_map.len());
        for (name, objective) in &name_map {
            objective_map.insert(objective.clone(), name.clone());
        }
        Ok(Objectives {
            objective_map,
            name_map,
        })
    } else {
        Ok(Objectives::default())
    }
}

/// Save the objectives to the mod folder
pub fn save_objectives(is: &ImportState<'_>, objectives: &Objectives) -> Result<(), Error> {
    let file = File::create(is.dirs.config().join("objectives.json"))?;
    serde_json::to_writer_pretty(file, &objectives.name_map)?;

    Ok(())
}
