//! Types and traits for reading data from byte sources

use std::{borrow::Cow, marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

mod error;
mod impls;

pub use error::*;

use super::len::Len;

/// Represents a object that can be deserialized from a binary file
pub trait BinaryDeserialize<'de, Ctx: ?Sized = ()>
where
    Self: Sized,
{
    /// Deserialize the object from the start of the reader with `ctx`
    #[inline(always)]
    fn deserialize_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        ctx: &Ctx,
    ) -> Result<Self, ReadError> {
        Self::deserialize_at_with_ctx(reader, &mut 0, ctx)
    }

    /// Deserialize the object from the reader at `position` with `ctx`
    ///
    /// Implementation note: Must restore position to the original value on error!
    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: &Ctx,
    ) -> Result<Self, ReadError>;
}

/// Wraps `BinaryDeserialize` to remove the `ctx` parameter when it can be `Default`ed
pub trait BinaryDeserializeExt<'de, Ctx: Default = ()>: BinaryDeserialize<'de, Ctx> {
    /// Deserialize the object from start of the reader
    #[inline(always)]
    fn deserialize(reader: &'de (impl ReadAtExt + ?Sized)) -> Result<Self, ReadError> {
        Self::deserialize_with_ctx(reader, &Ctx::default())
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Implementation note: Must restore position to the original value on error!
    #[inline(always)]
    fn deserialize_at(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        Self::deserialize_at_with_ctx(reader, position, &Ctx::default())
    }
}

impl<'de, Ctx: Default, T: BinaryDeserialize<'de, Ctx>> BinaryDeserializeExt<'de, Ctx> for T {}

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
    fn read_at<'rf, T, Ctx>(&'rf self, position: &mut u64) -> Result<T, ReadError>
    where
        Ctx: Default,
        T: BinaryDeserializeExt<'rf, Ctx>,
    {
        T::deserialize_at(self, position)
    }
    /// Read a `T` at `position` with `ctx`
    ///
    /// `position` will be incremented with what `T` reads if successful otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn read_at_with_ctx<'rf, T, Ctx>(
        &'rf self,
        position: &mut u64,
        ctx: &Ctx,
    ) -> Result<T, ReadError>
    where
        Ctx: ?Sized,
        T: BinaryDeserialize<'rf, Ctx>,
    {
        T::deserialize_at_with_ctx(self, position, ctx)
    }

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`]
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_string_at<'rf, L>(&'rf self, position: &mut u64) -> Result<Cow<'rf, str>, ReadError>
    where
        L: Len<'rf>,
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

    /// Read a byte slice at `position`
    ///
    /// It will first read the length of the byte slice as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    /// otherwise will remain the same.
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_slice_at<'rf, L>(&'rf self, position: &mut u64) -> Result<Cow<'rf, [u8]>, ReadError>
    where
        L: Len<'rf>,
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
    #[inline(always)]
    fn read_len_type_at<'rf, L, T>(
        &'rf self,
        position: &mut u64,
    ) -> Result<impl Iterator<Item = Result<T, ReadError>>, ReadError>
    where
        L: Len<'rf>,
        T: BinaryDeserializeExt<'rf>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            LenTypeIterator {
                remaining: len,
                position: *position,
                _type: PhantomData::<T>,
                reader: self,
            }
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
    #[inline(always)]
    fn read_len_type_at_with_ctx<'rf, 'ctx, L, T, Ctx>(
        &'rf self,
        position: &mut u64,
        ctx: &'ctx Ctx,
    ) -> Result<impl Iterator<Item = Result<T, ReadError>>, ReadError>
    where
        L: Len<'rf>,
        T: BinaryDeserialize<'rf, Ctx>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            LenTypeIteratorWithCtx {
                remaining: len,
                position: *position,
                _type: PhantomData::<T>,
                reader: self,
                ctx,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl<T> ReadAtExt for T where T: ReadAt + ?Sized {}

/// Iterator that reads `T` from a source for `Len` times
pub struct LenTypeIteratorWithCtx<'rf, 'ctx, Ctx, T, R>
where
    Ctx: ?Sized,
    T: BinaryDeserialize<'rf, Ctx>,
    R: ReadAtExt + ?Sized,
{
    /// Remaining items to read from the iterator
    remaining: usize,
    /// The current position in the reader
    position: u64,
    /// The type that is being read
    _type: PhantomData<T>,
    /// The reader
    reader: &'rf R,
    /// The context for deserializing the type
    ctx: &'ctx Ctx,
}

impl<'rf, 'ctx, T, R, Ctx> LenTypeIteratorWithCtx<'rf, 'ctx, Ctx, T, R>
where
    Ctx: ?Sized,
    T: BinaryDeserialize<'rf, Ctx>,
    R: ReadAtExt + ?Sized,
{
    #[must_use]
    /// The current position of the iterator
    ///
    /// This value might change after calling `next`
    pub const fn current_position(&self) -> u64 {
        self.position
    }
}

impl<'rf, 'ctx, T, R, Ctx> Iterator for LenTypeIteratorWithCtx<'rf, 'ctx, Ctx, T, R>
where
    Ctx: ?Sized,
    T: BinaryDeserialize<'rf, Ctx>,
    R: ReadAtExt + ?Sized,
{
    type Item = Result<T, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at_with_ctx(self.reader, &mut self.position, self.ctx);
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

/// Iterator that reads `T` from a source for `Len` times
pub struct LenTypeIterator<'rf, T, R>
where
    T: BinaryDeserializeExt<'rf>,
    R: ReadAtExt + ?Sized,
{
    /// Remaining items to read from the iterator
    remaining: usize,
    /// The current position in the reader
    position: u64,
    /// The type that is being read
    _type: PhantomData<T>,
    /// The reader
    reader: &'rf R,
}

impl<'rf, T, R> LenTypeIterator<'rf, T, R>
where
    T: BinaryDeserializeExt<'rf>,
    R: ReadAtExt + ?Sized,
{
    #[must_use]
    /// The current position of the iterator
    ///
    /// This value might change after calling `next`
    pub const fn current_position(&self) -> u64 {
        self.position
    }
}

impl<'rf, T, R> Iterator for LenTypeIterator<'rf, T, R>
where
    T: BinaryDeserializeExt<'rf>,
    R: ReadAtExt + ?Sized,
{
    type Item = Result<T, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at(self.reader, &mut self.position);
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
