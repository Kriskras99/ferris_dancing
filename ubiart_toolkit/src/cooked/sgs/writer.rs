use std::io::{Cursor, Write};

use anyhow::Error;
use byteorder::WriteBytesExt;

use super::{SceneConfigManager, SceneSettings, Sgs};

/// Write a `Sgs` file to the writer
///
/// # Errors
/// Will error when the JSON serialisation fails or the writer fails
pub fn create<W: Write>(mut writer: W, sgs: &Sgs) -> Result<(), Error> {
    writer.write_u8(b'S')?;
    serde_json::to_writer(&mut writer, sgs)?;
    writer.write_u8(0x0)?;

    Ok(())
}

/// Write a `SceneSettings` file to the writer
///
/// # Errors
/// Will error when the JSON serialisation fails or the writer fails
pub fn create_sgs<W: Write>(mut writer: W, sgs: &SceneSettings) -> Result<(), Error> {
    writer.write_u8(b'S')?;
    serde_json::to_writer(&mut writer, sgs)?;
    writer.write_u8(0x0)?;

    Ok(())
}

/// Write a `SceneConfigManager` file to the writer
///
/// # Errors
/// Will error when the JSON serialisation fails or the writer fails
pub fn create_sgscontainer<W: Write>(mut writer: W, sgs: &SceneConfigManager) -> Result<(), Error> {
    writer.write_u8(b'S')?;
    serde_json::to_writer(&mut writer, sgs)?;
    writer.write_u8(0x0)?;

    Ok(())
}

/// Create a `Sgs` file in a newly allocated `Vec`
///
/// # Errors
/// Will error when the JSON serialisation fails
pub fn create_vec(sgs: &Sgs) -> Result<Vec<u8>, Error> {
    let mut vec = vec![b'S'];
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, sgs)?;
    vec.push(0x0);
    vec.shrink_to_fit();
    Ok(vec)
}

/// Create a `SceneSettings` file in a newly allocated `Vec`
///
/// # Errors
/// Will error when the JSON serialisation fails
pub fn create_sgs_vec(sgs: &SceneSettings) -> Result<Vec<u8>, Error> {
    let mut vec = vec![b'S'];
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, sgs)?;
    vec.push(0x0);
    vec.shrink_to_fit();
    Ok(vec)
}

/// Create a `SceneConfigManager` file in a newly allocated `Vec`
///
/// # Errors
/// Will error when the JSON serialisation fails
pub fn create_sgscontainer_vec(sgs: &SceneConfigManager) -> Result<Vec<u8>, Error> {
    let mut vec = vec![b'S'];
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, sgs)?;
    vec.push(0x0);
    vec.shrink_to_fit();
    Ok(vec)
}
