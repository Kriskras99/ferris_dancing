//! Virtual Filesystem
//! Contains traits for a virtual filesystem and implementations of some basic filesystems.
use std::{borrow::Cow, ops::Deref, sync::Arc};

use memmap2::Mmap;
use stable_deref_trait::StableDeref;
use yoke::Yokeable;

use crate::bytes::read::{ReadError, TrivialClone, ZeroCopyReadAt};

pub mod layeredfs;
pub mod native;
pub mod path;
pub mod symlinkfs;
pub mod vecfs;

pub use path::{Component, VirtualPath, VirtualPathBuf};

/// Represents the operations that can be done on a readonly filesystem
pub trait VirtualFileSystem: Sync {
    /// Open a file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn open<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<VirtualFile<'rf>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>>;

    /// Check if `path` exists
    fn exists(&self, path: &VirtualPath) -> bool;
}

/// An iterator over all files under a directory
#[derive(Default)]
pub struct WalkFs<'a> {
    /// The files that haven't been iterated yet
    paths: Vec<&'a VirtualPath>,
}

impl ExactSizeIterator for WalkFs<'_> {}

impl<'a> WalkFs<'a> {
    /// Create a `WalkFs` that iterates over `paths`
    #[must_use]
    pub fn new(paths: Vec<&'a VirtualPath>) -> Self {
        Self { paths }
    }

    /// Merge another `WalkFs` iterator into this one
    pub fn merge(&mut self, other: &Self) {
        self.paths.extend_from_slice(&other.paths);
        self.paths.sort_unstable();
        self.paths.dedup();
        self.paths.shrink_to_fit();
    }
}

impl<'a> Iterator for WalkFs<'a> {
    type Item = &'a VirtualPath;

    fn next(&mut self) -> Option<Self::Item> {
        self.paths.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.paths.len();
        (len, Some(len))
    }
}

/// Represents metadata that can be obtained about a file
pub struct VirtualMetadata {
    /// The size of the file in bytes, without any filesystem based compression
    pub file_size: u64,
    /// Creation time of file in seconds since the Unix Epoch
    pub created: Result<u64, std::io::ErrorKind>,
}

impl VirtualMetadata {
    /// File size in bytes
    ///
    /// This is the after the file is retrieved from the virtual filesystem.
    /// Therefore, if the filesystem compresses files it might be different from the size on the filesystem.
    #[must_use]
    pub const fn file_size(&self) -> u64 {
        self.file_size
    }

    /// Creation time in seconds since the Unix epoch
    ///
    /// # Errors
    /// Can error if creation time is not available or file access failed
    pub fn created(&self) -> std::io::Result<u64> {
        self.created.map_err(Into::into)
    }
}

/// The content of a file from a filesystem
#[derive(Clone, Yokeable)]
pub enum VirtualFile<'f> {
    /// The content is directly borrowed from the virtual filesystem
    Slice(&'f [u8]),
    /// The content is owned by this type
    Vec(Arc<Vec<u8>>),
    /// The content is is a mmap
    Mmap(Arc<Mmap>),
}

unsafe impl StableDeref for VirtualFile<'_> {}

impl TrivialClone for VirtualFile<'_> {}

impl Deref for VirtualFile<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            VirtualFile::Slice(data) => data,
            VirtualFile::Vec(data) => data.deref(),
            VirtualFile::Mmap(data) => data.deref(),
        }
    }
}

impl<'vf> ZeroCopyReadAt for VirtualFile<'vf> {
    fn read_null_terminated_string_at<'de>(
        &'de self,
        position: &mut u64,
    ) -> Result<Cow<'de, str>, ReadError> {
        match self {
            VirtualFile::Slice(data) => data.read_null_terminated_string_at(position),
            VirtualFile::Vec(data) => data.deref().read_null_terminated_string_at(position),
            VirtualFile::Mmap(data) => data.deref().read_null_terminated_string_at(position),
        }
    }

    fn read_slice_at<'de>(
        &'de self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'de, [u8]>, ReadError> {
        match self {
            VirtualFile::Slice(data) => data.read_slice_at(position, len),
            VirtualFile::Vec(data) => data.deref().read_slice_at(position, len),
            VirtualFile::Mmap(data) => data.deref().read_slice_at(position, len),
        }
    }
}

impl<'f> From<&'f [u8]> for VirtualFile<'f> {
    fn from(value: &'f [u8]) -> Self {
        Self::Slice(value)
    }
}

impl From<Arc<Vec<u8>>> for VirtualFile<'_> {
    fn from(value: Arc<Vec<u8>>) -> Self {
        Self::Vec(value)
    }
}

impl From<Arc<Mmap>> for VirtualFile<'_> {
    fn from(value: Arc<Mmap>) -> Self {
        Self::Mmap(value)
    }
}
