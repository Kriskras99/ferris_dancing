//! Contains the new byte writing traits
use std::backtrace::Backtrace;

pub use byteorder::ByteOrder;
use byteorder::NativeEndian;
use positioned_io::WriteAt;
use thiserror::Error;
use ux::u24;

use super::Len;

/// Errors returend when the test* functions fail
#[derive(Error, Debug)]
pub enum NewWriteError {
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
    #[error("attempted to write more bytes than can be pointed to")]
    /// Attempted to write more bytes than can be pointed to
    TooManyBytes {
        /// Position in the destination
        position: u64,
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
}

impl NewWriteError {
    /// Create the [`ReadError::IoError`] error
    #[must_use]
    pub fn io_error(position: u64, error: std::io::Error) -> Self {
        Self::IoError {
            position,
            error,
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

    /// Create the [`ReadError::PositionOverflow`] error
    #[must_use]
    pub fn position_overflow(position: u64, n: u64) -> Self {
        Self::PositionOverflow {
            position,
            n,
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
}

/// Represents a object that can be deserialized from a binary file
pub trait BinarySerialize: Sized {
    /// Deserialize the object from the reader
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn serialize<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        self.serialize_at::<B>(writer, &mut 0)
    }

    /// Deserialize the object from the reader at `position`
    ///
    /// Note: Must restore position to the original value on error!
    ///
    /// # Errors
    /// This function will return an error when deserializing fails.
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder;
}

impl BinarySerialize for u8 {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        writer.write_slice_at(position, &[*self])?;
        Ok(())
    }
}

impl BinarySerialize for u16 {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        let mut buf = [0; 2];
        B::write_u16(&mut buf, *self);
        writer.write_slice_at(position, &buf)?;
        Ok(())
    }
}

impl BinarySerialize for u24 {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        let mut buf = [0; 3];
        B::write_u24(&mut buf, u32::from(*self));
        writer.write_slice_at(position, &buf)?;
        Ok(())
    }
}

impl BinarySerialize for u32 {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        let mut buf = [0; 4];
        B::write_u32(&mut buf, *self);
        writer.write_slice_at(position, &buf)?;
        Ok(())
    }
}

impl BinarySerialize for u64 {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        let mut buf = [0; 8];
        B::write_u64(&mut buf, *self);
        writer.write_slice_at(position, &buf)?;
        Ok(())
    }
}

impl<const N: usize> BinarySerialize for [u8; N] {
    fn serialize_at<B>(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
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
    fn write_at<B>(
        &mut self,
        position: &mut u64,
        ty: &impl BinarySerialize,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
    {
        ty.serialize_at::<B>(self, position)
    }

    /// Read a `&[u8]` of length `len` at `position`
    ///
    /// This function increments `position` with N if successful
    ///
    /// # Errors
    /// This function will return an error when the data would be (partially) outside the source.
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), NewWriteError>;

    /// Read a string at `position`
    ///
    /// It will first read the length of the string as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the string + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn write_len_string_at<'de, B, L>(
        &mut self,
        position: &mut u64,
        ty: &str,
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let slice = ty.as_bytes();
        self.write_len_slice_at::<B, L>(position, slice)
    }

    /// Read a byte slice at `position`
    ///
    /// It will first read the length of the byte slice as a [`Len`] with byteorder `B`
    /// This function increments `position` with the size of the byte slice + the size of `Len` if successful
    ///
    /// # Errors
    /// This function will return an error when the string would be (partially) outside the source.
    #[inline(always)]
    fn write_len_slice_at<'de, B, L>(
        &mut self,
        position: &mut u64,
        ty: &[u8],
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let len = L::try_from(ty.len()).map_err(|_| NewWriteError::too_many_bytes(*position))?;
        self.write_at::<B>(position, &len)?;
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
    fn write_len_type_at<'de, B, L>(
        &mut self,
        position: &mut u64,
        ty: &[impl BinarySerialize],
    ) -> Result<(), NewWriteError>
    where
        B: ByteOrder,
        L: Len<'de>,
    {
        let len = L::try_from(ty.len()).map_err(|_| NewWriteError::too_many_bytes(*position))?;
        self.write_at::<B>(position, &len)?;
        for t in ty {
            self.write_at::<B>(position, t)?;
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
    ) -> Result<(), NewWriteError> {
        let slice = ty.as_bytes();
        self.write_slice_at(position, slice)?;
        self.write_at::<NativeEndian>(position, &0u8)?;
        Ok(())
    }
}

impl<T> ZeroCopyWriteAt for T
where
    T: WriteAt,
{
    fn write_slice_at(&mut self, position: &mut u64, ty: &[u8]) -> Result<(), NewWriteError> {
        self.write_all_at(*position, ty)
            .map_err(|e| NewWriteError::io_error(*position, e))?;
        Ok(())
    }
}
