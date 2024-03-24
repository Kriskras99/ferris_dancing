pub use byteorder::ByteOrder;
use byteorder::WriteBytesExt;

use super::{errors::WriterError, PathId, SplitPath};

pub trait WriteBytesExtUbiArt: std::io::Write {
    // Writes the components of the split path and the crc to the writer
    fn write_path<T: ByteOrder>(&mut self, path: &SplitPath<'_>) -> Result<(), WriterError> {
        if path.is_empty() {
            self.write_u32::<T>(0)?; // filename length
            self.write_u32::<T>(0)?; // directory length
            self.write_u32::<T>(0xFFFF_FFFF)?; // crc
        } else {
            self.write_string::<T>(&path.filename)?;
            self.write_string::<T>(&path.path)?;
            self.write_u32::<T>(u32::from(PathId::from(path)))?;
        }
        Ok(())
    }

    /// Writes the length of a string and the string itself to the writer.
    fn write_string<T: ByteOrder>(&mut self, string: &str) -> Result<(), WriterError> {
        self.write_u32::<T>(u32::try_from(string.len())?)?;
        self.write_all(string.as_bytes())?;
        Ok(())
    }
}

/// All types that implement `Write` get methods defined in `WriteBytesExt`
/// for free.
impl<W: std::io::Write + ?Sized> WriteBytesExtUbiArt for W {}
