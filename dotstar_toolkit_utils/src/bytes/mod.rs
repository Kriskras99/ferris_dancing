//! Interfaces and utilities for reading and writing data and types

pub mod endian;
pub mod primitives;
// pub mod primitives2;
pub mod read;
pub mod write;

use std::{
    fmt::Debug,
    fs::File,
    io::{Error, ErrorKind, Read, Seek, Write},
    ops::Deref,
    path::Path,
};

use self::{
    read::{BinaryDeserialize, ReadAt, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
};

/// Read the file at path into a `Vec`
pub fn read_to_vec<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())?;
    let file_size = usize::try_from(file.metadata()?.len()).map_err(Error::other)?;
    let mut buf = Vec::with_capacity(file_size);
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    Ok(buf)
}

/// Represents the length of a string or slice to read from the reader
pub trait Len<'rf>:
    BinaryDeserialize<'rf> + BinarySerialize + Sized + TryFrom<usize> + TryInto<usize>
{
    /// Read the length at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn read_len_at(
        reader: &'rf (impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<usize, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let len = Self::deserialize_at(reader, position)?;
            TryInto::<usize>::try_into(len)
                .map_err(|_| ReadError::custom("Len does not fit in usize!".into()))?
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }

    /// Write `length` at `position`
    ///
    /// Will increment position with the size of length if successful
    ///
    /// # Errors
    /// This function will return an error when `Len` would be (partially) outside the source or the `Len` does not fit into a u64.
    fn write_len_at(
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        len: usize,
    ) -> Result<(), WriteError> {
        let len = Self::try_from(len).unwrap_or_else(|_| todo!());
        writer.write_at(position, &len)?;
        Ok(())
    }
}

pub struct CursorAt<T> {
    /// The item that is wrapped
    inner: T,
    /// The current position in the item
    position: u64,
}

impl<T: Clone> Clone for CursorAt<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            position: self.position,
        }
    }
}

impl<T> Debug for CursorAt<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CursorAt")
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}

impl<T> CursorAt<T> {
    /// Create a new `CursorAt` that will start writing at `position`
    pub const fn new(inner: T, position: u64) -> Self {
        Self { inner, position }
    }

    /// Deconstruct the cursor into the inner type and the current position
    pub fn into_inner(self) -> (T, u64) {
        (self.inner, self.position)
    }
}

impl<T: WriteAt> Write for CursorAt<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner
            .write_slice_at(&mut self.position, buf)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<T: ReadAt> Read for CursorAt<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let slice = self
            .inner
            .read_slice_at(&mut self.position, buf.len())
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        buf.copy_from_slice(&slice);
        Ok(buf.len())
    }
}

impl<T: Deref<Target = [u8]>> Seek for CursorAt<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(pos) => self.position = pos,
            std::io::SeekFrom::End(pos) => {
                let size = i64::try_from(self.inner.deref().len()).map_err(Error::other)?;
                let new_pos = size
                    .checked_sub(pos)
                    .ok_or_else(|| Error::other("Integer underflow"))?;
                self.position = u64::try_from(new_pos).map_err(Error::other)?;
            }
            std::io::SeekFrom::Current(pos) => {
                let old_pos = i64::try_from(self.position).map_err(Error::other)?;
                let new_pos = old_pos
                    .checked_sub(pos)
                    .ok_or_else(|| Error::other("Integer underflow"))?;
                self.position = u64::try_from(new_pos).map_err(Error::other)?;
            }
        };
        Ok(self.position)
    }
}
