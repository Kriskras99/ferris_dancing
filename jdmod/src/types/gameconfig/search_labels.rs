//! # Search labels
//! Describes search labels
use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use ubiart_toolkit::{json_types, utils::LocaleId};

use crate::types::localisation::LocaleIdMap;

/// A search label
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SearchLabel<'a> {
    /// Description of this label
    pub description: LocaleId,
    /// The label itself
    pub label: Cow<'a, str>,
}

impl<'a> SearchLabel<'a> {
    /// Convert the UbiArt representation to the mod representation
    pub fn from_song_search_tag(
        jd_tag: json_types::SongSearchTag<'a>,
        locale_id_map: &LocaleIdMap,
    ) -> Self {
        Self {
            description: locale_id_map.get(jd_tag.tag_loc_id).unwrap_or_default(),
            label: jd_tag.tag,
        }
    }
}

impl<'a> From<SearchLabel<'a>> for json_types::SongSearchTag<'a> {
    fn from(value: SearchLabel<'a>) -> Self {
        json_types::SongSearchTag {
            class: Some(json_types::SongSearchTag::CLASS),
            tag_loc_id: value.description,
            tag: value.label,
        }
    }
}
