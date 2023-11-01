//! Contains the parser implementation

use std::{fs, path::Path};

use anyhow::Error;
use byteorder::BigEndian;
use memmap2::Mmap;
use yoke::Yoke;

use crate::utils::{
    bytes::{read_null_terminated_string_at, read_slice_at, read_u32_at},
    testing::{test, test_le},
};

use super::{MovementSpaceMove, MovementSpaceMoveOwned};

/// Open the file at the given path and parse it as a MovementSpaceMove file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<MovementSpaceMoveOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse(data))?;
    Ok(MovementSpaceMoveOwned::from(yoke))
}

/// Parse a MovementSpaceMove file
///
/// # Errors
/// -  the file is not a MovementSpaceMove file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<MovementSpaceMove<'_>, Error> {
    let mut position = 0;

    // Check the magic
    let unk1 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk1, &0x1)?;
    let unk2 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk2, &0x7)?;

    // There are always 64 bytes for the string
    let buffer: &[u8; 64] = read_slice_at(src, &mut position)?;
    // Only save the part with the string
    let name = read_null_terminated_string_at(buffer, &mut 0)?;

    let buffer: &[u8; 64] = read_slice_at(src, &mut position)?;
    let map = read_null_terminated_string_at(buffer, &mut 0)?;

    let buffer: &[u8; 64] = read_slice_at(src, &mut position)?;
    let device = read_null_terminated_string_at(buffer, &mut 0)?;

    let unk3 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk4 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk5 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk6 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk7 = read_u32_at::<BigEndian>(src, &mut position)?;

    let unk8 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk8, &0x211c_0000)?;
    let unk9 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk9, &0x0)?;
    let unk10 = read_u32_at::<BigEndian>(src, &mut position)?;
    test_le(&unk10, &0x3)?;
    let points = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk12 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk12, &0x2)?;
    let unk13 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk13, &0x0)?;

    let data = &src[position..];

    Ok(MovementSpaceMove {
        name,
        map,
        device,
        data,
        points,
        unk3,
        unk4,
        unk5,
        unk6,
        unk7,
        unk10,
    })
}
