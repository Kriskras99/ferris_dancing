//! # Offline Recommendations
//! Build the offline recommendations
use std::{borrow::Cow, fs::File};

use anyhow::Error;
use ubiart_toolkit::json_types::v22::GameManagerConfig22;

use crate::build::BuildState;

/// Build the offline recommendations
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let offline_recommendation: Vec<Cow<'_, str>> = serde_json::from_reader(File::open(
        bs.dirs.config().join("offline_recommendations.json"),
    )?)?;

    gameconfig.offline_recommendation = offline_recommendation;

    Ok(())
}
