//! Contains the types that describe the usefull information in this filetype

use hipstr::HipStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSpaceMove<'a> {
    #[serde(borrow)]
    pub name: HipStr<'a>,
    #[serde(borrow)]
    pub map: HipStr<'a>,
    #[serde(borrow)]
    pub device: HipStr<'a>,
    pub data: Vec<(f32, f32)>,
    pub version: u32,
    pub unk3: f32,
    pub unk4: f32,
    pub unk5: f32,
    /// Only in version 7
    pub unk6: Option<f32>,
    /// Only in version 7
    pub unk7: Option<f32>,
    /// Does not appear in little endian format
    pub unk10: Option<u32>,
    pub unk11: u32,
    pub unk13: u32,
    pub unk14: f32,
    pub unk15: f32,
}
