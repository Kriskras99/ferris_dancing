//! # Offline Recommendations
//! Build the offline recommendations
use std::borrow::Cow;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use ubiart_toolkit::json_types::v22::GameManagerConfig22;

use crate::build::BuildState;

/// Build the offline recommendations
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let offline_recommendation_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("offline_recommendations.json"))?;
    let offline_recommendation: Vec<Cow<'_, str>> =
        serde_json::from_slice(&offline_recommendation_file)?;

    gameconfig.offline_recommendation = offline_recommendation;

    Ok(())
}
