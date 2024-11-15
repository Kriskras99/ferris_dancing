#![allow(
    clippy::struct_excessive_bools,
    reason = "Format is dictated by the engine"
)]
mod writer;

use std::collections::HashMap;

use anyhow::{anyhow, Error};
use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use superstruct::superstruct;
use ubiart_toolkit_shared_types::{errors::ParserError, LocaleId};
pub use writer::*;

use crate::shared_json_types::{Empty, ObjectiveDesc};
pub use crate::utils::json::parse;

pub fn parse_json(data: &[u8]) -> Result<Json<'_>, ParserError> {
    let res = crate::utils::json::parse(data, false)?;
    Ok(res)
}

pub type DifficultyColors<'a> = HashMap<Rarity, HipStr<'a>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "__class")]
pub enum Json<'a> {
    #[serde(borrow, rename = "JD_CarouselRules")]
    CarouselRules(CarouselRules<'a>),
    #[serde(borrow, rename = "JD_LocalAliases")]
    LocalAliases(LocalAliases<'a>),
    #[serde(borrow, rename = "JD_PlaylistDatabase_Template")]
    PlaylistDatabase(PlaylistDatabase<'a>),
    #[serde(borrow, rename = "JD_WDFLinearRewards")]
    WDFLinearRewards(WDFLinearRewards<'a>),
}

impl<'a> Json<'a> {
    pub fn into_playlist_database(self) -> Result<PlaylistDatabase<'a>, Error> {
        if let Self::PlaylistDatabase(db) = self {
            Ok(db)
        } else {
            Err(anyhow!("No PlaylistDatabase in Json"))
        }
    }

    pub fn into_local_aliases(self) -> Result<LocalAliases<'a>, Error> {
        if let Self::LocalAliases(db) = self {
            Ok(db)
        } else {
            Err(anyhow!("No LocalAliases in Json"))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRules<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_lists: HashMap<HipStr<'a>, ActionList<'a>>,
    // Not on WiiU
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub song_item_lists: Option<HashMap<HipStr<'a>, SongItemList<'a>>>,
    #[serde(borrow)]
    pub rules: HashMap<HipStr<'a>, CarouselRule<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ActionList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub item_type: HipStr<'a>,
    #[serde(borrow)]
    pub actions: Vec<Action<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Action<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub title_id: u32,
    pub online_title_id: u32,
    #[serde(borrow)]
    pub target: HipStr<'a>,
    #[serde(borrow)]
    pub banner_type: HipStr<'a>,
    #[serde(borrow)]
    pub banner_theme: HipStr<'a>,
    #[serde(borrow)]
    pub banner_context: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItemList<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(borrow)]
    pub list: Vec<SongItem<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SongItem<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub map_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselRule<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub categories: Vec<CategoryRule<'a>>,
    pub online_only: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CategoryRule<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub title: HipStr<'a>,
    pub title_id: u32,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub requests: Vec<CarouselRequestDesc<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

impl CategoryRule<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("CategoryRule");
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselRequestDesc<'a> {
    #[serde(borrow, rename = "JD_CarouselCustomizableItemRequestDesc")]
    CustomizableItem(CarouselCustomizableItemRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselUgcRequestDesc")]
    Ugc(CarouselUgcRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselMapRequestDesc")]
    Map(CarouselMapRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselDancerCardRequestDesc")]
    DancerCard(CarouselDancerCardRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselFriendRequestDesc")]
    Friend(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselItemRequestDesc")]
    Item(CarouselItemRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselPlaylistsRequestDesc")]
    Playlists(CarouselPlaylistsRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselComVideoRequestDesc")]
    ComVideo(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselLatestChallengesRequestDesc")]
    LatestChallenges(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselFriendChallengesRequestDesc")]
    FriendChallenges(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselPhotoRequestDesc")]
    PhotoRequest(CarouselPhotoRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselQuestRequestDesc")]
    QuestRequest(CarouselQuestRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselMapSearchRequestDesc")]
    MapSearch(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselSearchRequestDesc")]
    Search(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselGalaxyRequestDesc")]
    Galaxy(CarouselGalaxyRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselPlaylistRequestDesc")]
    Playlist(CarouselPlaylistRequestDesc<'a>),
    #[serde(borrow, rename = "JD_CarouselDMEpisodeRequestDesc")]
    DMEpisode(Empty<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselGalaxyRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub sub_type: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselCustomizableItemRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub jdversion: i32,
    pub lock_type: u32,
    pub sort_by: u32,
    pub item_type: u32,
    pub unlocked_only: bool,
    pub first_time_creation: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselUgcRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub uploading: bool,
    pub offline: bool,
    pub most_liked: bool,
    pub most_viewed: bool,
    pub featured: bool,
    pub query_pid: bool,
    pub player_pid: bool,
    pub friend_pids: bool,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselMapRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    #[serde(rename = "originalJDVersion")]
    pub original_jd_version: u32,
    pub coach_count: u32,
    #[serde(borrow)]
    pub order: HipStr<'a>,
    pub subscribed: bool,
    /// Not in 2016
    #[serde(default)]
    pub favorites: bool,
    /// Only included in nx2019
    pub sweat_toggle_item: Option<bool>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub included_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<CarouselFilter<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub custom_tags: Vec<HipStr<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub optional_tags: Vec<HipStr<'a>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselDancerCardRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub main: bool,
    pub create: bool,
    // Only on WiiU
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_save_item: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselItemRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    // Not on WiiU
    #[serde(borrow, default)]
    pub item_list: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPlaylistsRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub isc: HipStr<'a>,
    #[serde(borrow)]
    pub act: HipStr<'a>,
    #[serde(borrow, rename = "type")]
    pub type_it: HipStr<'a>,
    #[serde(borrow, rename = "playlistID")]
    pub playlist_id: HipStr<'a>,
}

impl Default for CarouselPlaylistsRequestDesc<'static> {
    fn default() -> Self {
        Self {
            class: Option::default(),
            isc: HipStr::borrowed("grp_row"),
            act: HipStr::borrowed("ui_carousel"),
            type_it: HipStr::borrowed("edito-pinned"),
            playlist_id: HipStr::borrowed(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPhotoRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselPlaylistRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub action_list_name: HipStr<'a>,
    pub create: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselQuestRequestDesc<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub start_action: HipStr<'a>,
    pub offline: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, tag = "__class")]
pub enum CarouselFilter<'a> {
    #[serde(borrow, rename = "JD_CarouselFilter")]
    Empty(Empty<'a>),
    #[serde(borrow, rename = "JD_CarouselSubscriptionFilter")]
    Subscription(CarouselSubscriptionFilter<'a>),
    #[serde(borrow, rename = "JD_CarouselSkuFilter")]
    Sku(CarouselSkuFilter<'a>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselSubscriptionFilter<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub subscribed: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CarouselSkuFilter<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub game_version: HipStr<'a>,
    #[serde(borrow)]
    pub platform: HipStr<'a>,
}

#[superstruct(
    variants(V2022, V19),
    variant_attributes(
        serde_as,
        derive(Debug, Serialize, Deserialize, Clone),
        serde(deny_unknown_fields, rename_all = "camelCase")
    ),
    enum_variant_attributes(serde(borrow))
)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub struct LocalAliases<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub locked_color: HipStr<'a>,
    #[serde(borrow)]
    pub difficulty_colors: DifficultyColors<'a>,
    #[superstruct(only(V2022), no_getter)]
    #[serde(borrow)]
    pub aliases: Vec<UnlockableAliasDescriptor2022<'a>>,
    #[superstruct(only(V19), no_getter)]
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    #[serde(borrow)]
    pub aliases: HashMap<u16, UnlockableAliasDescriptor19<'a>>,
}

impl LocalAliasesV2022<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_LocalAliases");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct UnlockableAliasDescriptor2022<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    #[serde(rename = "StringLocID")]
    pub string_loc_id: LocaleId,
    #[serde(rename = "StringLocIDFemale")]
    pub string_loc_id_female: LocaleId,
    #[serde(borrow)]
    pub string_online_localized: HipStr<'a>,
    #[serde(borrow)]
    pub string_online_localized_female: HipStr<'a>,
    #[serde(borrow)]
    pub string_placeholder: HipStr<'a>,
    pub unlocked_by_default: bool,
    #[serde(rename = "DescriptionLocID")]
    pub description_loc_id: LocaleId,
    #[serde(borrow)]
    pub description_localized: HipStr<'a>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub unlock_objective: Option<UnlockObjectiveOnlineInfo<'a>>,
    pub difficulty_color: Rarity,
    pub visibility: u32,
}

impl UnlockableAliasDescriptor2022<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_UnlockableAliasDescriptor");
}

impl Default for UnlockableAliasDescriptor2022<'static> {
    fn default() -> Self {
        Self {
            class: Some(UnlockableAliasDescriptor2022::CLASS),
            id: Default::default(),
            string_loc_id: LocaleId::default(),
            string_loc_id_female: LocaleId::default(),
            string_online_localized: HipStr::default(),
            string_online_localized_female: HipStr::default(),
            string_placeholder: HipStr::default(),
            unlocked_by_default: Default::default(),
            description_loc_id: LocaleId::default(),
            description_localized: HipStr::default(),
            unlock_objective: Some(UnlockObjectiveOnlineInfo::default()),
            difficulty_color: Rarity::Common,
            visibility: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct UnlockableAliasDescriptor19<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "StringLocID")]
    pub string_loc_id: LocaleId,
    #[serde(borrow)]
    pub string_online_localized: HipStr<'a>,
    #[serde(borrow)]
    pub string_placeholder: HipStr<'a>,
    pub difficulty_color: Rarity,
    pub restricted_to_unlimited_songs: bool,
    #[serde(borrow)]
    pub unlock_objective: ObjectiveDesc<'a>,
}

impl UnlockableAliasDescriptor19<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_UnlockableAliasDescriptor");
}

/// How rare is the alias
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Rarity {
    /// Common
    Common = 0,
    /// Uncommon
    Uncommon = 1,
    /// Rare
    Rare = 2,
    /// Epic
    Epic = 3,
    /// Legendary
    Legendary = 4,
    /// Exotic
    Exotic = 5,
}

impl<'de> Deserialize<'de> for Rarity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RarityVisitor;

        impl serde::de::Visitor<'_> for RarityVisitor {
            type Value = Rarity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 5")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(Rarity::Common),
                    1 => Ok(Rarity::Uncommon),
                    2 => Ok(Rarity::Rare),
                    3 => Ok(Rarity::Epic),
                    4 => Ok(Rarity::Legendary),
                    5 => Ok(Rarity::Exotic),
                    _ => Err(E::custom(format!("Rarity is unknown: {v}"))),
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "0" => Ok(Rarity::Common),
                    "1" => Ok(Rarity::Uncommon),
                    "2" => Ok(Rarity::Rare),
                    "3" => Ok(Rarity::Epic),
                    "4" => Ok(Rarity::Legendary),
                    "5" => Ok(Rarity::Exotic),
                    _ => Err(E::custom(format!("Rarity is unknown: {v}"))),
                }
            }

            // Similar for other methods:
            //   - visit_i16
            //   - visit_u8
            //   - visit_u16
            //   - visit_u32
            //   - visit_u64
        }

        deserializer.deserialize_any(RarityVisitor)
    }
}

impl Serialize for Rarity {
    #![allow(
        clippy::as_conversions,
        reason = "Rarity is repr(u8) and thus this is safe"
    )]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", *self as u8))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UnlockObjectiveOnlineInfo<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub unlock_objective_desc_id: HipStr<'a>,
}

impl UnlockObjectiveOnlineInfo<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_UnlockObjectiveOnlineInfo");
}

impl Default for UnlockObjectiveOnlineInfo<'_> {
    fn default() -> Self {
        Self {
            class: Some(Self::CLASS),
            unlock_objective_desc_id: HipStr::borrowed(""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlaylistDatabase<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub playlists: HashMap<HipStr<'a>, OfflinePlaylist<'a>>,
}

impl PlaylistDatabase<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("JD_PlaylistDatabase_Template");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OfflinePlaylist<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub title_id: LocaleId,
    pub description_id: LocaleId,
    #[serde(borrow)]
    pub cover_path: HipStr<'a>,
    #[serde(borrow)]
    pub maps: Vec<HipStr<'a>>,
}

impl OfflinePlaylist<'_> {
    pub const CLASS: HipStr<'static> = HipStr::borrowed("OfflinePlaylist");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFLinearRewards<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub rewards: Vec<WDFReward<'a>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct WDFReward<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(rename = "type")]
    pub type_it: u32,
    pub id: u32,
}
