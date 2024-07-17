//! Contains the types that describe the usefull information in this filetype

use image::RgbaImage;

#[derive(Debug, PartialEq, Eq)]
pub struct Png {
    pub width: u16,
    pub height: u16,
    pub unk2: u32,
    pub unk5: u16,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u16,
    pub texture: RgbaImage,
}

impl Default for Png {
    /// Creates a Png with default values for a picto
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            unk2: 0,
            unk5: 0x2000,
            unk8: 0,
            unk9: 0,
            unk10: 0,
            texture: RgbaImage::default(),
        }
    }
}
