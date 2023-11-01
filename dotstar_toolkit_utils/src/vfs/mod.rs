//! Virtual Filesystem
//! Contains traits for a virtual filesystem and implementations of some basic filesystems.
use std::{io::Result, ops::Deref, path::Path};

use stable_deref_trait::StableDeref;

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
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<VirtualFile<'fs>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &Path) -> Result<Box<dyn VirtualFileMetadata>>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn list_files(&self, path: &Path) -> Result<Vec<String>>;

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
    fn created(&self) -> Result<u64>;
}

/// Trait alias for a types that dereference to byte slices, stay in the same position, and are sync
pub trait VirtualFileInner: Deref<Target = [u8]> + StableDeref + Sync {}

impl VirtualFileInner for Vec<u8> {}

/// The content of a file from a filesystem
pub enum VirtualFile<'f> {
    /// The content is directly borrowed from the filesystem
    Borrowed(&'f [u8]),
    /// The content is owned by this type
    Owned(Box<dyn VirtualFileInner>),
}

impl<'f> From<&'f [u8]> for VirtualFile<'f> {
    fn from(value: &'f [u8]) -> Self {
        Self::Borrowed(value)
    }
}

impl From<Box<dyn VirtualFileInner>> for VirtualFile<'_> {
    fn from(value: Box<dyn VirtualFileInner>) -> Self {
        Self::Owned(value)
    }
}

impl Deref for VirtualFile<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            VirtualFile::Borrowed(inner) => inner,
            VirtualFile::Owned(inner) => inner.as_ref(),
        }
    }
}

unsafe impl StableDeref for VirtualFile<'_> {}
