//! # Search labels
//! Imports search labels for the maps
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fs::File,
};

use anyhow::Error;
use ubiart_toolkit::json_types::SongsSearchTags;

use crate::types::{gameconfig::search_labels::SearchLabel, ImportState};

/// Imports search labels for the maps
pub fn import_v19v22(
    is: &ImportState<'_>,
    new_search_labels: SongsSearchTags<'_>,
) -> Result<(), Error> {
    println!("Importing search labels...");

    let search_labels_path = is.dirs.config().join("search_labels.json");
    let mut search_labels: HashMap<Cow<'_, str>, HashSet<SearchLabel>> =
        if let Ok(file) = File::open(&search_labels_path) {
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

    for (name, tags) in new_search_labels.maps {
        let map = search_labels.entry(name).or_default();
        for tag in tags.tags {
            map.insert(SearchLabel::from_song_search_tag(tag, &is.locale_id_map));
        }
    }

    let search_labels_file = File::create(search_labels_path)?;
    serde_json::to_writer_pretty(search_labels_file, &search_labels)?;

    Ok(())
}
