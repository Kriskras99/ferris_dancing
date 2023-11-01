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

use self::{
    v17::{Template17, Template17Owned},
    v18::{Template18, Template18Owned},
    v19::{Template19, Template19Owned},
    v20::{Template20, Template20Owned},
    v20c::{Template20C, Template20COwned},
    v21::{Template21, Template21Owned},
    v22::{Template22, Template22Owned},
};

pub enum TemplateOwned<C: StableDeref> {
    V17(Template17Owned<C>),
    V18(Template18Owned<C>),
    V19(Template19Owned<C>),
    V20(Template20Owned<C>),
    V20C(Template20COwned<C>),
    V21(Template21Owned<C>),
    V22(Template22Owned<C>),
}

pub enum Template<'a> {
    V17(Template17<'a>),
    V18(Template18<'a>),
    V19(Template19<'a>),
    V20(Template20<'a>),
    V20C(Template20C<'a>),
    V21(Template21<'a>),
    V22(Template22<'a>),
}

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
