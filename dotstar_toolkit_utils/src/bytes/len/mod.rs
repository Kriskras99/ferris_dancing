use super::{
    read::{BinaryDeserialize, BinaryDeserializeExt, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
};

/// Represents the length of a string or slice to read from the reader
pub trait Len<'a>: BinaryDeserializeExt<'a> + BinarySerialize
where
    <Self as BinaryDeserialize<'a>>::Ctx: Default,
    <Self as BinarySerialize>::Ctx: Default,
    Self::Output: TryInto<usize>,
    Self::Input: TryFrom<usize>,
{
    /// Read the length at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn read_len_at(
        reader: &'a (impl ReadAtExt + ?Sized),
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
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        len: usize,
    ) -> Result<(), WriteError> {
        let len = Self::Input::try_from(len).unwrap_or_else(|_| todo!());
        writer.write_at::<Self>(position, len)?;
        Ok(())
    }
}
