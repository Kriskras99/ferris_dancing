//! Map Goals Building
//! Build map goals
use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::json_types::{v22::GameManagerConfig22, MapsGoals};

use crate::build::BuildState;

/// Build map goals
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let maps_goals_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("maps_goals.json"))?;
    let maps_goals: MapsGoals = serde_json::from_slice(&maps_goals_file)?;

    gameconfig.maps_goals = maps_goals;

    Ok(())
}
