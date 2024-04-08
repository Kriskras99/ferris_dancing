use std::borrow::Cow;

use dotstar_toolkit_utils::bytes::read::ReadError;

use super::addr_lib::surface_get_bits_per_pixel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtx<'a> {
    pub gfd: GfdHeader,
    pub blocks: Vec<Block<'a>>,
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
    pub tile_mode: u32,
    pub swizzle: u32,
    pub alignment: u32,
    pub pitch: u32,
    pub mip_offsets: [u32; 13],
    pub comp_sel: [u8; 4],
    pub bpp: u32,
    pub real_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Format {
    TcsR8G8B8A8Unorm = 0x0000_001A,
    TcsR8G8B8A8Srgb = 0x0000_041A,
    TcsR10G10B10A2Unorm = 0x0000_0019,
    TcsR5G6B5Unorm = 0x0000_0008,
    TcR5G5B5A1Unorm = 0x0000_000A,
    TcR4G4B4A4Unorm = 0x0000_000B,
    TcR8Unorm = 0x0000_0001,
    TcR8G8Unorm = 0x0000_0007,
    TcR4G4Unorm = 0x0000_0002,
    TBc1Unorm = 0x0000_0031,
    TBc1Srgb = 0x0000_0431,
    TBc2Unorm = 0x0000_0032,
    TBc2Srgb = 0x0000_0432,
    TBc3Unorm = 0x0000_0033,
    TBc3Srgb = 0x0000_0433,
    TBc4Unorm = 0x0000_0034,
    TBc4Snorm = 0x0000_0234,
    TBc5Unorm = 0x0000_0035,
    TBc5Snorm = 0x0000_0235,
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
    pub fn get_bpp(&self) -> u32 {
        (surface_get_bits_per_pixel(self.into()).unwrap_or_else(|_| unreachable!()) + 0x7) & !0x7
    }
}

impl TryFrom<u32> for Format {
    type Error = ReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0000_001A => Ok(Self::TcsR8G8B8A8Unorm),
            0x0000_041A => Ok(Self::TcsR8G8B8A8Srgb),
            0x0000_0019 => Ok(Self::TcsR10G10B10A2Unorm),
            0x0000_0008 => Ok(Self::TcsR5G6B5Unorm),
            0x0000_000A => Ok(Self::TcR5G5B5A1Unorm),
            0x0000_000B => Ok(Self::TcR4G4B4A4Unorm),
            0x0000_0001 => Ok(Self::TcR8Unorm),
            0x0000_0007 => Ok(Self::TcR8G8Unorm),
            0x0000_0002 => Ok(Self::TcR4G4Unorm),
            0x0000_0031 => Ok(Self::TBc1Unorm),
            0x0000_0431 => Ok(Self::TBc1Srgb),
            0x0000_0032 => Ok(Self::TBc2Unorm),
            0x0000_0432 => Ok(Self::TBc2Srgb),
            0x0000_0033 => Ok(Self::TBc3Unorm),
            0x0000_0433 => Ok(Self::TBc3Srgb),
            0x0000_0034 => Ok(Self::TBc4Unorm),
            0x0000_0234 => Ok(Self::TBc4Snorm),
            0x0000_0035 => Ok(Self::TBc5Unorm),
            0x0000_0235 => Ok(Self::TBc5Snorm),
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
