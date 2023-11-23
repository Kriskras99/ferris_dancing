use anyhow::Error;

use dotstar_toolkit_utils::testing::test;

use super::Sgs;

/// Parse a sgs file
///
/// # Errors
/// -  the file is not a sgs file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Sgs<'_>, Error> {
    let src = clean_buffer_sgs(src)?;
    let sgs: Sgs = serde_json::from_slice(src)?;

    Ok(sgs)
}

/// Remove the 'S' at the front and '\0' at the back of the buffer.
///
/// # Errors
/// Will error when the 'S' or the '\0' are missing
fn clean_buffer_sgs(buffer: &[u8]) -> Result<&[u8], Error> {
    test(&buffer[0], &b'S')?;
    test(&buffer[buffer.len() - 1], &0x0)?;

    Ok(&buffer[1..buffer.len() - 1])
}
