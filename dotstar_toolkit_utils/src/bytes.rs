//! # Bytes
//! Contains functions to read integers and strings from byte slices.
use anyhow::{anyhow, Context, Error};
pub use byteorder::ByteOrder;

/// Read a `u8` from `source` at position `position`
///
/// This function increments `position` with 1 if successful
///
/// # Errors
/// This function will return an error when the u8 would be (partially) outside the source.
///
/// # Panics
/// Will panic if the addition would overflow
pub fn read_u8_at(source: &[u8], position: &mut usize) -> Result<u8, Error> {
    let new_position = position.checked_add(1).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read 1 byte at {position} but file is only {} bytes!", source.len()))
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
pub fn read_u16_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u16, Error> {
    let new_position = position.checked_add(2).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read 2 bytes at {position} but file is only {} bytes!", source.len()))
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
pub fn read_u24_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, Error> {
    let new_position = position.checked_add(3).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read 3 bytes at {position} but file is only {} bytes!", source.len()))
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
pub fn read_u32_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u32, Error> {
    let new_position = position.checked_add(4).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read 4 bytes at {position} but file is only {} bytes!", source.len()))
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
pub fn read_u64_at<T: ByteOrder>(source: &[u8], position: &mut usize) -> Result<u64, Error> {
    let new_position = position.checked_add(8).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read 8 bytes at {position} but file is only {} bytes!", source.len()))
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
) -> Result<&'b str, Error> {
    let len = usize::try_from(read_u32_at::<T>(source, position)?)?;
    let new_position = position.checked_add(len).expect("Overflow occurred!");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read {len} bytes at {position} but file is only {} bytes!", source.len()))
    } else {
        let str = std::str::from_utf8(&source[*position..new_position]).with_context(|| {
            format!("Invalid UTF-8, attempting to read {len} bytes at {position}!")
        })?;
        *position = new_position;
        Ok(str)
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
) -> Result<&'b str, Error> {
    let null_pos = source.iter().skip(*position).position(|b| b == &0);
    if let Some(null_pos) = null_pos {
        let start = *position;
        #[allow(clippy::arithmetic_side_effects)]
        let end = null_pos - 1; // Exclude the null byte
        *position = null_pos;
        Ok(std::str::from_utf8(&source[start..end])?)
    } else {
        Err(anyhow!("Null byte not found for null terminated string!"))
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
) -> Result<&'b [u8; N], Error> {
    let new_position = position.checked_add(N).expect("Overflow occurred");
    if source.len() < (new_position) {
        Err(anyhow!("Source is not large enough! Attempted to read {N} bytes at {position} but file is only {} bytes!", source.len()))
    } else {
        let result: Result<&[u8; N], _> = TryFrom::try_from(&source[*position..new_position]);
        *position = new_position;
        // SAFETY: Slice will always be of size N
        Ok(unsafe { result.unwrap_unchecked() })
    }
}
