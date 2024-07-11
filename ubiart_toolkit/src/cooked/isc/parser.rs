use ubiart_toolkit_shared_types::errors::ParserError;

use super::Root;

/// Parse a isc file
pub fn parse(src: &[u8]) -> Result<Root<'_>, ParserError> {
    let string = std::str::from_utf8(src)?;
    let root: Root = quick_xml::de::from_str(string)?;

    Ok(root)
}
