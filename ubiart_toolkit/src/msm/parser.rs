//! Contains the parser implementation

use byteorder::BigEndian;
use dotstar_toolkit_utils::{
    bytes::{read_null_terminated_string_at, read_slice_at, read_u32_at},
    testing::{test, test_le},
};

use super::MovementSpaceMove;
use crate::utils::errors::ParserError;

/// Parse a MovementSpaceMove file
///
/// # Errors
/// -  the file is not a MovementSpaceMove file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<MovementSpaceMove<'_>, ParserError> {
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
    test(&device, &"Acc_Dev_Dir_NP")?;

    let unk3 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk4 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk5 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk6 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk7 = read_u32_at::<BigEndian>(src, &mut position)?;

    let unk8 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk8, &0x211C_0000)?;
    let unk9 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk9, &0x0)?;
    let unk10 = read_u32_at::<BigEndian>(src, &mut position)?;
    test_le(&unk10, &0x3)?;
    let points = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk12 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk12, &0x2)?;
    let unk13 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk13, &0x0)?;

    let unk14 = read_u32_at::<BigEndian>(src, &mut position)?;
    let unk15 = read_u32_at::<BigEndian>(src, &mut position)?;

    let mut data = Vec::with_capacity(usize::try_from(points)?);
    for _ in 0..points {
        let x = read_u32_at::<BigEndian>(src, &mut position)?;
        let y = read_u32_at::<BigEndian>(src, &mut position)?;
        data.push((x, y));
    }

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
        unk14,
        unk15,
    })
}
