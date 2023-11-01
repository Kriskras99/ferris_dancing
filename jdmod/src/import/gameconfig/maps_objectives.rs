//! # Map objectives
//! Import all map objectives
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use ubiart_toolkit::json_types::MapsObjectives;

use crate::types::ImportState;

/// Import all map objectives
pub fn import_v20v22(
    is: &ImportState<'_>,
    new_mapsobjectives: MapsObjectives,
) -> Result<(), Error> {
    println!("Importing maps objectives...");

    let maps_objectives_path = is.dirs.config().join("maps_objectives.json");
    let mut maps_objectives: MapsObjectives = if let Ok(file) = File::open(&maps_objectives_path) {
        serde_json::from_reader(file)?
    } else {
        HashMap::new()
    };

    for (name, objective) in new_mapsobjectives {
        maps_objectives.entry(name).or_insert(objective);
    }

    let maps_objectives_file = File::create(maps_objectives_path)?;
    serde_json::to_writer_pretty(maps_objectives_file, &maps_objectives)?;

    Ok(())
}
