use std::{borrow::Cow, fs::File, str::Utf8Error};

pub use byteorder::ByteOrder;
use positioned_io::ReadAt;
use thiserror::Error;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum ReadError {
    /// Trying to read outside source
    #[error("source is not large enough, attempted to read {n} bytes at {position} but source is only {size} bytes")]
    SourceTooSmall {
        /// Amount of bytes that were needed
        n: u64,
        /// Position in the source
        position: u64,
        /// The total size of the source
        size: u64,
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
    },
    #[error("no null-byte for null terminated string, attempted to read a string at {position}")]
    /// Encountered no null byte when trying to read a null-terminated string
    NoNullByte {
        /// Position in the source
        position: u64,
    },
    #[error("IoError occured while trying to read from the source at {position}: {error}")]
    IoError {
        /// Position in the source
        position: u64,
        /// The error
        error: std::io::Error,
    },
}

impl ReadError {
    /// Create the [`ReadError::SourceTooSmall`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn source_too_small(n: u64, position: u64, size: u64) -> Self {
        Self::SourceTooSmall { n, position, size }
    }

    /// Create the [`ReadError::InvalidUTF8`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn invalid_utf8(n: u64, position: u64, error: Utf8Error) -> Self {
        Self::InvalidUTF8 { n, position, error }
    }

    /// Create the [`ReadError::NoNullByte`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn no_null_byte(position: u64) -> Self {
        Self::NoNullByte { position }
    }

    /// Create the [`ReadError::IoError`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn io_error(position: u64, error: std::io::Error) -> Self {
        Self::IoError { position, error }
    }
}

pub trait ZeroCopyReadAt {
    /// Read a `u8` at position `position`
    ///
    /// This function increments `position` with 1 if successful
    ///
    /// # Errors
    /// This function will return an error when the u8 would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u8_at(&self, position: &mut u64) -> Result<u8, ReadError> {
        let slice: [u8; 1] = self.read_fixed_slice_at(position)?;
        Ok(slice[0])
    }

    /// Read a `u16` at position `position` with byteorder `T`
    ///
    /// This function increments `position` with 2 if successful
    ///
    /// # Errors
    /// This function will return an error when the u16 would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u16_at<T: ByteOrder>(&self, position: &mut u64) -> Result<u16, ReadError> {
        let slice: [u8; 2] = self.read_fixed_slice_at(position)?;
        let data = T::read_u16(slice.as_ref());
        Ok(data)
    }

    /// Read a `u24` at position `position` with byteorder `T`
    ///
    /// This function increments `position` with 3 if successful
    ///
    /// # Errors
    /// This function will return an error when the u24 would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u24_at<T: ByteOrder>(&self, position: &mut u64) -> Result<u32, ReadError> {
        let slice: [u8; 3] = self.read_fixed_slice_at(position)?;
        let data = T::read_u24(slice.as_ref());
        Ok(data)
    }

    /// Read a `u32` at position `position` with byteorder `T`
    ///
    /// This function increments `position` with 4 if successful
    ///
    /// # Errors
    /// This function will return an error when the u32 would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u32_at<T: ByteOrder>(&self, position: &mut u64) -> Result<u32, ReadError> {
        let slice: [u8; 4] = self.read_fixed_slice_at(position)?;
        let data = T::read_u32(slice.as_ref());
        Ok(data)
    }

    /// Read a `u64` at position `position` with byteorder `T`
    ///
    /// This function increments `position` with 8 if successful
    ///
    /// # Errors
    /// This function will return an error when the u64 would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u64_at<T: ByteOrder>(&self, position: &mut u64) -> Result<u64, ReadError> {
        let slice: [u8; 8] = self.read_fixed_slice_at(position)?;
        let data = T::read_u64(slice.as_ref());
        Ok(data)
    }

    /// Read a `&[u8: N]` at position `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_fixed_slice_at<const N: usize>(
        &self,
        position: &mut u64,
    ) -> Result<[u8; N], ReadError> {
        let slice: Cow<'_, [u8]> = self.read_slice_at(
            position,
            u64::try_from(N).unwrap_or_else(|_| unreachable!()),
        )?;

        let fixed_slice: [u8; N] =
            TryFrom::try_from(slice.as_ref()).unwrap_or_else(|_| unreachable!());
        Ok(fixed_slice)
    }

    /// Read a `&[u8]` of length `len` at position `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_slice_at<'a>(
        &'a self,
        position: &mut u64,
        len: u64,
    ) -> Result<Cow<'a, [u8]>, ReadError>;

    /// Read a `&str` at position `position`
    ///
    /// It will first read the length of the string as a `u32` with byteorder `T`
    /// This function increments `position` with the size of the string + 4 if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_u32_string_at<'a, T: ByteOrder>(
        &'a self,
        position: &mut u64,
    ) -> Result<Cow<'a, str>, ReadError> {
        let len = u64::from(self.read_u32_at::<T>(position)?);
        match self.read_slice_at(position, len) {
            Ok(Cow::Borrowed(slice)) => {
                match std::str::from_utf8(slice)
                    .map_err(|e| ReadError::invalid_utf8(len, *position, e))
                {
                    Ok(str) => Ok(Cow::Borrowed(str)),
                    Err(error) => {
                        // Reset the read position
                        *position = position.checked_sub(4).unwrap_or_else(|| unreachable!());
                        *position = position.checked_sub(len).unwrap_or_else(|| unreachable!());
                        Err(error)
                    }
                }
            }
            Ok(Cow::Owned(vec)) => {
                match String::from_utf8(vec)
                    .map_err(|e| ReadError::invalid_utf8(len, *position, e.utf8_error()))
                {
                    Ok(string) => Ok(Cow::Owned(string)),
                    Err(error) => {
                        // Reset the read position
                        *position = position.checked_sub(4).unwrap_or_else(|| unreachable!());
                        *position = position.checked_sub(len).unwrap_or_else(|| unreachable!());
                        Err(error)
                    }
                }
            }
            Err(error) => {
                // Reset the read position
                *position = position.checked_sub(4).unwrap_or_else(|| unreachable!());
                Err(error)
            }
        }
    }

    /// Read a `&str` from `source` at position `position`
    ///
    /// It will read until it finds a null byte, excluding it from the string.
    /// This function increments `position` with the size of the string + 1 if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    ///
    /// # Panics
    /// Will panic if the addition would overflow
    fn read_null_terminated_string_at<'a>(
        &'a self,
        position: &mut u64,
    ) -> Result<Cow<'a, str>, ReadError>;
}

impl ZeroCopyReadAt for &[u8] {
    fn read_fixed_slice_at<const N: usize>(
        &self,
        position: &mut u64,
    ) -> Result<[u8; N], ReadError> {
        let len = u64::try_from(N).expect("Value too large for u64");
        let new_position = position.checked_add(len).expect("Overflow occurred");
        let new_position_usize = usize::try_from(new_position).expect("Value too large for usize");
        let position_usize = usize::try_from(*position).expect("Value too large for usize");
        if self.len() < (new_position_usize) {
            Err(ReadError::source_too_small(
                len,
                *position,
                u64::try_from(self.len()).expect("Value too large for u64"),
            ))
        } else {
            let result: Result<[u8; N], _> =
                TryFrom::try_from(&self[position_usize..new_position_usize]);
            *position = new_position;
            // SAFETY: Slice will always be of size N
            Ok(result.unwrap_or_else(|_| unreachable!()))
        }
    }

    fn read_slice_at<'a>(
        &'a self,
        position: &mut u64,
        len: u64,
    ) -> Result<Cow<'a, [u8]>, ReadError> {
        let new_position = position.checked_add(len).expect("Overflow occurred");
        let new_position_usize = usize::try_from(new_position).expect("Value too large for usize");
        let position_usize = usize::try_from(*position).expect("Value too large for usize");
        if self.len() < (new_position_usize) {
            Err(ReadError::source_too_small(
                len,
                *position,
                u64::try_from(self.len()).expect("Value too large for u64"),
            ))
        } else {
            *position = new_position;
            // SAFETY: Slice will always be of size N
            Ok(Cow::Borrowed(&self[position_usize..new_position_usize]))
        }
    }

    fn read_null_terminated_string_at<'a>(
        &'a self,
        position: &mut u64,
    ) -> Result<Cow<'a, str>, ReadError> {
        let position_usize = usize::try_from(*position).expect("Value too large for usize");
        // Find the null byte, starting at `position_usize`
        let null_pos = self.iter().skip(position_usize).position(|b| b == &0);
        if let Some(null_pos) = null_pos {
            let null_pos_u64 = u64::try_from(null_pos).expect("Value too large for u64");
            match std::str::from_utf8(&self[position_usize..null_pos]) {
                Ok(str) => {
                    *position = null_pos_u64
                        .checked_add(1)
                        .unwrap_or_else(|| unreachable!());
                    Ok(Cow::Borrowed(str))
                }
                Err(error) => Err(ReadError::invalid_utf8(
                    null_pos_u64
                        .checked_sub(*position)
                        .unwrap_or_else(|| unreachable!()),
                    *position,
                    error,
                )),
            }
        } else {
            Err(ReadError::no_null_byte(*position))
        }
    }
}

impl ZeroCopyReadAt for File {
    fn read_slice_at(&self, position: &mut u64, len: u64) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_usize = usize::try_from(len).expect("Value too large for usize");
        let new_position = position.checked_add(len).expect("Overflow occured");
        let mut buf = vec![0; len_usize];
        self.read_exact_at(*position, &mut buf)
            .map_err(|e| ReadError::io_error(*position, e))?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

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
            let bytes_read = self
                .read_at(new_position, &mut read_buf)
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
                let end_position = new_position.checked_add(found).expect("Overflow occured");
                let string = String::from_utf8(result_buf).map_err(|error| {
                    ReadError::invalid_utf8(
                        end_position
                            .checked_sub(*position)
                            .unwrap_or_else(|| unreachable!()),
                        *position,
                        error.utf8_error(),
                    )
                })?;
                // Set position past the null byte
                *position = end_position.checked_add(1).expect("Overflow occured");
                return Ok(Cow::Owned(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position
                .checked_add(bytes_read)
                .expect("Overflow occured");
        }
    }
}

impl ZeroCopyReadAt for Vec<u8> {
    fn read_slice_at(&self, position: &mut u64, len: u64) -> Result<Cow<'static, [u8]>, ReadError> {
        let len_usize = usize::try_from(len).expect("Value too large for usize");
        let new_position = position.checked_add(len).expect("Overflow occured");
        let mut buf = vec![0; len_usize];
        self.read_exact_at(*position, &mut buf)
            .map_err(|e| ReadError::io_error(*position, e))?;
        *position = new_position;
        Ok(Cow::Owned(buf))
    }

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
            let bytes_read = self
                .read_at(new_position, &mut read_buf)
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
                let end_position = new_position.checked_add(found).expect("Overflow occured");
                let string = String::from_utf8(result_buf).map_err(|error| {
                    ReadError::invalid_utf8(
                        end_position
                            .checked_sub(*position)
                            .unwrap_or_else(|| unreachable!()),
                        *position,
                        error.utf8_error(),
                    )
                })?;
                // Set position past the null byte
                *position = end_position.checked_add(1).expect("Overflow occured");
                return Ok(Cow::Owned(string));
            }

            // No null byte found, add everything to `result_buf` and search further
            result_buf.extend_from_slice(&read_buf);
            new_position = new_position
                .checked_add(bytes_read)
                .expect("Overflow occured");
        }
    }
}
