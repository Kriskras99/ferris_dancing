//! Contains the types that describe the usefull information in this filetype

use anyhow::{anyhow, Error};
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
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
    pub mipmap_offsets: [u32; 0x10],
    pub unk1: u64,
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
            format: Format::DXT5,
            mipmaps: 1,
            slice_size: Default::default(),
            mipmap_offsets: Default::default(),
            unk1: 0x4_0000_0000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u32)]
pub enum Format {
    NvnFormatRGBA8 = 0x25,
    NvnFormatRGBA8SRGB = 0x38,
    NvnFormatRGB10A2 = 0x3d,
    NvnFormatRGB565 = 0x3c,
    NvnFormatRGB5A1 = 0x3b,
    NvnFormatRGBA4 = 0x39,
    NvnFormatR8 = 0x01,
    NvnFormatRG8 = 0x0d,
    DXT1 = 0x42,
    DXT3 = 0x43,
    DXT5 = 0x44,
    BC4U = 0x49,
    BC4S = 0x4a,
    BC5U = 0x4b,
    BC5S = 0x4c,
}

impl TryFrom<u32> for Format {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x25 => Ok(Self::NvnFormatRGBA8),
            0x38 => Ok(Self::NvnFormatRGBA8SRGB),
            0x3d => Ok(Self::NvnFormatRGB10A2),
            0x3c => Ok(Self::NvnFormatRGB565),
            0x3b => Ok(Self::NvnFormatRGB5A1),
            0x39 => Ok(Self::NvnFormatRGBA4),
            0x01 => Ok(Self::NvnFormatR8),
            0x0d => Ok(Self::NvnFormatRG8),
            0x42 => Ok(Self::DXT1),
            0x43 => Ok(Self::DXT3),
            0x44 => Ok(Self::DXT5),
            0x49 => Ok(Self::BC4U),
            0x4a => Ok(Self::BC4S),
            0x4b => Ok(Self::BC5U),
            0x4c => Ok(Self::BC5S),
            _ => Err(anyhow!("Unknown format! {value:x}")),
        }
    }
}

impl From<Format> for u32 {
    #[allow(clippy::as_conversions)]
    fn from(value: Format) -> Self {
        value as Self
    }
}

impl Format {
    #[must_use]
    pub const fn get_bpp(&self) -> u32 {
        match self {
            Self::NvnFormatR8 => 1,
            Self::NvnFormatRGB565
            | Self::NvnFormatRGB5A1
            | Self::NvnFormatRGBA4
            | Self::NvnFormatRG8 => 2,
            Self::NvnFormatRGBA8 | Self::NvnFormatRGBA8SRGB | Self::NvnFormatRGB10A2 => 4,
            Self::DXT1 | Self::BC4U | Self::BC4S => 8,
            Self::DXT3 | Self::DXT5 | Self::BC5U | Self::BC5S => 16,
        }
    }

    #[must_use]
    pub const fn is_bcn(&self) -> bool {
        matches!(
            self,
            Self::DXT1
                | Self::DXT3
                | Self::DXT5
                | Self::BC4U
                | Self::BC4S
                | Self::BC5U
                | Self::BC5S
        )
    }
}

pub enum BlockData<'a> {
    TextureHeader(TextureHeader),
    Data(&'a [u8]),
    Three(&'a [u8]),
}

pub struct Block<'a> {
    pub id: u32,
    pub data: BlockData<'a>,
}

pub struct Image {
    pub header: TextureHeader,
    pub data: Vec<Vec<u8>>,
}
pub struct Xtx {
    pub major_version: u32,
    pub minor_version: u32,
    pub images: Vec<Image>,
}

impl Default for Xtx {
    fn default() -> Self {
        Self {
            major_version: 0x1,
            minor_version: 0x1,
            images: Vec::new(),
        }
    }
}
