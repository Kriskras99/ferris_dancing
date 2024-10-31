use hipstr::HipStr;
use serde::{Deserialize, Serialize};

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFXManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub bus_list: Vec<Buses<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Buses<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub actor_type: HipStr<'a>,
    #[serde(borrow)]
    pub bus: HipStr<'a>,
}
