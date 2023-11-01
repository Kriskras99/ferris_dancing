use std::io::{Cursor, Write};

use anyhow::Error;
use byteorder::WriteBytesExt;

use crate::json_types::v22::Template22;

/// Write the template file to the writer.
///
/// # Errors
/// Will error when serialisation fails
pub fn create<W: Write>(mut writer: W, tpl: &Template22<'_>) -> Result<(), Error> {
    serde_json::to_writer(&mut writer, tpl)?;
    writer.write_u8(0x0)?;

    Ok(())
}

/// Create the template file in a newly allocated `Vec`.
///
/// # Errors
/// Will error when serialisation fails
pub fn create_vec(tpl: &Template22<'_>) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::with_capacity(1000);
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, tpl)?;
    vec.push(0);
    vec.shrink_to_fit();
    Ok(vec)
}

/// Create the template file in a newly allocated `Vec` with initial capacity `capacity`.
///
/// # Errors
/// Will error when serialisation fails
pub fn create_vec_with_capacity_hint(
    tpl: &Template22<'_>,
    capacity: usize,
) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::with_capacity(capacity);
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, tpl)?;
    vec.push(0);
    vec.shrink_to_fit();
    Ok(vec)
}
