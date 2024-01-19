//! Contains the new byte reading traits

use std::{backtrace::Backtrace, borrow::Cow, fs::File, str::Utf8Error};

use positioned_io::ReadAt;
use thiserror::Error;
use ux::u24;

use super::ByteOrder;
use super::Len;
use crate::testing::TestError;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum NewReadError {
    /// ReadError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// Trying to read outside source
    #[error("source is not large enough, attempted to read {n} bytes at {position} but source is only {size} bytes")]
    SourceTooSmall {
        /// Amount of bytes that were needed
        n: u64,
        /// Position in the source
        position: u64,
        /// The total size of the source
        size: u64,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Encountered invalid UTF-8 when trying to read a string from source
    #[error("invalid UTF-8 encountered, attempted to read a string of length {n} at {position}")]
    InvalidUTF8 {
        /// Amount of bytes that were needed
        n: u64,
        /// Position in the source
        position: u64,
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
    #[error("attempted to increment position {position} by {n}, but that would overflow")]
    /// Increasing the position would overflow the number
    PositionOverflow {
        /// Position in the source
        position: u64,
        /// How much the increment would be
        n: u64,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("attempted to read more bytes than can be pointed to")]
    /// Attempted to read more bytes than can be pointed to
    TooManyBytes {
        /// Position in the source
        position: u64,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// A read value did not match the expected value
    #[error("read a unexpected value: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
    },
    /// A custom error
    #[error("{string}")]
    Custom {
        /// The error description
        string: String,
    }
}

impl NewReadError {
    /// Create the [`ReadError::SourceTooSmall`] error
    #[must_use]
    pub fn source_too_small(n: u64, position: u64, size: u64) -> Self {
        Self::SourceTooSmall {
            n,
            position,
            size,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::InvalidUTF8`] error
    #[must_use]
    pub fn invalid_utf8(n: u64, position: u64, error: Utf8Error) -> Self {
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

    /// Create the [`ReadError::PositionOverflow`] error
    #[must_use]
    pub fn position_overflow(position: u64, n: u64) -> Self {
        Self::PositionOverflow {
            position,
            n,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::TooManyBytes`] error
    #[must_use]
    pub fn too_many_bytes(position: u64) -> Self {
        Self::TooManyBytes {
            position,
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
            string
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_custom<F: FnOnce() -> String>(f: F) -> Self {
        Self::Custom {
            string: f(),
        }
    }
}

/// Represents a object that can be deserialized from a binary file
pub trait BinaryDeserialize<'de>: Sized {
    /// Deserialize the object from the reader
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn deserialize<B>(reader: &(impl ZeroCopyReadAt<'de> + ?Sized)) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        Self::deserialize_at::<B>(reader, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder;
}

impl<'de> BinaryDeserialize<'de> for u8 {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let slice: [Self; 1] = reader.read_fixed_slice_at(position)?;
        Ok(slice[0])
    }
}

impl<'de> BinaryDeserialize<'de> for u16 {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let slice: [u8; 2] = reader.read_fixed_slice_at(position)?;
        Ok(B::read_u16(slice.as_ref()))
    }
}

impl<'de> BinaryDeserialize<'de> for u24 {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let slice: [u8; 3] = reader.read_fixed_slice_at(position)?;
        let temp = B::read_u24(slice.as_ref());
        Ok(Self::new(temp))
    }
}

impl<'de> BinaryDeserialize<'de> for u32 {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let slice: [u8; 4] = reader.read_fixed_slice_at(position)?;
        Ok(B::read_u32(slice.as_ref()))
    }
}

impl<'de> BinaryDeserialize<'de> for u64 {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'de> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let slice: [u8; 8] = reader.read_fixed_slice_at(position)?;
        Ok(B::read_u64(slice.as_ref()))
    }
}

/// Represents a byte source which uses Cow's to stay zerocopy
pub trait ZeroCopyReadAt<'de> {
    /// Read a `T` at `position`
    ///
    /// This function increments `position` with what `T` reads if successful
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn read_at<B, T>(&self, position: &mut u64) -> Result<T, NewReadError>
    where
        B: ByteOrder,
        T: BinaryDeserialize<'de>,
    {
        T::deserialize_at::<B>(self, position)
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
    ) -> Result<[u8; N], NewReadError> {
        let slice: Cow<'_, [u8]> = self.read_slice_at(
            position,
            u64::try_from(N).map_err(|_| NewReadError::too_many_bytes(*position))?,
        )?;

        let fixed_slice: [u8; N] =
            TryFrom::try_from(slice.as_ref()).unwrap_or_else(|_| unreachable!());
        Ok(fixed_slice)
    }

    /// Read a `&[u8]` of length `len` at `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    fn read_slice_at(&self, position: &mut u64, len: u64) -> Result<Cow<'de, [u8]>, NewReadError>;

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_string_at<B, L>(&self, position: &mut u64) -> Result<Cow<'de, str>, NewReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(self, position)?;
            match self.read_slice_at(position, len)? {
                Cow::Borrowed(slice) => std::str::from_utf8(slice)
                    .map(Cow::Borrowed)
                    .map_err(|e| NewReadError::invalid_utf8(len, *position, e))?,
                Cow::Owned(vec) => String::from_utf8(vec)
                    .map(Cow::Owned)
                    .map_err(|e| NewReadError::invalid_utf8(len, *position, e.utf8_error()))?,
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
    fn read_len_slice_at<B, L>(&self, position: &mut u64) -> Result<Cow<'de, [u8]>, NewReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(self, position)?;
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
    ///
    /// Note: This will read `Len` * `T` bytes, not `Len` bytes of `T`!
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn read_len_type_at<B, L, T>(&self, position: &mut u64) -> Result<Vec<T>, NewReadError>
    where
        B: ByteOrder,
        L: Len<'de>,
        T: BinaryDeserialize<'de>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at::<B>(self, position)?;
            let capacity =
                usize::try_from(len).map_err(|_| NewReadError::too_many_bytes(old_position))?;
            let mut buf = Vec::with_capacity(capacity);
            for _ in 0..len {
                buf.push(self.read_at::<B, T>(position)?);
            }
            buf
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Read a `&str` from `source` at `position`
    ///
    /// It will read until it finds a null byte, excluding it from the string.
    /// This function increments `position` with the size of the string + 1 if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, NewReadError>;
}

impl<'de> ZeroCopyReadAt<'de> for &'de [u8] {
    #[inline(always)]
    fn read_fixed_slice_at<const N: usize>(
        &self,
        position: &mut u64,
    ) -> Result<[u8; N], NewReadError> {
        let len = u64::try_from(N).map_err(|_| NewReadError::too_many_bytes(*position))?;
        let new_position = position
            .checked_add(len)
            .ok_or_else(|| NewReadError::position_overflow(*position, len))?;
        let new_position_usize =
            usize::try_from(new_position).map_err(|_| NewReadError::too_many_bytes(*position))?;
        let position_usize =
            usize::try_from(*position).map_err(|_| NewReadError::too_many_bytes(*position))?;
        if self.len() < (new_position_usize) {
            Err(NewReadError::source_too_small(
                len,
                *position,
                u64::try_from(self.len()).map_err(|_| NewReadError::too_many_bytes(*position))?,
            ))
        } else {
            *position = new_position;
            Ok(
                TryInto::<[u8; N]>::try_into(&self[position_usize..new_position_usize])
                    .unwrap_or_else(|_| unreachable!()),
            )
        }
    }

    #[inline(always)]
    fn read_slice_at(&self, position: &mut u64, len: u64) -> Result<Cow<'de, [u8]>, NewReadError> {
        let new_position = position
            .checked_add(len)
            .ok_or_else(|| NewReadError::position_overflow(*position, len))?;
        let new_position_usize =
            usize::try_from(new_position).map_err(|_| NewReadError::too_many_bytes(*position))?;
        let position_usize =
            usize::try_from(*position).map_err(|_| NewReadError::too_many_bytes(*position))?;
        if self.len() < (new_position_usize) {
            Err(NewReadError::source_too_small(
                len,
                *position,
                u64::try_from(self.len()).map_err(|_| NewReadError::too_many_bytes(*position))?,
            ))
        } else {
            *position = new_position;
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    #[inline(always)]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, NewReadError> {
        let position_usize =
            usize::try_from(*position).map_err(|_| NewReadError::too_many_bytes(*position))?;
        // Find the null byte, starting at `position_usize`
        let null_pos = self.iter().skip(position_usize).position(|b| b == &0);
        if let Some(null_pos) = null_pos {
            let null_pos_u64 =
                u64::try_from(null_pos).map_err(|_| NewReadError::too_many_bytes(*position))?;
            match std::str::from_utf8(&self[position_usize..null_pos]) {
                Ok(str) => {
                    *position = null_pos_u64
                        .checked_add(1)
                        .unwrap_or_else(|| unreachable!());
                    Ok(Cow::Borrowed(str))
                }
                Err(error) => Err(NewReadError::invalid_utf8(
                    null_pos_u64
                        .checked_sub(*position)
                        .unwrap_or_else(|| unreachable!()),
                    *position,
                    error,
                )),
            }
        } else {
            Err(NewReadError::no_null_byte(*position))
        }
    }
}

impl<'de> ZeroCopyReadAt<'de> for File {
    #[inline(always)]
    fn read_slice_at(&self, position: &mut u64, len: u64) -> Result<Cow<'de, [u8]>, NewReadError> {
        let len_usize =
            usize::try_from(len).map_err(|_| NewReadError::too_many_bytes(*position))?;
        let new_position = position
            .checked_add(len)
            .ok_or_else(|| NewReadError::position_overflow(*position, len))?;
        let mut buf = vec![0; len_usize];
        self.read_exact_at(*position, &mut buf)
            .map_err(|e| NewReadError::io_error(*position, e))?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

    #[inline(always)]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, NewReadError> {
        // Buffer used to read parts from the file
        let mut read_buf = vec![0; 0x10];
        // Buffer that stores the resulting string
        let mut result_buf = Vec::new();
        // Keep track of search position here, so that the original position is not affected
        let mut new_position = *position;
        loop {
            let bytes_read = ReadAt::read_at(self, new_position, &mut read_buf)
                .map_err(|e| NewReadError::io_error(*position, e))?;
            let bytes_read = u64::try_from(bytes_read).unwrap_or_else(|_| unreachable!());
            if bytes_read == 0 {
                // End of file reached, give up
                return Err(NewReadError::no_null_byte(*position));
            }
            if let Some(found) = read_buf.iter().position(|b| *b == 0x0) {
                // Found null byte, add everything upto the null byte in `result_buf`
                result_buf.extend_from_slice(&read_buf[0..found]);
                let found = u64::try_from(found).unwrap_or_else(|_| unreachable!());
                let end_position = new_position
                    .checked_add(found)
                    .ok_or_else(|| NewReadError::position_overflow(new_position, found))?;
                let string = String::from_utf8(result_buf).map_err(|error| {
                    NewReadError::invalid_utf8(
                        end_position
                            .checked_sub(*position)
                            .unwrap_or_else(|| unreachable!()),
                        *position,
                        error.utf8_error(),
                    )
                })?;
                // Set position past the null byte
                *position = end_position
                    .checked_add(1)
                    .ok_or_else(|| NewReadError::position_overflow(end_position, 1))?;
                return Ok(Cow::Owned(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position
                .checked_add(bytes_read)
                .ok_or_else(|| NewReadError::position_overflow(new_position, bytes_read))?;
        }
    }
}
