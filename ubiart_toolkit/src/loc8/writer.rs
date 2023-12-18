use std::io::{Cursor, Seek, SeekFrom, Write};

use byteorder::{BigEndian, WriteBytesExt};

use super::types::Language;
use crate::{
    loc8::types::Loc8,
    utils::{bytes::WriteBytesExtUbiArt, errors::WriterError, LocaleId},
};

/// Creates a .loc8 file and writes it to the writer
pub fn create<W: Write + Seek>(
    mut writer: W,
    language: Language,
    strings: impl Iterator<Item = (LocaleId, &'_ str)>,
) -> Result<(), WriterError> {
    writer.write_u32::<BigEndian>(1)?;
    writer.write_u32::<BigEndian>(u32::from(language))?;
    let size_hint = u32::try_from(strings.size_hint().0)?;
    writer.write_u32::<BigEndian>(size_hint)?;

    let mut actual_size = 0;

    for (locale_id, string) in strings {
        writer.write_u32::<BigEndian>(u32::from(locale_id))?;
        writer.write_string::<BigEndian>(string)?;
        actual_size += 1;
    }

    if size_hint != actual_size {
        writer.seek(SeekFrom::Start(8))?;
        writer.write_u32::<BigEndian>(actual_size)?;
    }

    writer.write_u32::<BigEndian>(0)?; // unk2
    writer.write_all(&Loc8::FOOTERS[0])?;
    Ok(())
}

/// Creates a .loc8 file in a newly allocated `Vec`
pub fn create_vec(
    language: Language,
    strings: impl Iterator<Item = (LocaleId, &'_ str)>,
) -> Result<Vec<u8>, WriterError> {
    let mut vec = Vec::with_capacity(2_000_000);
    let cursor = Cursor::new(&mut vec);
    create(cursor, language, strings)?;
    vec.shrink_to_fit();
    Ok(vec)
}
