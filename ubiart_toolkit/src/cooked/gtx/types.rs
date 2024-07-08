use std::{borrow::Cow, fmt::Debug};

use dotstar_toolkit_utils::bytes::read::ReadError;
use wiiu_swizzle::TileMode;

#[derive(Debug, PartialEq, Eq)]
pub struct Gtx {
    pub gfd: GfdHeader,
    pub images: Vec<Image>,
}

#[derive(PartialEq, Eq)]
pub struct Image {
    pub surface: Gx2Surface,
    pub data: Vec<u8>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("surface", &self.surface)
            .field("data", &format!("[u8; {}]", self.data.len()))
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GfdHeader {
    pub align_mode: u32,
}

impl GfdHeader {
    pub const MAGIC: u32 = 0x4766_7832;
    pub const GPU_VERSION: u32 = 0x2;
}

#[derive(Clone, PartialEq, Eq)]
pub enum Block<'a> {
    Surface(Gx2Surface),
    Data(Cow<'a, [u8]>),
    Mip(Cow<'a, [u8]>),
    Unknown(u32, Cow<'a, [u8]>),
}

impl Block<'_> {
    pub const MAGIC: u32 = 0x424C_4B7B;
}

impl std::fmt::Debug for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(arg0) => f.debug_tuple("Surface").field(arg0).finish(),
            Self::Data(arg0) => f
                .debug_tuple("Data")
                .field(&format!("[u8; {}]", arg0.len()))
                .finish(),
            Self::Mip(arg0) => f
                .debug_tuple("Mip")
                .field(&format!("[u8; {}]", arg0.len()))
                .finish(),
            Self::Unknown(type_it, arg0) => f
                .debug_tuple("Unknown")
                .field(type_it)
                .field(&format!("[u8; {}]", arg0.len()))
                .finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gx2Surface {
    pub dim: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub num_mips: u32,
    pub format: Format,
    pub use_it: u32,
    pub image_size: u32,
    pub image_ptr: u32,
    pub mip_size: u32,
    pub mip_ptr: u32,
    pub tile_mode: TileMode,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_offsets: [u32; 13],
    pub bpp: u32,
    pub real_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Format {
    TcsR8G8B8A8Unorm = 0x01A,
    TcsR8G8B8A8Srgb = 0x41A,
    TcsR10G10B10A2Unorm = 0x019,
    TcsR5G6B5Unorm = 0x008,
    TcR5G5B5A1Unorm = 0x00A,
    TcR4G4B4A4Unorm = 0x00B,
    TcR8Unorm = 0x001,
    TcR8G8Unorm = 0x007,
    TcR4G4Unorm = 0x002,
    TBc1Unorm = 0x031,
    TBc1Srgb = 0x431,
    TBc2Unorm = 0x032,
    TBc2Srgb = 0x432,
    TBc3Unorm = 0x033,
    TBc3Srgb = 0x433,
    TBc4Unorm = 0x034,
    TBc4Snorm = 0x234,
    TBc5Unorm = 0x035,
    TBc5Snorm = 0x235,
}

impl Format {
    #[must_use]
    pub const fn is_bcn(&self) -> bool {
        matches!(
            self,
            Self::TBc1Srgb
                | Self::TBc1Unorm
                | Self::TBc2Srgb
                | Self::TBc2Unorm
                | Self::TBc3Srgb
                | Self::TBc3Unorm
                | Self::TBc4Snorm
                | Self::TBc4Unorm
                | Self::TBc5Snorm
                | Self::TBc5Unorm
        )
    }

    #[must_use]
    pub const fn get_bpp(&self) -> u32 {
        match self {
            Self::TcR8Unorm => 1,
            Self::TcR4G4Unorm => 8,
            Self::TcsR5G6B5Unorm
            | Self::TcR5G5B5A1Unorm
            | Self::TcR4G4B4A4Unorm
            | Self::TcR8G8Unorm => 16,
            Self::TcsR8G8B8A8Unorm | Self::TcsR8G8B8A8Srgb | Self::TcsR10G10B10A2Unorm => 32,
            Self::TBc1Unorm | Self::TBc1Srgb | Self::TBc4Unorm | Self::TBc4Snorm => 64,
            Self::TBc2Unorm
            | Self::TBc2Srgb
            | Self::TBc3Unorm
            | Self::TBc3Srgb
            | Self::TBc5Unorm
            | Self::TBc5Snorm => 128,
        }
    }
}

impl TryFrom<u32> for Format {
    type Error = ReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x01A => Ok(Self::TcsR8G8B8A8Unorm),
            0x41A => Ok(Self::TcsR8G8B8A8Srgb),
            0x019 => Ok(Self::TcsR10G10B10A2Unorm),
            0x008 => Ok(Self::TcsR5G6B5Unorm),
            0x00A => Ok(Self::TcR5G5B5A1Unorm),
            0x00B => Ok(Self::TcR4G4B4A4Unorm),
            0x001 => Ok(Self::TcR8Unorm),
            0x007 => Ok(Self::TcR8G8Unorm),
            0x002 => Ok(Self::TcR4G4Unorm),
            0x031 => Ok(Self::TBc1Unorm),
            0x431 => Ok(Self::TBc1Srgb),
            0x032 => Ok(Self::TBc2Unorm),
            0x432 => Ok(Self::TBc2Srgb),
            0x033 => Ok(Self::TBc3Unorm),
            0x433 => Ok(Self::TBc3Srgb),
            0x034 => Ok(Self::TBc4Unorm),
            0x234 => Ok(Self::TBc4Snorm),
            0x035 => Ok(Self::TBc5Unorm),
            0x235 => Ok(Self::TBc5Snorm),
            _ => Err(ReadError::custom(format!("Unknown format!: 0x{value:x}"))),
        }
    }
}

impl From<Format> for u32 {
    #[allow(clippy::as_conversions, reason = "Format is repr(u32)")]
    fn from(value: Format) -> Self {
        value as Self
    }
}

impl From<&Format> for u32 {
    fn from(value: &Format) -> Self {
        (*value).into()
    }
}
