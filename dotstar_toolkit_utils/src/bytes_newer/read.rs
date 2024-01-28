use std::{backtrace::Backtrace, borrow::Cow, marker::PhantomData, str::Utf8Error, sync::Arc};

use byteorder::ByteOrder;
use thiserror::Error;
use ux::u24;

use super::Len;
use crate::testing::TestError;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum ReadError {
    /// ReadError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// Encountered invalid UTF-8 when trying to read a string from source
    #[error("invalid UTF-8 encountered, attempted to read a string of length {n} at {position}")]
    InvalidUTF8 {
        /// Position in the source
        position: u64,
        /// Amount of bytes that were needed
        n: usize,
        /// Original UTF-8 error
        error: Utf8Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("no null-byte for null terminated string, attempted to read a string at {position}")]
    /// Encountered no null byte when trying to read a null-terminated string
    NoNullByte {
        /// Position in the source
        position: u64,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Encountered an I/O error while trying to read from the source
    #[error("IoError occured while trying to read from the source at {position}: {error}")]
    IoError {
        /// Position in the source
        position: u64,
        /// The error
        error: std::io::Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// A read value did not match the expected value
    #[error("read a unexpected value: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// A custom error
    #[error("{string}")]
    Custom {
        /// The error description
        string: String,
        /// Backtrace
        backtrace: Backtrace,
    },
}

impl ReadError {
    /// Create the [`ReadError::InvalidUTF8`] error
    #[must_use]
    pub fn invalid_utf8(n: usize, position: u64, error: Utf8Error) -> Self {
        Self::InvalidUTF8 {
            n,
            position,
            error,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::NoNullByte`] error
    #[must_use]
    pub fn no_null_byte(position: u64) -> Self {
        Self::NoNullByte {
            position,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::IoError`] error
    #[must_use]
    pub fn io_error(position: u64, error: std::io::Error) -> Self {
        Self::IoError {
            position,
            error,
            backtrace: Backtrace::capture(),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn context<C: std::fmt::Debug>(self, context: C) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{context:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_context<C: std::fmt::Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{:?}", f()),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn custom(string: String) -> Self {
        Self::Custom {
            string,
            backtrace: Backtrace::capture(),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_custom<F: FnOnce() -> String>(f: F) -> Self {
        Self::Custom {
            string: f(),
            backtrace: Backtrace::capture(),
        }
    }
}

/// Represents a object that can be deserialized from a binary file
pub trait BinaryDeserialize<'de>: Sized {
    /// Deserialize the object from the reader
    fn deserialize<B>(reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        Self::deserialize_at::<B>(reader, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Implementation note: Must restore position to the original value on error!
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder;

    /// Lazily read `n` * `Self` at `position`
    #[inline(always)]
    fn read_n_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: u64,
        n: usize,
    ) -> impl Iterator<Item = Result<Self, ReadError>>
    where
        B: ByteOrder,
    {
        let iter: LenTypeIterator<'de, B, Self> = LenTypeIterator {
            remaining: n,
            position,
            _byteorder: PhantomData,
            _type: PhantomData,
            reader: reader.clone(),
        };
        iter
    }

    /// Lazily read `Len` * `Self` at `position`
    ///
    /// It will first read [`Len`] with byteorder `B` and then produce
    /// an iterator that produces [`Len`] times `Self`.
    #[inline(always)]
    fn read_n_len_at<B, L>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<impl Iterator<Item = Result<Self, ReadError>>, ReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(reader, position)?;
            Self::read_n_at::<B>(reader, *position, len)
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

pub struct LenTypeIterator<'de, B, T>
where
    B: ByteOrder,
    T: BinaryDeserialize<'de>,
{
    remaining: usize,
    position: u64,
    _byteorder: PhantomData<B>,
    _type: PhantomData<T>,
    reader: Arc<dyn ZeroCopyReadAt<'de> + 'de>,
}

impl<'de, B, T> LenTypeIterator<'de, B, T>
where
    B: ByteOrder,
    T: BinaryDeserialize<'de>,
{
    pub fn current_position(&self) -> u64 {
        self.position
    }
}

impl<'de, B, T> Iterator for LenTypeIterator<'de, B, T>
where
    B: ByteOrder,
    T: BinaryDeserialize<'de>,
{
    type Item = Result<T, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at::<B>(&self.reader, &mut self.position);
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

impl<'de> BinaryDeserialize<'de> for u8 {
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        let slice = reader.read_slice_at(position, 1)?;
        Ok(slice[0])
    }
}

impl<'de> BinaryDeserialize<'de> for u16 {
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        let slice = reader.read_slice_at(position, 2)?;
        Ok(B::read_u16(slice.as_ref()))
    }
}

impl<'de> BinaryDeserialize<'de> for u24 {
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        let slice = reader.read_slice_at(position, 3)?;
        let temp = B::read_u24(slice.as_ref());
        Ok(Self::new(temp))
    }
}

impl<'de> BinaryDeserialize<'de> for u32 {
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        let slice = reader.read_slice_at(position, 4)?;
        Ok(B::read_u32(slice.as_ref()))
    }
}

impl<'de> BinaryDeserialize<'de> for u64 {
    fn deserialize_at<B>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>
    where
        B: ByteOrder,
    {
        let slice = reader.read_slice_at(position, 8)?;
        Ok(B::read_u64(slice.as_ref()))
    }
}

pub trait ZeroCopyReadAt<'de> {
    /// Read a `&str` from `source` at `position`
    ///
    /// It will read until it finds a null byte, excluding it from the string.
    /// This function increments `position` with the size of the string + 1 if successful
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, ReadError>;

    /// Read a `&[u8]` of length `len` at `position`
    ///
    /// This function increments `position` with `len` if successful
    fn read_slice_at(&self, position: &mut u64, len: usize) -> Result<Cow<'de, [u8]>, ReadError>;

    /// Read a `&str` of length `len` at `position`
    ///
    /// This function increments `position` with `len` if successful
    fn read_string_at(&self, position: &mut u64, len: usize) -> Result<Cow<'de, str>, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            match self.read_slice_at(position, len)? {
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

    /// Read a byte slice at `position`
    ///
    /// It will first read the length of the byte slice as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_slice_at<B, L>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Cow<'de, [u8]>, ReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
        Self: Sized,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(reader, position)?;
            reader.read_slice_at(position, len)?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Read a `&[u8: N]` at `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    #[inline(always)]
    fn read_fixed_slice_at<const N: usize>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<[u8; N], ReadError>
    where
        Self: Sized,
    {
        let slice: Cow<'_, [u8]> = reader.read_slice_at(position, N)?;

        let fixed_slice: [u8; N] =
            TryFrom::try_from(slice.as_ref()).unwrap_or_else(|_| unreachable!());
        Ok(fixed_slice)
    }

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_string_at<B, L>(
        reader: &Arc<dyn ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, ReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
        Self: Sized,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(reader, position)?;
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
}

pub trait ZeroCopyReadAtExt<'de>: ZeroCopyReadAt<'de> {}

/// All types that implement `ZeroCopyReadAt` get methods defined in `ZeroCopyReadAtExt`
/// for free.
impl<'de, Z: ZeroCopyReadAt<'de> + ?Sized> ZeroCopyReadAtExt<'de> for Z {}
