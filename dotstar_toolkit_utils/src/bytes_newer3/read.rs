use std::{backtrace::Backtrace, borrow::Cow, marker::PhantomData, str::Utf8Error, sync::Arc};

use thiserror::Error;

use super::{primitives::Endianness, Len};
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
    type Endian: Endianness;

    /// Deserialize the object from the reader
    fn deserialize(reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>) -> Result<Self, ReadError> {
        Self::deserialize_at(reader, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Implementation note: Must restore position to the original value on error!
    fn deserialize_at(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError>;

    /// Lazily read `n` * `Self` at `position`
    #[inline(always)]
    fn read_n_at(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: u64,
        n: usize,
    ) -> impl Iterator<Item = Result<Self, ReadError>> {
        LenTypeIterator {
            remaining: n,
            position,
            _type: PhantomData,
            _life: PhantomData,
            reader: reader.clone(),
        }
    }

    /// Lazily read `Len` * `Self` at `position`
    ///
    /// It will first read [`Len`] with byteorder `B` and then produce
    /// an iterator that produces [`Len`] times `Self`.
    #[inline(always)]
    fn read_len_n_at<L>(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<impl Iterator<Item = Result<Self, ReadError>>, ReadError>
    where
        L: Len<'de>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(reader, position)?;
            Self::read_n_at(reader, *position, len)
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

pub struct LenTypeIterator<'de, T, R: ZeroCopyReadAt<'de> + 'de>
where
    T: BinaryDeserialize<'de>,
{
    remaining: usize,
    position: u64,
    _type: PhantomData<T>,
    _life: PhantomData<&'de bool>,
    reader: Arc<R>,
}

impl<'de, T, R: ZeroCopyReadAt<'de>> LenTypeIterator<'de, T, R>
where
    T: BinaryDeserialize<'de>,
{
    pub fn current_position(&self) -> u64 {
        self.position
    }
}

impl<'de, T, R: ZeroCopyReadAt<'de>> Iterator for LenTypeIterator<'de, T, R>
where
    T: BinaryDeserialize<'de>,
{
    type Item = Result<T, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at(&self.reader, &mut self.position);
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

    /// Read a `&[u8: N]` at `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    #[inline(always)]
    fn read_fixed_slice_at<const N: usize>(
        &self,
        position: &mut u64,
    ) -> Result<[u8; N], ReadError> {
        let slice: Cow<'_, [u8]> = self.read_slice_at(position, N)?;

        let fixed_slice: [u8; N] =
            TryFrom::try_from(slice.as_ref()).unwrap_or_else(|_| unreachable!());
        Ok(fixed_slice)
    }
}
