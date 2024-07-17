//! Contains the types that describe the usefull information in this filetype

use std::{borrow::Cow, collections::VecDeque};

use crate::decoder::DecoderError;

pub struct XtxRaw<'a> {
    pub minor_version: u32,
    pub blocks: VecDeque<Block<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureHeader {
    pub image_size: u64,
    pub alignment: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub target: u32,
    pub format: Format,
    pub mipmaps: u32,
    pub slice_size: u32,
    pub mipmap_offsets: [u32; 17],
    /// Set to zero when creating this struct
    pub block_height_log2: u8,
}

impl Default for TextureHeader {
    fn default() -> Self {
        Self {
            image_size: Default::default(),
            alignment: 0x200,
            width: Default::default(),
            height: Default::default(),
            depth: 0x1,
            target: 0x1,
            format: Format::BC3,
            mipmaps: 1,
            slice_size: Default::default(),
            mipmap_offsets: Default::default(),
            block_height_log2: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Format {
    NvnFormatRGBA8 = 0x25,
    NvnFormatRGBA8SRGB = 0x38,
    NvnFormatRGB10A2 = 0x3D,
    NvnFormatRGB565 = 0x3C,
    NvnFormatRGB5A1 = 0x3B,
    NvnFormatRGBA4 = 0x39,
    NvnFormatR8 = 0x01,
    NvnFormatRG8 = 0x0D,
    /// Also known as DXT1
    BC1 = 0x42,
    /// Also known as DXT3
    BC2 = 0x43,
    /// Also known as DXT5
    BC3 = 0x44,
    BC4U = 0x49,
    BC4S = 0x4A,
    BC5U = 0x4B,
    BC5S = 0x4C,
}

impl TryFrom<u32> for Format {
    type Error = DecoderError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x25 => Ok(Self::NvnFormatRGBA8),
            0x38 => Ok(Self::NvnFormatRGBA8SRGB),
            0x3D => Ok(Self::NvnFormatRGB10A2),
            0x3C => Ok(Self::NvnFormatRGB565),
            0x3B => Ok(Self::NvnFormatRGB5A1),
            0x39 => Ok(Self::NvnFormatRGBA4),
            0x01 => Ok(Self::NvnFormatR8),
            0x0D => Ok(Self::NvnFormatRG8),
            0x42 => Ok(Self::BC1),
            0x43 => Ok(Self::BC2),
            0x44 => Ok(Self::BC3),
            0x49 => Ok(Self::BC4U),
            0x4A => Ok(Self::BC4S),
            0x4B => Ok(Self::BC5U),
            0x4C => Ok(Self::BC5S),
            _ => Err(DecoderError::UnknownTextureFormat(value)),
        }
    }
}

impl From<Format> for u32 {
    #[allow(
        clippy::as_conversions,
        reason = "Format is repr(u32) thus this is always safe"
    )]
    fn from(value: Format) -> Self {
        value as Self
    }
}

impl Format {
    #[must_use]
    /// Get the amount of bytes per pixel/texel
    pub const fn get_bpp(self) -> u32 {
        match self {
            Self::NvnFormatR8 => 1,
            Self::NvnFormatRGB565
            | Self::NvnFormatRGB5A1
            | Self::NvnFormatRGBA4
            | Self::NvnFormatRG8 => 2,
            Self::NvnFormatRGBA8 | Self::NvnFormatRGBA8SRGB | Self::NvnFormatRGB10A2 => 4,
            Self::BC1 | Self::BC4U | Self::BC4S => 8,
            Self::BC2 | Self::BC3 | Self::BC5U | Self::BC5S => 16,
        }
    }

    #[must_use]
    pub const fn is_bcn(self) -> bool {
        matches!(
            self,
            Self::BC1 | Self::BC2 | Self::BC3 | Self::BC4U | Self::BC4S | Self::BC5U | Self::BC5S
        )
    }
}

pub const TEX_HEAD_BLK_TYPE: u32 = 0x2;
pub const DATA_BLK_TYPE: u32 = 0x3;
pub const UNKNOWN_BLK_TYPE_FIVE: u32 = 0x5;
pub const FIVE_EXPECTED_DATA: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub struct Block<'a> {
    pub id: u32,
    pub data: BlockData<'a>,
}

pub enum BlockData<'a> {
    TextureHeader(TextureHeader),
    DataLazy(Data),
    Data(Vec<u8>),
    Five(Cow<'a, [u8]>),
}

pub struct Data {
    pub position: u64,
    pub size: usize,
}
