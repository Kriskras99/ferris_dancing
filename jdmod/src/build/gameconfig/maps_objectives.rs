//! # Maps Objective Building
//! Build the maps objective
use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ownable::traits::IntoOwned;
use ubiart_toolkit::cooked::isg::{GameManagerConfigV22, MapsObjectives};

use crate::build::BuildState;

/// Build the maps objective
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfigV22<'_>) -> Result<(), Error> {
    let maps_objectives_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("maps_objectives.json"))?;
    let maps_objectives =
        serde_json::from_slice::<MapsObjectives>(&maps_objectives_file)?.into_owned();

    gameconfig.mapsobjectives = maps_objectives;

    Ok(())
}
