use std::{
    collections::HashMap,
    io::{Cursor, Write},
};

use anyhow::Error;
use byteorder::{BigEndian, WriteBytesExt};

use crate::{
    loc8::types::Loc8,
    utils::{bytes::WriteBytesExtUbiArt, LocaleId},
};

use super::types::Language;

/// Creates a .loc8 file and writes it to the writer
///
/// # Errors
/// Will error if there are more than `u32::MAX` translations in the map or if the writer fails
pub fn create<W: Write, S: AsRef<str>>(
    mut writer: W,
    language: Language,
    strings: &HashMap<LocaleId, S>,
) -> Result<(), Error> {
    writer.write_u32::<BigEndian>(1)?;
    writer.write_u32::<BigEndian>(u32::from(language))?;
    writer.write_u32::<BigEndian>(u32::try_from(strings.len())?)?;

    let mut ids: Vec<_> = strings.keys().collect();
    ids.sort();

    for id in ids {
        writer.write_u32::<BigEndian>(u32::from(*id))?;
        let string = strings.get(id).unwrap_or_else(|| unreachable!()).as_ref();
        writer.write_string::<BigEndian>(string)?;
    }

    writer.write_u32::<BigEndian>(0)?; // unk2
    writer.write_all(&Loc8::FOOTERS[0])?;
    Ok(())
}

/// Creates a .loc8 file in a newly allocated `Vec`
///
/// # Errors
/// Will error if there are more than `u32::MAX` translations in the map
pub fn create_vec<S: AsRef<str>>(
    language: Language,
    strings: &HashMap<LocaleId, S>,
) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::with_capacity(2_000_000);
    let cursor = Cursor::new(&mut vec);
    create(cursor, language, strings)?;
    vec.shrink_to_fit();
    Ok(vec)
}
