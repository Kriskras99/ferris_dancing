//! Virtual Filesystem
//! Contains traits for a virtual filesystem and implementations of some basic filesystems.
use std::{borrow::Cow, ops::Deref, path::Path, sync::Arc};

use memmap2::Mmap;

use crate::bytes_newer4::read::{ZeroCopyReadAt, ReadError};

pub mod layeredfs;
pub mod native;
pub mod symlinkfs;
pub mod vecfs;

/// Represents the operations that can be done on a readonly filesystem
pub trait VirtualFileSystem: Sync {
    /// Open a file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<Arc<VirtualFile<'fs>>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn list_files(&self, path: &Path) -> std::io::Result<Vec<String>>;

    /// Check if `path` exists
    fn exists(&self, path: &Path) -> bool;
}

/// Represents metadata that can be obtained about a file
pub trait VirtualFileMetadata {
    /// File size in bytes
    ///
    /// This is the after the file is retrieved from the virtual filesystem.
    /// Therefore, if the filesystem compresses files it might be different from the size on the filesystem.
    fn file_size(&self) -> u64;

    /// Creation time in seconds since the Unix epoch
    ///
    /// # Errors
    /// Can error if creation time is not available or file access failed
    fn created(&self) -> std::io::Result<u64>;
}

/// The content of a file from a filesystem
pub enum VirtualFile<'f> {
    /// The content is directly borrowed from the virtual filesystem
    Slice(&'f [u8]),
    /// The content is owned by this type
    Vec(Vec<u8>),
    /// The content is is a mmap
    Mmap(Mmap),
}

impl<'vf> ZeroCopyReadAt for VirtualFile<'vf> {
    fn read_null_terminated_string_at<'de>(
        &'de self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, ReadError> {
        self.deref().read_null_terminated_string_at(position)
    }

    fn read_slice_at<'de>(&'de self, position: &mut u64, len: usize) -> Result<Cow<'de, [u8]>, ReadError> {
        self.deref().read_slice_at(position, len)
    }
}

impl<'f> From<&'f [u8]> for VirtualFile<'f> {
    fn from(value: &'f [u8]) -> Self {
        Self::Slice(value)
    }
}

impl From<Vec<u8>> for VirtualFile<'_> {
    fn from(value: Vec<u8>) -> Self {
        Self::Vec(value)
    }
}

impl From<Mmap> for VirtualFile<'_> {
    fn from(value: Mmap) -> Self {
        Self::Mmap(value)
    }
}
