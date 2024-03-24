use std::{
    backtrace::Backtrace, borrow::Cow, fs::File, marker::PhantomData, num::TryFromIntError,
    ops::Deref, rc::Rc, str::Utf8Error, sync::Arc,
};

use positioned_io::{RandomAccessFile, ReadAt as PRead};
use thiserror::Error;

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
    /// An integer could not be converted to another integer size
    #[error("an integer could not be converted to another integer size: {tfie:?}")]
    IntConversion {
        /// The original test error
        #[from]
        tfie: TryFromIntError,
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
    fn deserialize(reader: &'de impl ZeroCopyReadAtExt) -> Result<Self, ReadError> {
        Self::deserialize_at(reader, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Implementation note: Must restore position to the original value on error!
    fn deserialize_at(
        reader: &'de impl ZeroCopyReadAtExt,
        position: &mut u64,
    ) -> Result<Self, ReadError>;
}

impl<'de> BinaryDeserialize<'de> for u8 {
    fn deserialize_at(
        reader: &'de impl ZeroCopyReadAtExt,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        reader.read_fixed_slice_at::<1>(position).map(|s| s[0])
    }
}

pub trait ZeroCopyReadAt {
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

    /// Read a `&str` of length `len` at `position`
    ///
    /// This function increments `position` with `len` if successful
    fn read_string_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, str>, ReadError> {
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

impl ZeroCopyReadAt for File {
    #[inline(always)]
    fn read_slice_at(
        &self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_u64 = u64::try_from(len).unwrap();
        let new_position = position.checked_add(len_u64).ok_or_else(|| {
            ReadError::custom(format!(
                "Tried to add {len_u64} to {position} and overflowed"
            ))
        })?;
        let mut buf = vec![0; len];
        PRead::read_exact_at(self, *position, &mut buf)
            .map_err(|e| ReadError::io_error(*position, e))?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

    #[inline(always)]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'static, str>, ReadError> {
        // Buffer used to read parts from the file
        let mut read_buf = vec![0; 0x10];
        // Buffer that stores the resulting string
        let mut result_buf = Vec::new();
        // Keep track of search position here, so that the original position is not affected
        let mut new_position = *position;
        loop {
            let bytes_read = PRead::read_at(self, new_position, &mut read_buf)
                .map_err(|e| ReadError::io_error(*position, e))?;
            let bytes_read = u64::try_from(bytes_read).unwrap_or_else(|_| unreachable!());
            if bytes_read == 0 {
                // End of file reached, give up
                return Err(ReadError::no_null_byte(*position));
            }
            if let Some(found) = read_buf.iter().position(|b| *b == 0x0) {
                // Found null byte, add everything upto the null byte in `result_buf`
                result_buf.extend_from_slice(&read_buf[0..found]);
                let found = u64::try_from(found).unwrap_or_else(|_| unreachable!());
                let end_position = new_position.checked_add(found).ok_or_else(|| {
                    ReadError::custom(format!("Tried to add {found} to {position} and overflowed"))
                })?;
                let string = String::from_utf8(result_buf).map_err(|error| {
                    ReadError::invalid_utf8(
                        usize::try_from(
                            end_position
                                .checked_sub(*position)
                                .unwrap_or_else(|| unreachable!()),
                        )
                        .unwrap_or_else(|_| unreachable!()),
                        *position,
                        error.utf8_error(),
                    )
                })?;
                // Set position past the null byte
                *position = end_position.checked_add(1).ok_or_else(|| {
                    ReadError::custom(format!("Tried to add 1 to {end_position} and overflowed"))
                })?;
                return Ok(Cow::Owned(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position.checked_add(bytes_read).ok_or_else(|| {
                ReadError::custom(format!(
                    "Tried to add {bytes_read} to {new_position} and overflowed"
                ))
            })?;
        }
    }
}

impl ZeroCopyReadAt for RandomAccessFile {
    #[inline(always)]
    fn read_slice_at(
        &self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_u64 = u64::try_from(len).unwrap();
        let new_position = position.checked_add(len_u64).ok_or_else(|| {
            ReadError::custom(format!(
                "Tried to add {len_u64} to {position} and overflowed"
            ))
        })?;
        let mut buf = vec![0; len];
        PRead::read_exact_at(self, *position, &mut buf)
            .map_err(|e| ReadError::io_error(*position, e))?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

    #[inline(always)]
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'static, str>, ReadError> {
        // Buffer used to read parts from the file
        let mut read_buf = vec![0; 0x10];
        // Buffer that stores the resulting string
        let mut result_buf = Vec::new();
        // Keep track of search position here, so that the original position is not affected
        let mut new_position = *position;
        loop {
            let bytes_read = PRead::read_at(self, new_position, &mut read_buf)
                .map_err(|e| ReadError::io_error(*position, e))?;
            let bytes_read = u64::try_from(bytes_read).unwrap_or_else(|_| unreachable!());
            if bytes_read == 0 {
                // End of file reached, give up
                return Err(ReadError::no_null_byte(*position));
            }
            if let Some(found) = read_buf.iter().position(|b| *b == 0x0) {
                // Found null byte, add everything upto the null byte in `result_buf`
                result_buf.extend_from_slice(&read_buf[0..found]);
                let found = u64::try_from(found).unwrap_or_else(|_| unreachable!());
                let end_position = new_position.checked_add(found).ok_or_else(|| {
                    ReadError::custom(format!("Tried to add {found} to {position} and overflowed"))
                })?;
                let string = String::from_utf8(result_buf).map_err(|error| {
                    ReadError::invalid_utf8(
                        usize::try_from(
                            end_position
                                .checked_sub(*position)
                                .unwrap_or_else(|| unreachable!()),
                        )
                        .unwrap_or_else(|_| unreachable!()),
                        *position,
                        error.utf8_error(),
                    )
                })?;
                // Set position past the null byte
                *position = end_position.checked_add(1).ok_or_else(|| {
                    ReadError::custom(format!("Tried to add 1 to {end_position} and overflowed"))
                })?;
                return Ok(Cow::Owned(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position.checked_add(bytes_read).ok_or_else(|| {
                ReadError::custom(format!(
                    "Tried to add {bytes_read} to {new_position} and overflowed"
                ))
            })?;
        }
    }
}

impl ZeroCopyReadAt for [u8] {
    #[inline(always)]
    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        let new_position = position.checked_add(len as u64).unwrap();
        let new_position_usize = usize::try_from(new_position).unwrap();
        let position_usize = usize::try_from(*position).unwrap();
        if self.len() < (new_position_usize) {
            todo!()
        } else {
            *position = new_position;
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    #[inline(always)]
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        let position_usize = usize::try_from(*position).unwrap();
        // Find the null byte, starting at `position_usize`
        let null_pos = self.iter().skip(position_usize).position(|b| b == &0);
        if let Some(null_pos) = null_pos {
            let null_pos_u64 = u64::try_from(null_pos).unwrap();
            match std::str::from_utf8(&self[position_usize..null_pos]) {
                Ok(str) => {
                    *position = null_pos_u64
                        .checked_add(1)
                        .unwrap_or_else(|| unreachable!());
                    Ok(Cow::Borrowed(str))
                }
                Err(_error) => todo!(),
            }
        } else {
            Err(ReadError::no_null_byte(*position))
        }
    }
}

impl ZeroCopyReadAt for Vec<u8> {
    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        let new_position = position.checked_add(len as u64).unwrap();
        let new_position_usize = usize::try_from(new_position).unwrap();
        let position_usize = usize::try_from(*position).unwrap();
        if self.len() < (new_position_usize) {
            todo!()
        } else {
            *position = new_position;
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        let position_usize = usize::try_from(*position).unwrap();
        // Find the null byte, starting at `position_usize`
        let null_pos = self.iter().skip(position_usize).position(|b| b == &0);
        if let Some(null_pos) = null_pos {
            let null_pos_u64 = u64::try_from(null_pos).unwrap();
            match std::str::from_utf8(&self[position_usize..null_pos]) {
                Ok(str) => {
                    *position = null_pos_u64
                        .checked_add(1)
                        .unwrap_or_else(|| unreachable!());
                    Ok(Cow::Borrowed(str))
                }
                Err(_error) => todo!(),
            }
        } else {
            Err(ReadError::no_null_byte(*position))
        }
    }
}

/// Mark this type as trivial to clone.
///
/// What is trivial?
/// Trivial is relative, but [`Arc`] is considered trivial while [`Vec`] is not.
pub trait TrivialClone: Clone {}

impl<T> TrivialClone for Arc<T> {}
impl<T> TrivialClone for Rc<T> {}
impl<T: ZeroCopyReadAt> ZeroCopyReadAt for Arc<T> {
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
}
impl<T: ZeroCopyReadAt> ZeroCopyReadAt for Rc<T> {
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
}

pub trait ZeroCopyReadAtExt: ZeroCopyReadAt + TrivialClone {
    /// Read a `T` at `position`
    ///
    /// This function increments `position` with what `T` reads if successful
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn read_at<'rf, T>(&'rf self, position: &mut u64) -> Result<T, ReadError>
    where
        T: BinaryDeserialize<'rf>,
    {
        T::deserialize_at(self, position)
    }

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`]
    /// This function increments `position` with the size of the string + the size of `Len` if successful
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
                Cow::Borrowed(slice) => std::str::from_utf8(slice).map(Cow::Borrowed).unwrap(),
                Cow::Owned(vec) => String::from_utf8(vec).map(Cow::Owned).unwrap(),
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
        T: BinaryDeserialize<'rf>,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = L::read_len_at(self, position)?;
            LenTypeIterator {
                remaining: len,
                position: *position,
                _type: PhantomData,
                reader: self,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl<T> ZeroCopyReadAtExt for T where T: ZeroCopyReadAt + TrivialClone {}

pub struct LenTypeIterator<'rf, T, R: ZeroCopyReadAtExt>
where
    T: BinaryDeserialize<'rf>,
{
    remaining: usize,
    position: u64,
    _type: PhantomData<T>,
    reader: &'rf R,
}

impl<'rf, T, R: ZeroCopyReadAtExt> LenTypeIterator<'rf, T, R>
where
    T: BinaryDeserialize<'rf>,
{
    pub fn current_position(&self) -> u64 {
        self.position
    }
}

impl<'rf, T, R: ZeroCopyReadAtExt> Iterator for LenTypeIterator<'rf, T, R>
where
    T: BinaryDeserialize<'rf>,
{
    type Item = Result<T, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let res = T::deserialize_at(self.reader, &mut self.position);
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
