//! # Search labels
//! Describes search labels
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::{cooked::isg::SongSearchTag, utils::LocaleId};

use crate::types::localisation::LocaleIdMap;

/// A search label
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, IntoOwned)]
pub struct SearchLabel<'a> {
    /// Description of this label
    pub description: LocaleId,
    /// The label itself
    #[serde(borrow)]
    pub label: HipStr<'a>,
}

impl<'a> SearchLabel<'a> {
    /// Convert the UbiArt representation to the mod representation
    #[must_use]
    pub fn from_song_search_tag(jd_tag: SongSearchTag<'a>, locale_id_map: &LocaleIdMap) -> Self {
        Self {
            description: locale_id_map.get(jd_tag.tag_loc_id).unwrap_or_default(),
            label: jd_tag.tag,
        }
    }
}

impl<'a> From<SearchLabel<'a>> for SongSearchTag<'a> {
    fn from(value: SearchLabel<'a>) -> Self {
        SongSearchTag {
            class: Some(SongSearchTag::CLASS),
            tag_loc_id: value.description,
            tag: value.label,
        }
    }
}
