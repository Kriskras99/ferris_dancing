//! # Search labels building
//! Build the search labels
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::File,
};

use anyhow::Error;
use ubiart_toolkit::json_types::{self, v22::GameManagerConfig22};

use crate::{build::BuildState, types::gameconfig::search_labels::SearchLabel};

/// Build the search labels
pub fn build(bs: &BuildState, gameconfig: &mut GameManagerConfig22<'_>) -> Result<(), Error> {
    let search_labels: HashMap<Cow<'_, str>, HashSet<SearchLabel>> =
        serde_json::from_reader(File::open(bs.rel_tree.config().join("search_labels.json"))?)?;

    gameconfig.search_labels.maps = search_labels
        .into_iter()
        .map(|(name, tags)| {
            (
                name,
                json_types::isg::SongSearchTags {
                    class: Some(json_types::isg::SongSearchTags::CLASS),
                    tags: tags
                        .into_iter()
                        .map(json_types::isg::SongSearchTag::from)
                        .collect(),
                },
            )
        })
        .collect();

    Ok(())
}
