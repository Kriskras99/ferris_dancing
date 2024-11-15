use hipstr::HipStr;
use serde::{Deserialize, Serialize};
use ubiart_toolkit_shared_types::errors::ParserError;

pub fn parse(data: &[u8]) -> Result<FeedbackFXManager<'_>, ParserError> {
    let res = crate::utils::json::parse(data, false)?;
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackFXManager<'a> {
    #[serde(rename = "__class", default, skip_serializing_if = "Option::is_none")]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub bus_list: Vec<Buses<'a>>,
}

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
