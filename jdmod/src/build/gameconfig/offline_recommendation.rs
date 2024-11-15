//! # Offline Recommendations
//! Build the offline recommendations

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use ownable::traits::IntoOwned;
use ubiart_toolkit::cooked::isg::GameManagerConfigV22;

use crate::build::BuildState;

/// Build the offline recommendations
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfigV22) -> Result<(), Error> {
    let offline_recommendation_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("offline_recommendations.json"))?;
    let offline_recommendation =
        serde_json::from_slice::<Vec<HipStr<'_>>>(&offline_recommendation_file)?.into_owned();

    gameconfig.offline_recommendation = offline_recommendation;

    Ok(())
}
