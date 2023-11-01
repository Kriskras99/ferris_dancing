use std::borrow::Cow;

use anyhow::Error;
use byteorder::WriteBytesExt;
pub use dotstar_toolkit_utils::bytes::*;

use super::{testing::test, PathId, SplitPath};

/// Read a `SplitPath` from `source` at position `position` and check the CRC
///
/// This function increments `position` with the size of the string + 12 if successful
///
/// # Errors
/// This function will return an error when the string would be (partially) outside the source.
pub fn read_path_at<'b, T: ByteOrder>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<Option<SplitPath<'b>>, Error> {
    let filename = read_string_at::<T>(source, position)?;
    let path = read_string_at::<T>(source, position)?;
    let path_id = PathId::from(read_u32_at::<T>(source, position)?);
    let result = if filename.is_empty() {
        test(&path.len(), &0)?;
        test(&path_id, &PathId::from(0xFFFF_FFFF))?;
        Ok(None)
    } else {
        let split_path = SplitPath {
            path: Cow::Borrowed(path),
            filename: Cow::Borrowed(filename),
        };
        let path_id_calc = PathId::from(&split_path);
        test(&path_id, &path_id_calc)?;
        Ok(Some(split_path))
    };
    let padding = read_u32_at::<T>(source, position)?;
    test(&padding, &0)?;
    result
}

pub trait WriteBytesExtUbiArt: std::io::Write {
    // Writes the components of the split path and the crc to the writer
    ///
    /// # Errors
    /// Will error if the individual components are longer than `u32::MAX` or if the writer fails
    fn write_path<T: ByteOrder>(&mut self, path: Option<&SplitPath<'_>>) -> Result<(), Error> {
        if let Some(path) = path {
            self.write_string::<T>(&path.filename)?;
            self.write_string::<T>(&path.path)?;
            self.write_u32::<T>(u32::from(PathId::from(path)))?;
        } else {
            self.write_u32::<T>(0)?; // filename length
            self.write_u32::<T>(0)?; // directory length
            self.write_u32::<T>(0xffff_ffff)?; // crc
        }
        Ok(())
    }

    /// Writes the length of a string and the string itself to the writer.
    ///
    /// # Errors
    /// Will error if the string is longer than `u32::MAX` or if the writer fails
    fn write_string<T: ByteOrder>(&mut self, string: &str) -> Result<(), Error> {
        self.write_u32::<T>(u32::try_from(string.len())?)?;
        self.write_all(string.as_bytes())?;
        Ok(())
    }
}

/// All types that implement `Write` get methods defined in `WriteBytesExt`
/// for free.
impl<W: std::io::Write + ?Sized> WriteBytesExtUbiArt for W {}
