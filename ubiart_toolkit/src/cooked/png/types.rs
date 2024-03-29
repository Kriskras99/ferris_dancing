//! Contains the types that describe the usefull information in this filetype

use crate::cooked::xtx::Xtx;

#[derive(Debug, PartialEq, Eq)]
pub struct Png {
    pub width: u16,
    pub height: u16,
    pub unk2: u32,
    pub unk5: u16,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u16,
    pub xtx: Xtx,
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
            xtx: Xtx::default(),
        }
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for Png {
    fn arbitrary(u: &mut arbitrary::Unstructured) -> arbitrary::Result<Self> {
        let xtx: Xtx = u.arbitrary()?;

        Ok(Self {
            width: xtx.images[0].header.width as u16,
            height: xtx.images[0].header.height as u16,
            unk2: u.arbitrary()?,
            unk5: *u.choose(&[0x1800, 0x1801, 0x2000, 0x2002])?,
            unk8: u.arbitrary()?,
            unk9: u.arbitrary()?,
            unk10: *u.choose(&[0x0202, 0x0])?,
            xtx,
        })
    }
}
