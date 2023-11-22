mod isg;
mod just_dance;
#[cfg(feature = "full_json_types")]
mod msh;
mod tape;
mod tpl;
mod v1719;

pub mod v17;
pub mod v18;
pub mod v19;
pub mod v20;
pub mod v20c;
pub mod v21;
pub mod v22;

use std::{borrow::Cow, collections::HashMap};

use anyhow::{anyhow, Error};
pub use isg::*;
pub use just_dance::*;
#[cfg(feature = "full_json_types")]
pub use msh::*;
use stable_deref_trait::StableDeref;
pub use tape::*;
pub use tpl::*;
pub use v1719::*;

use serde::{Deserialize, Serialize};
use yoke::{Yoke, Yokeable};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(deny_unknown_fields)]
pub struct Empty<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
}

pub type AliasesObjectives<'a> = HashMap<u16, Cow<'a, str>>;
pub type DifficultyColors<'a> = HashMap<u8, Cow<'a, str>>;
pub type MapsGoals<'a> = HashMap<Cow<'a, str>, Vec<Cow<'a, str>>>;
pub type MapsObjectives<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;
pub type OfflineRecommendation<'a> = Vec<Cow<'a, str>>;
pub type AvatarsObjectives<'a> = HashMap<u16, Cow<'a, str>>;
pub type PhoneImages<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFXManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub bus_list: Vec<Buses<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buses<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    class: Option<&'a str>,
    pub actor_type: Cow<'a, str>,
    pub bus: Cow<'a, str>,
}
