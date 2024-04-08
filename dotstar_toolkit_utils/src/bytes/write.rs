//! Contains the new byte writing traits
use std::{
    backtrace::Backtrace,
    fs::File,
    io::{BufWriter, Cursor, ErrorKind, Seek, SeekFrom, Write},
    num::TryFromIntError,
};

use positioned_io::WriteAt as PWriteAt;
use thiserror::Error;

use super::Len;
use crate::testing::TestError;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum WriteError {
    /// WriteError with context
    #[error("{source:?}\n    Context: {context}")]
    Context {
        /// The original error
        source: Box<Self>,
        /// Added context
        context: String,
    },
    /// Encountered an I/O error while trying to write to the destination
    #[error("IoError occured while trying to write to the destination: {error}")]
    IoError {
        /// The error
        #[from]
        error: std::io::Error,
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
    /// An integer could not be converted to another integer size
    #[error("an integer could not be converted to another integer size: {tfie:?}")]
    IntConversion {
        /// The original test error
        #[from]
        tfie: TryFromIntError,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Something went wrong
    #[error("something went wrong: {test:?}")]
    Test {
        /// The original test error
        #[from]
        test: TestError,
        /// Backtrace
        backtrace: Backtrace,
    },
    /// Integer over/underflow
    #[error("an integer over/underflow occured")]
    IntUnderOverflow {
        /// Backtrace
        backtrace: Backtrace,
    },
}

impl WriteError {
    /// Create the [`WriteError::IntUnderOverflow`] error
    #[must_use]
    pub fn int_under_overflow() -> Self {
        Self::IntUnderOverflow {
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

    /// Create a custom [`WriteError`]
    #[must_use]
    pub fn custom(string: String) -> Self {
        Self::Custom {
            string,
            backtrace: Backtrace::capture(),
        }
    }

    /// Create a custom [`WriteError`]
    #[must_use]
    pub fn with_custom<F: FnOnce() -> String>(f: F) -> Self {
        Self::Custom {
            string: f(),
            backtrace: Backtrace::capture(),
        }
    }
}

/// Represents a object that can be deserialized from a binary file
pub trait BinarySerialize: Sized {
    /// Deserialize the object from the reader
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn serialize(&self, writer: &mut (impl WriteAt + ?Sized)) -> Result<(), WriteError> {
        self.serialize_at(writer, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn serialize_at(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError>;
}

impl BinarySerialize for u8 {
    fn serialize_at(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, &[*self])
    }
}

impl<const N: usize> BinarySerialize for [u8; N] {
    fn serialize_at(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, self.as_slice())
    }
}

/// Represents a byte source which uses Cow's to stay zerocopy
pub trait WriteAt {
    /// Read a `T` at `position`
    ///
    /// This function increments `position` with what `T` reads if successful
    ///
    /// # Errors
    /// This function will return an error when the T would be (partially) outside the source.
    fn write_at(
        &mut self,
        position: &mut u64,
        ty: &impl BinarySerialize,
    ) -> Result<(), WriteError> {
        ty.serialize_at(self, position)
    }

    /// Read a `&[u8]` of length `len` at `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    fn write_slice_at(&mut self, position: &mut u64, buf: &[u8]) -> Result<(), WriteError>;

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn write_len_string_at<'de, L>(
        &mut self,
        position: &mut u64,
        string: &str,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        let slice = string.as_bytes();
        self.write_len_slice_at::<L>(position, slice)
    }

    /// Read a byte slice at `position`
    ///
    /// It will first read the length of the byte slice as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn write_len_slice_at<'de, L>(
        &mut self,
        position: &mut u64,
        buf: &[u8],
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        L::write_len_at(self, position, buf.len())?;
        self.write_slice_at(position, buf)?;
        Ok(())
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
    fn write_len_type_at<'de, 'a, L>(
        &mut self,
        position: &mut u64,
        ty: impl ExactSizeIterator<Item = &'a (impl BinarySerialize + 'a)>,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        L::write_len_at(self, position, ty.len())?;
        for t in ty {
            self.write_at(position, t)?;
        }
        Ok(())
    }

    /// Read a `&str` from `source` at `position`
    ///
    /// It will read until it finds a null byte, excluding it from the string.
    /// This function increments `position` with the size of the string + 1 if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    fn write_null_terminated_string_at(
        &mut self,
        position: &mut u64,
        string: &str,
    ) -> Result<(), WriteError> {
        let slice = string.as_bytes();
        self.write_slice_at(position, slice)?;
        self.write_at(position, &0u8)?;
        Ok(())
    }
}

// impl<T> ZeroCopyWriteAt for T
// where
//     T: PWrite,
// {
//     fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
//         self.write_all_at(*position, ty)
//             .map_err(|e| WriteError::io_error(*position, e))?;
//         Ok(())
//     }
// }

impl WriteAt for Vec<u8> {
    fn write_slice_at(&mut self, position: &mut u64, buf: &[u8]) -> Result<(), WriteError> {
        let position_usize = usize::try_from(*position)?;
        let end = position_usize
            .checked_add(buf.len())
            .ok_or_else(WriteError::int_under_overflow)?;
        if end >= self.len() {
            self.resize(end, 0);
        }
        self[position_usize..end].copy_from_slice(buf);
        *position = u64::try_from(end)?;
        Ok(())
    }
}

// How to make this generic??
impl WriteAt for Cursor<&mut Vec<u8>> {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.seek(SeekFrom::Start(*position))?;
        self.write_all(ty)?;
        *position = self.position();
        Ok(())
    }
}

impl WriteAt for File {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.write_all_at(*position, ty)?;
        *position = position
            .checked_add(u64::try_from(ty.len())?)
            .ok_or_else(WriteError::int_under_overflow)?;
        Ok(())
    }
}

impl<T: Write + Seek> WriteAt for BufWriter<T> {
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.seek(SeekFrom::Start(*position))?;
        self.write_all(ty)?;
        *position = position
            .checked_add(u64::try_from(ty.len())?)
            .ok_or_else(WriteError::int_under_overflow)?;
        Ok(())
    }
}

/// Provides a wrapper around a `WriteAt` item so it can be used with `Write`-based interfaces
pub struct CursorAt<'a, W: WriteAt + ?Sized> {
    /// The writer that is wrapped
    writer: &'a mut W,
    /// The current position in the writer
    position: &'a mut u64,
}

impl<'a, W: WriteAt + ?Sized> CursorAt<'a, W> {
    /// Create a new `CursorAt` that will start writing at `position`
    pub fn new(writer: &'a mut W, position: &'a mut u64) -> Self {
        Self { writer, position }
    }
}

impl<'a, W: WriteAt + ?Sized> Write for CursorAt<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer
            .write_slice_at(self.position, buf)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
