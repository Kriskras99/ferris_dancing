use hipstr::HipStr;
use serde::{Deserialize, Serialize};

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
pub struct FontTemplate<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub info: Info<'a>,
    #[serde(borrow)]
    pub common: Common<'a>,
    #[serde(borrow)]
    pub pages: Vec<Page<'a>>,
    #[serde(borrow)]
    pub chars: Vec<Char<'a>>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    #[serde(borrow)]
    pub face: HipStr<'a>,
    pub size: u32,
    pub bold: u32,
    pub italic: u32,
    #[serde(borrow)]
    pub charset: HipStr<'a>,
    pub unicode: u32,
    pub stretch_h: u32,
    pub smooth: u32,
    pub aa: u32,
    pub padding_left: u32,
    pub padding_right: u32,
    pub padding_top: u32,
    pub padding_bottom: u32,
    pub spacing_left: u32,
    pub spacing_top: u32,
    pub outline: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Common<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub line_height: u32,
    pub base: u32,
    pub scale_w: u32,
    pub scale_h: u32,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    #[serde(borrow)]
    pub file: HipStr<'a>,
}

#[cfg(feature = "full_json_types")]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Char<'a> {
    #[serde(
        borrow,
        rename = "__class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub class: Option<HipStr<'a>>,
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: u32,
    pub page: u32,
    pub chnl: u32,
}
