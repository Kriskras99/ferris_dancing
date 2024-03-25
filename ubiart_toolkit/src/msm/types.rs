//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementSpaceMove<'a> {
    pub name: Cow<'a, str>,
    pub map: Cow<'a, str>,
    pub device: Cow<'a, str>,
    pub data: Vec<(u32, u32)>,
    pub points: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk10: u32,
    pub unk14: u32,
    pub unk15: u32,
}
