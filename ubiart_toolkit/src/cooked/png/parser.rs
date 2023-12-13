//! Contains the parser implementation

use anyhow::Error;
use byteorder::BigEndian;
use dotstar_toolkit_utils::testing::{test, test_any};

use super::Png;
use crate::{
    cooked::xtx,
    utils::bytes::{read_u16_at, read_u32_at, read_u64_at},
};

/// Parse a .png.ckd file
///
/// # Errors
/// -  the file is not a .png.ckd file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Png, Error> {
    let mut position = 0;

    let magic = read_u64_at::<BigEndian>(src, &mut position)?;
    test(&magic, &0x9_5445_5800)?;

    let header_size = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&header_size, &0x2C)?;

    let unk2 = read_u32_at::<BigEndian>(src, &mut position)?;

    let width = read_u16_at::<BigEndian>(src, &mut position)?;
    let height = read_u16_at::<BigEndian>(src, &mut position)?;

    let unk4 = read_u16_at::<BigEndian>(src, &mut position)?;
    test(&unk4, &0x0001)?;

    let unk5 = read_u16_at::<BigEndian>(src, &mut position)?;
    test_any(&unk5, &[0x1800, 0x1801, 0x2000, 0x2002])?;

    let unk6 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk2, &unk6)?;

    let unk7 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk7, &0x0)?;

    let unk8 = read_u32_at::<BigEndian>(src, &mut position)?;
    // largest values are all montage
    let unk9 = read_u32_at::<BigEndian>(src, &mut position)?;

    let unk10 = read_u16_at::<BigEndian>(src, &mut position)?;
    // montage is always 0x0202
    test_any(&unk10, &[0x0202, 0x0])?;

    // Always zero for just dance 2022
    let _unk11 = read_u16_at::<BigEndian>(src, &mut position)?;

    // Start of XTX header (0x2C)
    let xtx = xtx::parse(&src[0x2C..])?;

    if xtx.images.len() > 1 {
        println!("Multiple XTX images!");
    }

    Ok(Png {
        width,
        height,
        unk2,
        unk5,
        unk8,
        unk9,
        unk10,
        xtx,
    })
}
