//! Contains the new byte writing traits
mod error;
mod impls;

pub use error::*;

use super::{len::Len, read::BinaryDeserialize};

/// Represents a object that can be deserialized from a binary file
pub trait BinarySerialize {
    type Ctx;
    type Input: Sized;
    /// Serialize this type to the writer with `ctx`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when serializing fails.
    #[inline]
    fn serialize_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        Self::serialize_at_with_ctx(input, writer, &mut 0, ctx)
    }

    /// Serialize this type to the writer with `ctx` at `position`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when serializing fails.
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<(), WriteError>;
}

// impl<'a, T> BinarySerialize for &'a T
// where
//     T: BinarySerialize,
//     T::Input: 'a + Copy,
// {
//     type Ctx = T::Ctx;
//     type Input = &'a T::Input;

//     fn serialize_at_with_ctx(
//         input: Self::Input,
//         writer: &mut (impl WriteAt + ?Sized),
//         position: &mut u64,
//         ctx: Self::Ctx,
//     ) -> Result<(), WriteError> {
//         T::serialize_at_with_ctx(*input, writer, position, ctx)
//     }
// }

pub trait BinarySerializeExt: BinarySerialize
where
    Self::Ctx: Default,
{
    /// Serialize this type to the writer
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when serializing fails.
    #[inline]
    fn serialize(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
    ) -> Result<(), WriteError> {
        Self::serialize_with_ctx(input, writer, Self::Ctx::default())
    }

    /// Serialize this type to the writer at `position`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when serializing fails.
    #[inline]
    fn serialize_at(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        Self::serialize_at_with_ctx(input, writer, position, Self::Ctx::default())
    }
}

impl<T: BinarySerialize + ?Sized> BinarySerializeExt for T where T::Ctx: Default {}

/// Represents a byte source which uses Cow's to stay zerocopy
pub trait WriteAt {
    /// Write a `T` at `position`
    ///
    /// This function increments `position` with what `T` writes if successful
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the writer.
    fn write_at<T: BinarySerializeExt + ?Sized>(
        &mut self,
        position: &mut u64,
        ty: T::Input,
    ) -> Result<(), WriteError>
    where
        T::Ctx: Default,
    {
        T::serialize_at(ty, self, position)
    }

    /// Write a `T` at `position` with `ctx`
    ///
    /// This function increments `position` with what `T` writes if successful
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the writer.
    fn write_at_with_ctx<T>(
        &mut self,
        position: &mut u64,
        ty: T::Input,
        ctx: T::Ctx,
    ) -> Result<(), WriteError>
    where
        T: BinarySerialize + ?Sized,
    {
        T::serialize_at_with_ctx(ty, self, position, ctx)
    }

    /// Write a `&[u8]` at `position`
    ///
    /// This function increments `position` with the size of the slice if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the writer.
    fn write_slice_at(&mut self, position: &mut u64, buf: &[u8]) -> Result<(), WriteError>;

    /// Write the length and string at `position`
    ///
    /// It will first write the length of the string as a [`Len`]
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the writer.
    #[inline]
    fn write_len_string_at<'de, L>(
        &mut self,
        position: &mut u64,
        string: &str,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
        <L as BinaryDeserialize<'de>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
    {
        let slice = string.as_bytes();
        self.write_len_slice_at::<L>(position, slice)
    }

    /// Write the length and byte slice at `position`
    ///
    /// It will first write the length of the byte slice as a [`Len`].
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the writer.
    #[inline]
    fn write_len_slice_at<'de, L>(
        &mut self,
        position: &mut u64,
        buf: &[u8],
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
        <L as BinaryDeserialize<'de>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
    {
        L::write_len_at(self, position, buf.len())?;
        self.write_slice_at(position, buf)?;
        Ok(())
    }

    /// Write the length and the vector of `T` at `position`
    ///
    /// It will first write the length of the vector as a [`Len`].
    /// This function increments `position` with the size of the vector + the size of `Len` if successful
    ///
    /// Note: This will read `Len` * `T` bytes, not `Len` bytes of `T`!
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the writer.
    #[inline]
    fn write_len_type_at<'de, 'a, L, T>(
        &mut self,
        position: &mut u64,
        tys: impl ExactSizeIterator<Item = T::Input>,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
        <L as BinaryDeserialize<'de>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
        T: BinarySerializeExt,
        T::Ctx: Default,
    {
        L::write_len_at(self, position, tys.len())?;
        for ty in tys {
            T::serialize_at(ty, self, position)?;
        }
        Ok(())
    }

    /// Write the length and the vector of `T` at `position`
    ///
    /// It will first write the length of the vector as a [`Len`].
    /// This function increments `position` with the size of the vector + the size of `Len` if successful
    ///
    /// Note: This will read `Len` * `T` bytes, not `Len` bytes of `T`!
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the writer.
    #[inline]
    fn write_len_type_at_with_ctx<'de, 'a, L, T>(
        &mut self,
        position: &mut u64,
        tys: impl ExactSizeIterator<Item = T::Input>,
        ctx: T::Ctx,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
        <L as BinaryDeserialize<'de>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
        T: BinarySerialize,
        T::Ctx: Copy,
    {
        L::write_len_at(self, position, tys.len())?;
        for ty in tys {
            T::serialize_at_with_ctx(ty, self, position, ctx)?;
        }
        Ok(())
    }

    /// Write a `&str` from `source` at `position`
    ///
    /// It will write the string and then a null byte.
    /// This function increments `position` with the size of the string + 1 if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    fn write_null_terminated_string_at(
        &mut self,
        position: &mut u64,
        string: &str,
    ) -> Result<(), WriteError> {
        let slice = string.as_bytes();
        self.write_slice_at(position, slice)?;
        self.write_at::<u8>(position, 0u8)?;
        Ok(())
    }
}
