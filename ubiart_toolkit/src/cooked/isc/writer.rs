use std::io::Write;

use serde::Serialize;
use ubiart_toolkit_shared_types::errors::WriterError;

use super::Root;

/// Write the `Root` to the writer
pub fn create<W: Write>(mut src: W, root: &Root) -> Result<(), WriterError> {
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
pub fn create_vec(root: &Root) -> Result<Vec<u8>, WriterError> {
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
pub fn create_vec_with_capacity_hint(root: &Root, capacity: usize) -> Result<Vec<u8>, WriterError> {
    let mut buf = String::with_capacity(capacity);
    buf.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>\n");
    let mut serializer = quick_xml::se::Serializer::with_root(&mut buf, Some("root"))
        .unwrap_or_else(|_| unreachable!());
    serializer.indent('\t', 1);
    root.serialize(serializer)?;
    buf.shrink_to_fit();
    Ok(buf.into_bytes())
}
