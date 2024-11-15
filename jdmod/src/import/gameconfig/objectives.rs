//! # Objectives
//! Import all objectives.
//!
//! For Just Dance 2018-2019 this is done in Scheduled Quests and other places where quests are used.
//! This is because the objectives are part of the quest in these games.
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use hipstr::HipStr;
use ownable::traits::IntoOwned;
use ubiart_toolkit::{cooked, cooked::isg::ObjectivesDatabase};

use crate::{
    types::{
        gameconfig::objectives::{Objective, Objectives},
        ImportState,
    },
    utils::cook_path,
};

/// Import all objectives.
pub fn import_v20v22(is: &ImportState<'_>, objectives_path: &str) -> Result<(), Error> {
    let objectives_file = is.vfs.open(cook_path(objectives_path, is.ugi)?.as_ref())?;
    let objective_database = cooked::isg::parse::<ObjectivesDatabase>(&objectives_file, is.lax)?;

    let mut objectives = load_objectives(is)?;

    for (name, descriptor) in &objective_database.objective_descs {
        objectives.add_objective_with_name(
            Objective::from_descriptor(descriptor, &is.locale_id_map)?,
            name.clone(),
        )?;
    }

    save_objectives(is, &objectives)?;

    Ok(())
}

/// Load existing objectives in the mod
pub fn load_objectives(is: &ImportState<'_>) -> Result<Objectives<'static>, Error> {
    if let Ok(file) = std::fs::read(is.dirs.config().join("objectives.json")) {
        let name_map = serde_json::from_slice::<HashMap<HipStr, Objective>>(&file)?.into_owned();
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
pub fn save_objectives(is: &ImportState<'_>, objectives: &Objectives) -> std::io::Result<()> {
    let file = File::create(is.dirs.config().join("objectives.json"))?;
    serde_json::to_writer_pretty(file, &objectives.name_map)?;

    Ok(())
}
