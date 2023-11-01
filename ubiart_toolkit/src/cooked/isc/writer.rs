use std::io::Write;

use anyhow::Error;
use serde::Serialize;

use super::Root;

/// Write the `Root` to the writer
///
/// # Errors
/// Will error when serialisation fails or the writer fails
pub fn create<W: Write>(mut src: W, root: &Root) -> Result<(), Error> {
    let mut buf = String::with_capacity(1000);
    buf.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
        .unwrap_or_else(|_| unreachable!());
    serializer.indent('\t', 1);
    root.serialize(serializer)?;
    src.write_all(buf.as_bytes())?;
    Ok(())
}

/// Create a `Vec` with the XML representation of `root`
///
/// # Errors
/// Will error when serialisation fails
pub fn create_vec(root: &Root) -> Result<Vec<u8>, Error> {
    let mut buf = String::with_capacity(1000);
    buf.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
        .unwrap_or_else(|_| unreachable!());
    serializer.indent('\t', 1);
    root.serialize(serializer)?;
    buf.shrink_to_fit();
    Ok(buf.into_bytes())
}

/// Create a `Vec` with the XML representation of `root`, providing a capacity hint to prevent reallocations.
///
/// # Errors
/// Will error when serialisation fails
pub fn create_vec_with_capacity_hint(root: &Root, capacity: usize) -> Result<Vec<u8>, Error> {
    let mut buf = String::with_capacity(capacity);
    buf.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
        .unwrap_or_else(|_| unreachable!());
    serializer.indent('\t', 1);
    root.serialize(serializer)?;
    buf.shrink_to_fit();
    Ok(buf.into_bytes())
}
