//! Interfaces and utilities for reading and writing data and types

pub mod endian;
pub mod len;
pub mod primitives;
pub mod read;
pub mod write;

use std::{
    fmt::Debug,
    fs::File,
    io::{Error, ErrorKind, Read, Seek, Write},
    path::Path,
};

use read::ReadError;
use tracing::instrument;

use self::{read::ReadAt, write::WriteAt};

/// Read the file at path into a `Vec`
pub fn read_to_vec<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path.as_ref())?;
    let file_size = usize::try_from(file.metadata()?.len()).map_err(Error::other)?;
    let mut buf = Vec::with_capacity(file_size);
    std::io::Read::read_to_end(&mut file, &mut buf)?;
    Ok(buf)
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
    #[instrument(skip(self, buf))]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut len = buf.len();
        while len > 0 {
            match self.inner.read_slice_at(&mut self.position, len) {
                Ok(slice) => {
                    buf[0..len].copy_from_slice(&slice);
                    break;
                }
                Err(ReadError::IoError {
                    error,
                    backtrace: _,
                }) if error.kind() == ErrorKind::UnexpectedEof => {
                    len = len.checked_sub(1).unwrap_or_else(|| unreachable!());
                }
                Err(err) => {
                    return Err(Error::other(err));
                }
            }
        }
        Ok(len)
    }
}

impl<T> Seek for CursorAt<T> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        match pos {
            std::io::SeekFrom::Start(pos) => self.position = pos,
            std::io::SeekFrom::End(_) => {
                return Err(Error::new(
                    ErrorKind::Unsupported,
                    "SeekFrom::End is not supported for CursorAt",
                ))
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
