//! # Bytes
//! Contains functions to read integers and strings from byte slices.
use std::{backtrace::Backtrace, fmt::Debug, fs::File, path::Path, str::Utf8Error};

pub use byteorder::{BigEndian, ByteOrder, LittleEndian};
use thiserror::Error;

/// Read the file at path into a `Vec`
///
/// # Errors
/// - Cannot open the file
/// - Cannot get metadata for the file
/// - Filesize is bigger than `usize`
/// - Cannot read the entire file
pub fn read_to_vec<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())?;
    let file_size = usize::try_from(file.metadata()?.len()).map_err(std::io::Error::other)?;
    let mut buf = Vec::with_capacity(file_size);
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    Ok(buf)
}

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
    /// Trying to read outside source
    #[error("source is not large enough, attempted to read {n} bytes at {position} but source is only {size} bytes")]
    SourceTooSmall {
        /// Amount of bytes that were needed
        n: usize,
        /// Position in the source
        position: usize,
        /// The total size of the source
        size: usize,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Encountered invalid UTF-8 when trying to read a string from source
    #[error("invalid UTF-8 encountered, attempted to read a string of length {n} at {position}")]
    InvalidUTF8 {
        /// Amount of bytes that were needed
        n: usize,
        /// Position in the source
        position: usize,
        /// Original UTF-8 error
        error: Utf8Error,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("no null-byte for null terminated string, attempted to read a string at {position}")]
    /// Encountered no null byte when trying to read a null-terminated string
    NoNullByte {
        /// Position in the source
        position: usize,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("attempted to increment position {position} by {n}, but that would overflow")]
    /// Increasing the position would overflow the number
    PositionOverflow {
        /// Position in the source
        position: usize,
        /// How much the increment would be
        n: usize,
        /// Backtrace
        backtrace: Backtrace,
    },
    #[error("attempted to read more bytes than can be pointed to")]
    /// Attempted to read more bytes than can be pointed to
    TooManyBytes {
        /// Position in the source
        position: usize,
        /// Backtrace
        backtrace: Backtrace,
    },
}

impl ReadError {
    /// Create the [`ReadError::SourceTooSmall`] error
    fn source_too_small(n: usize, position: usize, size: usize) -> Self {
        Self::SourceTooSmall {
            n,
            position,
            size,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::InvalidUTF8`] error
    fn invalid_utf8(n: usize, position: usize, error: Utf8Error) -> Self {
        Self::InvalidUTF8 {
            n,
            position,
            error,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::NoNullByte`] error
    fn no_null_byte(position: usize) -> Self {
        Self::NoNullByte {
            position,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::PositionOverflow`] error
    fn position_overflow(position: usize, n: usize) -> Self {
        Self::PositionOverflow {
            position,
            n,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create the [`ReadError::TooManyBytes`] error
    fn too_many_bytes(position: usize) -> Self {
        Self::TooManyBytes {
            position,
            backtrace: Backtrace::capture(),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn context<C: Debug>(self, context: C) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{context:?}"),
        }
    }

    /// Add context for this error
    #[must_use]
    pub fn with_context<C: Debug, F: FnOnce() -> C>(self, f: F) -> Self {
        Self::Context {
            source: Box::new(self),
            context: format!("{:?}", f()),
        }
    }
}

/// Read a `u8` from `source` at position `position`
///
/// This function increments `position` with 1 if successful
///
/// # Errors
/// This function will return an error when the u8 would be (partially) outside the source.
pub fn read_u8_at(source: &[u8], position: &mut usize) -> Result<u8, ReadError> {
    let new_position = position
        .checked_add(1)
        .ok_or_else(|| ReadError::position_overflow(*position, 1))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(1, *position, source.len()))
    } else {
        let data = source[*position];
        *position = new_position;
        Ok(data)
    }
}

/// Read a `u16` from `source` at position `position` with byteorder `T`
///
/// This function increments `position` with 2 if successful
///
/// # Errors
/// This function will return an error when the u16 would be (partially) outside the source.
pub fn read_u16_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u16, ReadError> {
    let new_position = position
        .checked_add(2)
        .ok_or_else(|| ReadError::position_overflow(*position, 2))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(2, *position, source.len()))
    } else {
        let data = T::read_u16(&source[*position..new_position]);
        *position = new_position;
        Ok(data)
    }
}

/// Read a `u24` from `source` at position `position` with byteorder `T`
///
/// This function increments `position` with 3 if successful
///
/// # Errors
/// This function will return an error when the u24 would be (partially) outside the source.
pub fn read_u24_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, ReadError> {
    let new_position = position
        .checked_add(3)
        .ok_or_else(|| ReadError::position_overflow(*position, 3))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(3, *position, source.len()))
    } else {
        let data = T::read_u24(&source[*position..new_position]);
        *position = new_position;
        Ok(data)
    }
}

/// Read a `u32` from `source` at position `position` with byteorder `T`
///
/// This function increments `position` with 4 if successful
///
/// # Errors
/// This function will return an error when the u32 would be (partially) outside the source.
pub fn read_u32_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, ReadError> {
    let new_position = position
        .checked_add(4)
        .ok_or_else(|| ReadError::position_overflow(*position, 4))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(4, *position, source.len()))
    } else {
        let data = T::read_u32(&source[*position..new_position]);
        *position = new_position;
        Ok(data)
    }
}

/// Read a `u64` from `source` at position `position` with byteorder `T`
///
/// This function increments `position` with 8 if successful
///
/// # Errors
/// This function will return an error when the u64 would be (partially) outside the source.
pub fn read_u64_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u64, ReadError> {
    let new_position = position
        .checked_add(8)
        .ok_or_else(|| ReadError::position_overflow(*position, 8))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(8, *position, source.len()))
    } else {
        let data = T::read_u64(&source[*position..new_position]);
        *position = new_position;
        Ok(data)
    }
}

/// Read a `&str` from `source` at position `position`
///
/// It will first read the length of the string as a `u32` with byteorder `T`
/// This function increments `position` with the size of the string + 4 if successful
///
/// # Errors
/// This function will return an error when the string would be (partially) outside the source.
pub fn read_string_at<'b, T: ByteOrder>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<&'b str, ReadError> {
    let len = usize::try_from(read_u32_at::<T>(source, position)?)
        .map_err(|_| ReadError::too_many_bytes(*position))?;
    let new_position = position
        .checked_add(len)
        .ok_or_else(|| ReadError::position_overflow(*position, len))?;
    if source.len() < (new_position) {
        // Reset the read position
        *position = position.checked_sub(4).unwrap_or_else(|| unreachable!());
        Err(ReadError::source_too_small(len, *position, source.len()))
    } else {
        match std::str::from_utf8(&source[*position..new_position])
            .map_err(|error| ReadError::invalid_utf8(len, *position, error))
        {
            Ok(str) => {
                *position = new_position;
                Ok(str)
            }
            Err(error) => {
                // Reset the read position
                *position = position.checked_sub(4).unwrap_or_else(|| unreachable!());
                Err(error)
            }
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
pub fn read_null_terminated_string_at<'b>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<&'b str, ReadError> {
    // Find the null byte, starting at `position`
    let null_pos = source.iter().skip(*position).position(|b| b == &0);
    if let Some(null_pos) = null_pos {
        match std::str::from_utf8(&source[*position..null_pos]) {
            Ok(str) => {
                *position = null_pos.checked_add(1).unwrap_or_else(|| unreachable!());
                Ok(str)
            }
            Err(error) => Err(ReadError::invalid_utf8(
                null_pos
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

/// Read a `&[u8: N]` from `source` at position `position`
///
/// This function increments `position` with N if successful
///
/// # Errors
/// This function will return an error when the data would be (partially) outside the source.
pub fn read_slice_at<'b, const N: usize>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<&'b [u8; N], ReadError> {
    let new_position = position
        .checked_add(N)
        .ok_or_else(|| ReadError::position_overflow(*position, N))?;
    if source.len() < (new_position) {
        Err(ReadError::source_too_small(N, *position, source.len()))
    } else {
        let result: Result<&[u8; N], _> = TryFrom::try_from(&source[*position..new_position]);
        *position = new_position;
        // SAFETY: Slice will always be of size N
        Ok(unsafe { result.unwrap_unchecked() })
    }
}

#[cfg(test)]
mod tests {
    use super::read_null_terminated_string_at;

    #[test]
    fn test_empty_null_terminated_string() {
        let data = [0x0];
        assert_eq!(read_null_terminated_string_at(&data, &mut 0).unwrap(), "")
    }

    #[test]
    fn test_null_terminated_string() {
        let data = b"HelloWorld\0";
        assert_eq!(
            read_null_terminated_string_at(data, &mut 0).unwrap(),
            "HelloWorld"
        )
    }
}
