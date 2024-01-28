use std::borrow::Cow;
use std::sync::Arc;

pub mod primitives;
pub mod read;

use self::read::BinaryDeserialize;
use self::read::ReadError;
use self::read::ZeroCopyReadAt;

/// Represents the length of a string or slice to read from the reader
pub trait Len<'de>: BinaryDeserialize<'de> + Sized + TryFrom<usize> + TryInto<usize> {
    /// Read the length at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn read_len_at(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
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
    /// Read a byte slice at `position`
    ///
    /// It will first read the length of the byte slice as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_slice_at<L>(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Cow<'de, [u8]>, ReadError>
    where
        L: Len<'de>
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(reader, position)?;
            reader.read_slice_at(position, len)?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_string_at<L>(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, ReadError>
    where
        L: Len<'de>
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(reader, position)?;
            match reader.read_slice_at(position, len)? {
                Cow::Borrowed(slice) => std::str::from_utf8(slice)
                    .map(Cow::Borrowed)
                    .map_err(|e| ReadError::invalid_utf8(len, *position, e))?,
                Cow::Owned(vec) => String::from_utf8(vec)
                    .map(Cow::Owned)
                    .map_err(|e| ReadError::invalid_utf8(len, *position, e.utf8_error()))?,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    // /// Write `length` at `position`
    // ///
    // /// Will increment position with the size of length if successful
    // ///
    // /// # Errors
    // /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    // fn write_len_at<B>(
    //     writer: &mut (impl ZeroCopyWriteAt + ?Sized),
    //     position: &mut u64,
    //     len: usize,
    // ) -> Result<(), NewWriteError>
    // where
    //     B: ByteOrder,
    // {
    //     let len = Self::try_from(len).map_err(|_| NewWriteError::too_many_bytes(*position))?;
    //     writer.write_at::<B>(position, &len)?;
    //     Ok(())
    // }
}
