use std::io::{Cursor, Write};

use serde::Serialize;

use crate::utils::errors::WriterError;

/// Write the json structure to the writer, appending a null byte.
pub fn create(mut writer: impl Write, value: &impl Serialize) -> Result<(), WriterError> {
    serde_json::to_writer(&mut writer, value)?;
    writer.write_all(&[0x0])?;

    Ok(())
}

/// Create the json structure in a newly allocated `Vec`, appending a null byte.
pub fn create_vec(value: &impl Serialize) -> Result<Vec<u8>, WriterError> {
    let mut vec = Vec::with_capacity(1000);
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, value)?;
    vec.push(0);
    vec.shrink_to_fit();
    Ok(vec)
}

/// Create the json structure in a newly allocated `Vec` with initial capacity `capacity`, appending a null byte.
pub fn create_vec_with_capacity_hint(
    value: &impl Serialize,
    capacity: usize,
) -> Result<Vec<u8>, WriterError> {
    let mut vec = Vec::with_capacity(capacity);
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, value)?;
    vec.push(0);
    vec.shrink_to_fit();
    Ok(vec)
}
