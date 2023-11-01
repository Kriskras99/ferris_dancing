//! Map Goals Building
//! Build map goals
use std::fs::File;

use anyhow::Error;
use ubiart_toolkit::json_types::{v22::GameManagerConfig22, MapsGoals};

use crate::build::BuildState;

/// Build map goals
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let maps_goals: MapsGoals =
        serde_json::from_reader(File::open(bs.dirs.config().join("maps_goals.json"))?)?;

    gameconfig.maps_goals = maps_goals;

    Ok(())
}
