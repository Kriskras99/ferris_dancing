use anyhow::Error;

use super::Root;

/// Parse a isc file
///
/// # Errors
/// -  the file is not a isc file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Root<'_>, Error> {
    let string = std::str::from_utf8(src)?;
    let root: Root = quick_xml::de::from_str(string)?;

    Ok(root)
}
