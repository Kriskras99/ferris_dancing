//! Contains the parser implementation

use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
};

use anyhow::{Context, Error};
use byteorder::BigEndian;

use crate::{
    loc8::types::Language,
    utils::{
        bytes::{read_string_at, read_u32_at},
        LocaleId,
    },
};
use dotstar_toolkit_utils::testing::test_any;

use super::types::Loc8;

/// Parse a .loc8 file
///
/// # Errors
/// -  the file is not a loc8 file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Loc8<'_>, Error> {
    let mut position = 0;

    let unk1 = read_u32_at::<BigEndian>(src, &mut position)?;
    let language = Language::try_from(read_u32_at::<BigEndian>(src, &mut position)?)?;

    // When unk1 == 2 there's a second version of strings
    // However these alternative strings seem to be riddled with typos, so just use the first one
    test_any(&unk1, &[1, 2])?;

    let mut strings = HashMap::new();

    for i in 0..unk1 {
        let string_count = read_u32_at::<BigEndian>(src, &mut position)?;

        for _ in 0..string_count {
            let id = LocaleId::from(read_u32_at::<BigEndian>(src, &mut position)?);
            let string = read_string_at::<BigEndian>(src, &mut position)
                .with_context(|| format!("ID: {id:?}, POS: {position}"))?;

            if i == 0 {
                strings.insert(id, Cow::Borrowed(string));
            } else if i == 1 {
                // Only insert new strings the second time around
                if let Entry::Vacant(e) = strings.entry(id) {
                    e.insert(Cow::Borrowed(string));
                }
            }
        }

        let _unk2 = read_u32_at::<BigEndian>(src, &mut position)?;
    }

    if test_any(&src[position..position + 100].try_into()?, Loc8::FOOTERS).is_err() {
        println!(
            "Warning! Unexpected footer in loc8 file: {:?}",
            &src[position..position + 100]
        );
    }

    if src.len() > position + 100 {
        println!(
            "Warning! loc8 file is bigger than expected! Expected: {}, actual: {}",
            position + 100,
            src.len()
        );
    }

    Ok(Loc8 { language, strings })
}
