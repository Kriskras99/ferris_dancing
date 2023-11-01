//! Contains the types that describe the usefull information in this filetype

use std::path::PathBuf;

use anyhow::{anyhow, Error};
use nohash_hasher::IntMap;
use stable_deref_trait::StableDeref;
use yoke::{Yoke, Yokeable};

use crate::utils::{self, GamePlatform, PathId, SplitPath};

// More values!
// https://github.com/RayCarrot/RayCarrot.RCP.Metro/blob/190c884a7745dedde6a33337a4c9684e5094c90a/src/RayCarrot.RCP.Metro/Archive/Manager/UbiArt_Ipk/UbiArtIPKArchiveConfigViewModel.cs#L85
// https://github.com/BinarySerializer/BinarySerializer.UbiArt/blob/main/src/DataTypes/Bundle/BundleBootHeader.cs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Platform {
    X360 = 0x1,
    Ps4 = 0x3,
    Wii = 0x5,
    WiiU = 0x8,
    Nx = 0xb,
}

impl TryFrom<u32> for Platform {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1 => Ok(Self::X360),
            0x3 => Ok(Self::Ps4),
            0x5 => Ok(Self::Wii),
            0x8 => Ok(Self::WiiU),
            0xb => Ok(Self::Nx),
            _ => Err(anyhow!("Unknown platform id {value}!")),
        }
    }
}

impl From<Platform> for u32 {
    #[allow(clippy::as_conversions)]
    fn from(value: Platform) -> Self {
        value as Self
    }
}

impl Platform {
    #[must_use]
    pub fn matches_game_platform(&self, gp: GamePlatform) -> bool {
        match self {
            Self::X360 => gp.platform == utils::Platform::X360,
            Self::Ps4 => gp.platform == utils::Platform::Ps4,
            Self::Wii => gp.platform == utils::Platform::Wii,
            Self::WiiU => gp.platform == utils::Platform::WiiU,
            Self::Nx => gp.platform == utils::Platform::Nx,
        }
    }
}

pub struct BundleOwned<C: StableDeref> {
    yoke: Yoke<Bundle<'static>, C>,
}

impl<C: StableDeref> From<Yoke<Bundle<'static>, C>> for BundleOwned<C> {
    fn from(yoke: Yoke<Bundle<'static>, C>) -> Self {
        Self { yoke }
    }
}

impl<'a, C: StableDeref> BundleOwned<C> {
    pub fn version(&self) -> u32 {
        self.yoke.get().version
    }

    pub fn platform(&self) -> Platform {
        self.yoke.get().platform
    }

    pub fn unk4(&self) -> u32 {
        self.yoke.get().unk4
    }

    pub fn engine_version(&self) -> u32 {
        self.yoke.get().engine_version
    }

    pub fn game_platform(&self) -> GamePlatform {
        self.yoke.get().game_platform
    }

    pub fn get_file(&'a self, path_id: &PathId) -> Option<&'a IpkFile<'a>> {
        self.yoke.get().files.get(path_id)
    }

    pub fn list_files(&self) -> Vec<String> {
        let ipk = &self.yoke.get().files;
        let mut path = Vec::with_capacity(ipk.len());
        for file in ipk.values() {
            path.push(format!("{}{}", file.path.path, file.path.filename));
        }
        path
    }
}

#[derive(Clone, Yokeable)]
pub struct Bundle<'a> {
    pub version: u32,
    pub platform: Platform,
    pub unk4: u32,
    pub engine_version: u32,
    pub game_platform: GamePlatform,
    pub files: IntMap<PathId, IpkFile<'a>>,
}

#[derive(Clone)]
pub struct IpkFile<'a> {
    pub timestamp: u64,
    pub path: SplitPath<'a>,
    pub is_cooked: bool,
    pub data: Data<'a>,
}

#[derive(Clone, Copy)]
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
        self.len() == 0
    }
}

#[derive(Clone, Copy)]
pub struct Uncompressed<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Copy)]
pub struct Compressed<'a> {
    pub uncompressed_size: usize,
    pub data: &'a [u8],
}

pub struct FileToPack {
    pub file_path: PathBuf,
    pub name: String,
    pub path: String,
    pub is_cooked: bool,
    pub compress: bool,
    pub timestamp: u64,
    pub checksum: u32,
}
