//! Types and traits for reading data from byte sources

use std::{borrow::Cow, marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

mod error;
mod impls;

pub use error::*;

use super::{len::Len, write::BinarySerialize};

/// Represents a object that can be deserialized from a binary file
pub trait BinaryDeserialize<'de> {
    type Ctx: Sized;
    type Output: Sized;

    /// Deserialize the object from the start of the reader with `ctx`
    #[inline]
    fn deserialize_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::deserialize_at_with(reader, &mut 0, ctx)
    }

    /// Deserialize the object from the reader at `position` with `ctx`
    ///
    /// Implementation note: Must restore position to the original value on error!
    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError>;
}

/// Wraps `BinaryDeserialize` to remove the `ctx` parameter when it can be `Default`ed
pub trait BinaryDeserializeExt<'de>: BinaryDeserialize<'de>
where
    Self::Ctx: Default,
{
    /// Deserialize the object from start of the reader
    #[inline]
    fn deserialize(reader: &'de (impl ReadAtExt + ?Sized)) -> Result<Self::Output, ReadError> {
        Self::deserialize_with(reader, Self::Ctx::default())
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Implementation note: Must restore position to the original value on error!
    #[inline]
    fn deserialize_at(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self::Output, ReadError> {
        Self::deserialize_at_with(reader, position, Self::Ctx::default())
    }
}

impl<'de, T: BinaryDeserialize<'de>> BinaryDeserializeExt<'de> for T where T::Ctx: Default {}

/// A byte source implementing
#[expect(
    clippy::len_without_is_empty,
    reason = "It's meant for files not containers"
)]
pub trait ReadAt {
    /// Read a `&str` from `source` at `position`
    ///
    /// It will read until it finds a null byte, excluding it from the string.
    /// This function increments `position` with the size of the string + 1 if successful
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError>;

    /// Read a `&[u8]` of length `len` at `position`
    ///
    /// This function increments `position` with `len` if successful
    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError>;

    /// Returns the size of the reader in bytes
    fn len(&self) -> Result<u64, ReadError>;
}

impl<T: ReadAt> ReadAt for Arc<T> {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        self.deref().read_null_terminated_string_at(position)
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        self.deref().read_slice_at(position, len)
    }

    fn len(&self) -> Result<u64, ReadError> {
        self.deref().len()
    }
}
impl<T: ReadAt> ReadAt for Rc<T> {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        self.deref().read_null_terminated_string_at(position)
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        self.deref().read_slice_at(position, len)
    }

    fn len(&self) -> Result<u64, ReadError> {
        self.deref().len()
    }
}

/// Utility functions for items that implement `ZeroCopyReadAt`
pub trait ReadAtExt: ReadAt {
    /// Read a `T` at `position`
    ///
    /// `position` will be incremented with what `T` reads if successful otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn read_at<'rf, T>(&'rf self, position: &mut u64) -> Result<T::Output, ReadError>
    where
        T::Ctx: Default,
        T: BinaryDeserializeExt<'rf>,
    {
        T::deserialize_at(self, position)
    }
    /// Read a `T` at `position` with `ctx`
    ///
    /// `position` will be incremented with what `T` reads if successful otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn read_at_with<'rf, T>(
        &'rf self,
        position: &mut u64,
        ctx: T::Ctx,
    ) -> Result<T::Output, ReadError>
    where
        T: BinaryDeserialize<'rf>,
    {
        T::deserialize_at_with(self, position, ctx)
    }

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`]
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline]
    fn read_len_string_at<'rf, L>(&'rf self, position: &mut u64) -> Result<Cow<'rf, str>, ReadError>
    where
        L: Len<'rf>,
        <L as BinaryDeserialize<'rf>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            match self.read_slice_at(position, len)? {
                Cow::Borrowed(slice) => std::str::from_utf8(slice)
                    .map(Cow::Borrowed)
                    .map_err(ReadError::from)?,
                Cow::Owned(vec) => String::from_utf8(vec)
                    .map(Cow::Owned)
                    .map_err(ReadError::from)?,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Read a lossy string at `position`
    ///
    /// This will replace any invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER, which looks like this: �.
    ///
    /// It will first read the length of the string as a [`Len`]
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline]
    fn read_len_string_lossy_at<'rf, L>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError>
    where
        L: Len<'rf>,
        <L as BinaryDeserialize<'rf>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            match self.read_slice_at(position, len)? {
                Cow::Borrowed(slice) => String::from_utf8_lossy(slice),
                Cow::Owned(vec) => String::from_utf8(vec)
                    .map(Cow::Owned)
                    .map_err(ReadError::from)?,
            }
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
    /// otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline]
    fn read_len_slice_at<'rf, L>(&'rf self, position: &mut u64) -> Result<Cow<'rf, [u8]>, ReadError>
    where
        L: Len<'rf>,
        <L as BinaryDeserialize<'rf>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            self.read_slice_at(position, len)?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Read a vector of `T` at `position`
    ///
    /// It will first read the length of the vector as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the vector + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// Note: This will read `Len` * `T` bytes, not `Len` bytes of `T`!
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline]
    fn read_len_type_at<'rf, 'pos, L, T>(
        &'rf self,
        position: &'pos mut u64,
    ) -> Result<impl Iterator<Item = Result<T::Output, ReadError>>, ReadError>
    where
        L: Len<'rf>,
        <L as BinaryDeserialize<'rf>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
        T::Ctx: Default + Copy,
        T: BinaryDeserializeExt<'rf>,
    {
        let old_position = *position;
        let len: Result<_, _> = try { L::read_len_at(self, position)? };
        match len {
            Ok(remaining) => Ok(LenTypeIteratorWithCtx {
                remaining,
                position,
                _type: PhantomData::<T>,
                reader: self,
                ctx: T::Ctx::default(),
            }),
            Err(error) => {
                *position = old_position;
                Err(error)
            }
        }
    }

    /// Read a vector of `T` at `position`
    ///
    /// It will first read the length of the vector as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the vector + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// Note: This will read `Len` * `T` bytes, not `Len` bytes of `T`!
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline]
    fn read_len_type_at_with<'rf, 'pos, L, T>(
        &'rf self,
        position: &'pos mut u64,
        ctx: T::Ctx,
    ) -> Result<impl Iterator<Item = Result<T::Output, ReadError>>, ReadError>
    where
        L: Len<'rf>,
        <L as BinaryDeserialize<'rf>>::Ctx: Default,
        <L as BinarySerialize>::Ctx: Default,
        L::Output: TryInto<usize>,
        L::Input: TryFrom<usize>,
        T: BinaryDeserialize<'rf>,
        T::Ctx: Copy,
    {
        let old_position = *position;
        let len: Result<_, _> = try { L::read_len_at(self, position)? };
        match len {
            Ok(remaining) => Ok(LenTypeIteratorWithCtx {
                remaining,
                position,
                _type: PhantomData::<T>,
                reader: self,
                ctx,
            }),
            Err(error) => {
                *position = old_position;
                Err(error)
            }
        }
    }
}

impl<T> ReadAtExt for T where T: ReadAt + ?Sized {}

/// Iterator that reads `T::Output` from a source for `Len` times
pub struct LenTypeIteratorWithCtx<'rf, 'pos, T, R>
where
    T: BinaryDeserialize<'rf>,
    R: ReadAtExt + ?Sized,
    T::Ctx: Copy,
{
    /// Remaining items to read from the iterator
    remaining: usize,
    /// The current position in the reader
    position: &'pos mut u64,
    /// The type that is being read
    _type: PhantomData<T>,
    /// The reader
    reader: &'rf R,
    /// The context for deserializing the type
    ctx: T::Ctx,
}

impl<'rf, T, R> LenTypeIteratorWithCtx<'rf, '_, T, R>
where
    T: BinaryDeserialize<'rf>,
    R: ReadAtExt + ?Sized,
    T::Ctx: Copy,
{
    #[must_use]
    /// The current position of the iterator
    ///
    /// This value might change after calling `next`
    pub const fn current_position(&self) -> u64 {
        *self.position
    }
}

impl<'rf, T, R> Iterator for LenTypeIteratorWithCtx<'rf, '_, T, R>
where
    T: BinaryDeserialize<'rf>,
    R: ReadAtExt + ?Sized,
    T::Ctx: Copy,
{
    type Item = Result<T::Output, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at_with(self.reader, self.position, self.ctx);
            #[allow(
                clippy::arithmetic_side_effects,
                reason = "It's checked that remaining is larger than 0"
            )]
            if res.is_ok() {
                self.remaining -= 1;
            }
            Some(res)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}
