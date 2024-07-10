use std::borrow::Cow;

use serde::{Deserialize, Serialize};

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
