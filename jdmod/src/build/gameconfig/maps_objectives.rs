//! # Maps Objective Building
//! Build the maps objective
use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::json_types::{v22::GameManagerConfig22, MapsObjectives};

use crate::build::BuildState;

/// Build the maps objective
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let maps_objectives_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("maps_objectives.json"))?;
    let maps_objectives: MapsObjectives = serde_json::from_slice(&maps_objectives_file)?;

    gameconfig.mapsobjectives = maps_objectives;

    Ok(())
}
