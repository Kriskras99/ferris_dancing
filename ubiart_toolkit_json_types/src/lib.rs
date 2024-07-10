#![allow(
    clippy::struct_excessive_bools,
    reason = "The booleans are imposed by the UbiArt engine"
)]

#[cfg(feature = "full_json_types")]
pub mod frt;
pub mod isg;
pub mod just_dance;
#[cfg(feature = "full_json_types")]
pub mod msh;
pub mod tape;
#[cfg(feature = "full_json_types")]
pub mod tfn;
pub mod tpl;
pub mod v1819;

pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v20c;
pub mod v21;
pub mod v22;

use std::{borrow::Cow, collections::HashMap};

use isg::Rarity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Empty<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
}

pub type AliasesObjectives<'a> = HashMap<u16, Cow<'a, str>>;
pub type DifficultyColors<'a> = HashMap<Rarity, Cow<'a, str>>;
pub type MapsGoals<'a> = HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>;
pub type MapsObjectives<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;
pub type OfflineRecommendation<'a> = Vec<Cow<'a, str>>;
pub type AvatarsObjectives<'a> = HashMap<u16, Cow<'a, str>>;
pub type PhoneImages<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;
