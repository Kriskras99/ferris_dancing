//! # Map goals
//! Import all map goals
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use anyhow::Error;
use hipstr::HipStr;
use ubiart_toolkit::cooked::isg::MapsGoals;

use crate::types::ImportState;

/// Import all map goals
pub fn import_v20v22(is: &ImportState, new_maps_goals: MapsGoals) -> Result<(), Error> {
    println!("Importing maps goals...");

    let maps_goals_path = is.dirs.config().join("maps_goals.json");
    let maps_goals_file = std::fs::read(&maps_goals_path).unwrap_or_else(|_| vec![b'{', b'}']);
    let mut maps_goals: HashMap<HipStr, HashSet<HipStr>> =
        serde_json::from_slice(&maps_goals_file)?;

    for (name, goals) in new_maps_goals {
        maps_goals
            .entry(name)
            .or_default()
            .extend(goals.into_iter());
    }

    let maps_goals_file = File::create(maps_goals_path)?;
    serde_json::to_writer_pretty(maps_goals_file, &maps_goals)?;

    Ok(())
}
