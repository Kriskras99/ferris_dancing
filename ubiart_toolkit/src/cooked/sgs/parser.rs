use dotstar_toolkit_utils::testing::{test, TestError};

use super::Sgs;
use crate::utils::errors::ParserError;

/// Parse a sgs file
pub fn parse(src: &[u8]) -> Result<Sgs<'_>, ParserError> {
    let src = clean_buffer_sgs(src)?;
    let sgs: Sgs = serde_json::from_slice(src)?;

    Ok(sgs)
}

/// Remove the 'S' at the front and '\0' at the back of the buffer.
fn clean_buffer_sgs(buffer: &[u8]) -> Result<&[u8], TestError> {
    test(&buffer[0], &b'S')?;
    test(&buffer[buffer.len() - 1], &0x0)?;

    Ok(&buffer[1..buffer.len() - 1])
}
