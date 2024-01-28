use std::sync::Arc;

use byteorder::ByteOrder;

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
    fn read_len_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<usize, ReadError>
    where
        B: ByteOrder,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = Self::deserialize_at::<B>(reader, position)?;
            TryInto::<usize>::try_into(len)
                .map_err(|_| ReadError::custom("Len does not fit in usize!".into()))?
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
impl<'de> Len<'de> for u8 {}
impl<'de> Len<'de> for u16 {}
impl<'de> Len<'de> for u32 {}
impl<'de> Len<'de> for u64 {}
