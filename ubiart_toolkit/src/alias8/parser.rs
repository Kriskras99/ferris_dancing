//! Contains the parser implementation

use byteorder::BigEndian;

use anyhow::Error;
use dotstar_toolkit_utils::testing::test;

use crate::utils::{
    bytes::{read_string_at, read_u16_at, read_u32_at},
    string_id_2,
};

use super::types::{Alias, Alias8};

/// Parse a .loc8 file
///
/// # Errors
/// -  the file is not a loc8 file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Alias8<'_>, Error> {
    let mut position = 0;

    // Read the unknown value at the beginning of the file and check it's correct
    let unk1 = read_u32_at::<BigEndian>(src, &mut position)?;
    test(&unk1, &0x2)?;

    // Read the amount of expected aliases
    let alias_count = read_u32_at::<BigEndian>(src, &mut position)?;

    // Create the vector where the aliases will be stored
    let mut aliases = Vec::with_capacity(usize::try_from(alias_count)?);

    for _ in 0..alias_count {
        // Read the strings
        let first_alias = read_string_at::<BigEndian>(src, &mut position)?;
        let second_alias = read_string_at::<BigEndian>(src, &mut position)?;
        let filename = read_string_at::<BigEndian>(src, &mut position)?;
        let path = read_string_at::<BigEndian>(src, &mut position)?;

        // Verify the path id
        let path_id = read_u32_at::<BigEndian>(src, &mut position)?;
        test(&path_id, &string_id_2(path, filename))?;

        // Read the unknown values and check them
        let unk1 = read_u32_at::<BigEndian>(src, &mut position)?;
        let unk2 = read_u16_at::<BigEndian>(src, &mut position)?;
        let unk3 = read_u16_at::<BigEndian>(src, &mut position)?;
        test(&unk1, &0x0)?;
        test(&unk2, &0xFFFF)?;
        aliases.push(Alias {
            first_alias,
            second_alias,
            filename,
            path,
            unk3,
        });
    }

    Ok(Alias8 { aliases })
}
