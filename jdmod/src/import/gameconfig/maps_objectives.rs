//! # Map objectives
//! Import all map objectives
use std::fs::File;

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
    let maps_objectives_file =
        std::fs::read(&maps_objectives_path).unwrap_or_else(|_| vec![b'{', b'}']);
    let mut maps_objectives: MapsObjectives = serde_json::from_slice(&maps_objectives_file)?;

    for (name, objective) in new_mapsobjectives {
        maps_objectives.entry(name).or_insert(objective);
    }

    let maps_objectives_file = File::create(maps_objectives_path)?;
    serde_json::to_writer_pretty(maps_objectives_file, &maps_objectives)?;

    Ok(())
}
