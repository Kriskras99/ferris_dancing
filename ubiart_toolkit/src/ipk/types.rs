//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use nohash_hasher::IntMap;
use yoke::Yokeable;

use crate::utils::{PathId, Platform, SplitPath, UniqueGameId};

#[derive(Clone, Yokeable)]
pub struct Bundle<'a> {
    pub version: u32,
    pub platform: Platform,
    pub unk4: u32,
    pub engine_version: u32,
    pub game_platform: UniqueGameId,
    pub files: IntMap<PathId, IpkFile<'a>>,
}

#[derive(Clone)]
pub struct IpkFile<'a> {
    pub timestamp: u64,
    pub path: SplitPath<'a>,
    pub is_cooked: bool,
    pub data: Data<'a>,
}

#[derive(Clone)]
pub enum Data<'a> {
    Uncompressed(Uncompressed<'a>),
    Compressed(Compressed<'a>),
}

impl Data<'_> {
    /// Get the size of the (uncompressed) data.
    #[must_use]
    pub fn len(&self) -> u64 {
        match self {
            Data::Uncompressed(data) => {
                u64::try_from(data.data.len()).unwrap_or_else(|_| unreachable!())
            }
            Data::Compressed(data) => {
                u64::try_from(data.uncompressed_size).unwrap_or_else(|_| unreachable!())
            }
        }
    }

    /// Check if this file is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Data::Uncompressed(data) => data.data.is_empty(),
            Data::Compressed(data) => data.uncompressed_size == 0,
        }
    }
}

#[derive(Clone)]
pub struct Uncompressed<'a> {
    pub data: Cow<'a, [u8]>,
}

#[derive(Clone)]
pub struct Compressed<'a> {
    pub uncompressed_size: usize,
    pub data: Cow<'a, [u8]>,
}
