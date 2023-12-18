//! Contains the parser implementation

use byteorder::BigEndian;
use dotstar_toolkit_utils::{
    bytes::{read_string_at, read_u16_at, read_u32_at},
    testing::{test, test_any},
};

use super::types::{Alias, Alias8};
use crate::utils::{bytes::read_path_at, errors::ParserError};

/// Parse an .alias8 file
pub fn parse(src: &[u8]) -> Result<Alias8<'_>, ParserError> {
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
        let path = read_path_at::<BigEndian>(src, &mut position)?;

        // Read the unknown values and check them
        let unk2 = read_u16_at::<BigEndian>(src, &mut position)?;
        let unk3 = read_u16_at::<BigEndian>(src, &mut position)?;
        test(&unk2, &0xFFFF)?;
        test_any(&unk3, Alias::UNK3)?;
        aliases.push(Alias {
            first_alias,
            second_alias,
            path,
            unk3,
        });
    }

    Ok(Alias8 { aliases })
}
