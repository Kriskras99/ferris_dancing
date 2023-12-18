use super::Root;
use crate::utils::errors::ParserError;

/// Parse a isc file
pub fn parse(src: &[u8]) -> Result<Root<'_>, ParserError> {
    let string = std::str::from_utf8(src)?;
    let root: Root = quick_xml::de::from_str(string)?;

    Ok(root)
}
