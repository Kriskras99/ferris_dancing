use std::{fs, path::Path};

use anyhow::Error;
use memmap2::Mmap;
use yoke::Yoke;

use super::{Root, RootOwned};

/// Open the file at the given path and parse it as a isc file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<RootOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse(data))?;
    Ok(RootOwned::from(yoke))
}

/// Parse a isc file
///
/// # Errors
/// -  the file is not a isc file or the parser encounters an unexpected value.
pub fn parse(src: &[u8]) -> Result<Root<'_>, Error> {
    let string = std::str::from_utf8(src)?;
    let root: Root = quick_xml::de::from_str(string)?;

    Ok(root)
}
