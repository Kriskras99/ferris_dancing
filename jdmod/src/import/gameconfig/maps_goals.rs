//! # Map goals
//! Import all map goals
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::File,
};

use anyhow::Error;
use ubiart_toolkit::json_types::MapsGoals;

use crate::types::ImportState;

/// Import all map goals
pub fn import_v20v22(is: &ImportState<'_>, new_maps_goals: MapsGoals) -> Result<(), Error> {
    println!("Importing maps goals...");

    let maps_goals_path = is.dirs.config().join("maps_goals.json");
    let mut maps_goals: HashMap<Cow<'_, str>, HashSet<Cow<'_, str>>> =
        if let Ok(file) = File::open(&maps_goals_path) {
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

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
