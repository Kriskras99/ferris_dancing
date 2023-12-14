//! Contains the writer implementation
use std::io::{Seek, Write};

use byteorder::{BigEndian, WriteBytesExt};

use super::Alias8;
use crate::utils::{bytes::WriteBytesExtUbiArt, errors::WriterError};

/// Write an alias8 file.
///
/// # Errors
/// When there are too many aliases or the aliases are too long: `ParserError::IntegerConversionError`.
/// When the writer fails: `ParserError::IO`.
pub fn create<W: Write + Seek>(mut writer: W, alias8: &Alias8) -> Result<(), WriterError> {
    writer.write_u32::<BigEndian>(0x2)?;
    let alias_count = u32::try_from(alias8.aliases.len())?;
    writer.write_u32::<BigEndian>(alias_count)?;

    for alias in &alias8.aliases {
        writer.write_u32::<BigEndian>(u32::try_from(alias.first_alias.as_bytes().len())?)?;
        writer.write_all(alias.first_alias.as_bytes())?;
        writer.write_u32::<BigEndian>(u32::try_from(alias.second_alias.as_bytes().len())?)?;
        writer.write_all(alias.second_alias.as_bytes())?;
        writer.write_path::<BigEndian>(&alias.path)?;
        writer.write_u32::<BigEndian>(0)?;
        writer.write_u16::<BigEndian>(0xFFFF)?;
        writer.write_u16::<BigEndian>(alias.unk3)?;
    }

    Ok(())
}
