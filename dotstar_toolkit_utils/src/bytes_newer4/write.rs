//! Contains the new byte writing traits
use std::backtrace::Backtrace;

use positioned_io::WriteAt as PWrite;
use thiserror::Error;

use super::Len;

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
    #[error("IoError occured while trying to write to the destination at {position}: {error}")]
    IoError {
        /// Position in the destination
        position: u64,
        /// The error
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
}

impl WriteError {
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
pub trait BinarySerialize: Sized {
    /// Deserialize the object from the reader
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn serialize(&self, writer: &mut (impl ZeroCopyWriteAt + ?Sized)) -> Result<(), WriteError> {
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
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError>;
}

impl BinarySerialize for u8 {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, &[*self])
    }
}

impl<const N: usize> BinarySerialize for [u8; N] {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_slice_at(position, self.as_slice())
    }
}

/// Represents a byte source which uses Cow's to stay zerocopy
pub trait ZeroCopyWriteAt {
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
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError>;

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
        ty: &str,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        let slice = ty.as_bytes();
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
        ty: &[u8],
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        L::write_len_at(self, position, ty.len())?;
        self.write_slice_at(position, ty)?;
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
    fn write_len_type_at<'de, L>(
        &mut self,
        position: &mut u64,
        ty: impl Iterator<Item = impl BinarySerialize> + ExactSizeIterator,
    ) -> Result<(), WriteError>
    where
        L: Len<'de>,
    {
        L::write_len_at(self, position, ty.len())?;
        for t in ty {
            self.write_at(position, &t)?;
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
        ty: &str,
    ) -> Result<(), WriteError> {
        let slice = ty.as_bytes();
        self.write_slice_at(position, slice)?;
        self.write_at(position, &0u8)?;
        Ok(())
    }
}

impl<T> ZeroCopyWriteAt for T
where
    T: PWrite,
{
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), WriteError> {
        self.write_all_at(*position, ty)
            .map_err(|e| WriteError::io_error(*position, e))?;
        Ok(())
    }
}
