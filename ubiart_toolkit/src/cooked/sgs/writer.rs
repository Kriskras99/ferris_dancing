use std::io::Cursor;

use super::{SceneConfigManager, SceneSettings, Sgs};
use crate::utils::errors::WriterError;

/// Create a `Sgs` file in a newly allocated `Vec`
///
/// # Errors
/// Will error when the JSON serialisation fails
pub fn create_vec(sgs: &Sgs) -> Result<Vec<u8>, WriterError> {
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
pub fn create_sgs_vec(sgs: &SceneSettings) -> Result<Vec<u8>, WriterError> {
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
pub fn create_sgscontainer_vec(sgs: &SceneConfigManager) -> Result<Vec<u8>, WriterError> {
    let mut vec = vec![b'S'];
    let cursor = Cursor::new(&mut vec);
    serde_json::to_writer(cursor, sgs)?;
    vec.push(0x0);
    vec.shrink_to_fit();
    Ok(vec)
}
