//! # Search labels building
//! Build the search labels
use std::collections::{HashMap, HashSet};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use ownable::traits::IntoOwned;
use ubiart_toolkit::{cooked, cooked::isg::GameManagerConfigV22};

use crate::{build::BuildState, types::gameconfig::search_labels::SearchLabel};

/// Build the search labels
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfigV22<'_>) -> Result<(), Error> {
    let search_labels_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("search_labels.json"))?;
    let search_labels =
        serde_json::from_slice::<HashMap<HipStr<'_>, HashSet<SearchLabel>>>(&search_labels_file)?
            .into_owned();

    gameconfig.search_labels.maps = search_labels
        .into_iter()
        .map(|(name, tags)| {
            (
                name,
                cooked::isg::SongSearchTags {
                    class: Some(cooked::isg::SongSearchTags::CLASS),
                    tags: tags
                        .into_iter()
                        .map(cooked::isg::SongSearchTag::from)
                        .collect(),
                },
            )
        })
        .collect();

    Ok(())
}
