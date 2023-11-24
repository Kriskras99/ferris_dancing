//! # Bytes
//! Contains functions to read integers and strings from byte slices.
use std::{fs::File, path::Path, str::Utf8Error};

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
    /// Trying to read outside source
    #[error("source is not large enough, attempted to read {n} bytes at {position} but source is only {size} bytes")]
    SourceTooSmall {
        /// Amount of bytes that were needed
        n: usize,
        /// Position in the source
        position: usize,
        /// The total size of the source
        size: usize,
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
    },
    #[error("no null-byte for null terminated string, attempted to read a string at {position}")]
    /// Encountered no null byte when trying to read a null-terminated string
    NoNullByte {
        /// Position in the source
        position: usize,
    },
}

impl ReadError {
    /// Create the [`ReadError::SourceTooSmall`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    fn source_too_small(n: usize, position: usize, size: usize) -> Self {
        Self::SourceTooSmall { n, position, size }
    }

    /// Create the [`ReadError::InvalidUTF8`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    fn invalid_utf8(n: usize, position: usize, error: Utf8Error) -> Self {
        Self::InvalidUTF8 { n, position, error }
    }

    /// Create the [`ReadError::NoNullByte`] error
    // Want to add std::backtrace::Backtrace, but blocked on https://github.com/rust-lang/rust/issues/99301
    #[allow(clippy::missing_const_for_fn)]
    fn no_null_byte(position: usize) -> Self {
        Self::NoNullByte { position }
    }
}

/// Read a `u8` from `source` at position `position`
///
/// This function increments `position` with 1 if successful
///
/// # Errors
/// This function will return an error when the u8 would be (partially) outside the source.
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u8_at(source: &[u8], position: &mut usize) -> Result<u8, ReadError> {
    let new_position = position.checked_add(1).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u16_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u16, ReadError> {
    let new_position = position.checked_add(2).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u24_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, ReadError> {
    let new_position = position.checked_add(3).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u32_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, ReadError> {
    let new_position = position.checked_add(4).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u64_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u64, ReadError> {
    let new_position = position.checked_add(8).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_string_at<'b, T: ByteOrder>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<&'b str, ReadError> {
    let len = usize::try_from(read_u32_at::<T>(source, position)?)
        .expect("Attempted to read more bytes than can fit in a usize");
    let new_position = position.checked_add(len).expect("Overflow occurred!");
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
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_slice_at<'b, const N: usize>(
    source: &'b [u8],
    position: &mut usize,
) -> Result<&'b [u8; N], ReadError> {
    let new_position = position.checked_add(N).expect("Overflow occurred");
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
