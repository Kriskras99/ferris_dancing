//! # Maps Objective Building
//! Build the maps objective
use std::fs::File;

use anyhow::Error;
use ubiart_toolkit::json_types::{v22::GameManagerConfig22, MapsObjectives};

use crate::build::BuildState;

/// Build the maps objective
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let maps_objectives: MapsObjectives = serde_json::from_reader(File::open(
        bs.rel_tree.config().join("maps_objectives.json"),
    )?)?;

    gameconfig.mapsobjectives = maps_objectives;

    Ok(())
}
