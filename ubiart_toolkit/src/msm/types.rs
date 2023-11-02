//! Contains the types that describe the usefull information in this filetype

use serde::{Deserialize, Serialize};
use stable_deref_trait::StableDeref;
use yoke::{Yoke, Yokeable};

pub struct MovementSpaceMoveOwned<C: StableDeref> {
    yoke: Yoke<MovementSpaceMove<'static>, C>,
}

impl<C: StableDeref> From<Yoke<MovementSpaceMove<'static>, C>> for MovementSpaceMoveOwned<C> {
    fn from(yoke: Yoke<MovementSpaceMove<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> MovementSpaceMoveOwned<C> {
    pub fn msm(&'a self) -> &'a MovementSpaceMove<'a> {
        self.yoke.get()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Yokeable)]
pub struct MovementSpaceMove<'a> {
    pub name: &'a str,
    pub map: &'a str,
    pub device: &'a str,
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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub control: u8,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
