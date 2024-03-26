pub mod endian;
pub mod primitives;
// pub mod primitives2;
pub mod read;
pub mod write;

use std::fs::File;
use std::path::Path;

use self::read::BinaryDeserialize;
use self::read::ReadError;
use self::read::ZeroCopyReadAtExt;
use self::write::BinarySerialize;
use self::write::WriteError;
use self::write::ZeroCopyWriteAt;

/// Read the file at path into a `Vec`
///
/// # Errors
/// - Cannot open the file
/// - Cannot get metadata for the file
/// - Filesize is bigger than `usize`
/// - Cannot read the entire file
pub fn read_to_vec<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())?;
    let file_size = usize::try_from(file.metadata()?.len()).map_err(std::io::Error::other)?;
    let mut buf = Vec::with_capacity(file_size);
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    Ok(buf)
}

/// Represents the length of a string or slice to read from the reader
pub trait Len<'rf>:
    BinaryDeserialize<'rf> + BinarySerialize + Sized + TryFrom<usize> + TryInto<usize>
{
    /// Read the length at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn read_len_at(
        reader: &'rf (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<usize, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = Self::deserialize_at(reader, position)?;
            TryInto::<usize>::try_into(len)
                .map_err(|_| ReadError::custom("Len does not fit in usize!".into()))?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Write `length` at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn write_len_at(
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
        len: usize,
    ) -> Result<(), WriteError> {
        let len = Self::try_from(len).unwrap_or_else(|_| todo!());
        writer.write_at(position, &len)?;
        Ok(())
    }
}