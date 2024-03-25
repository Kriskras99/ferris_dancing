//! Virtual Filesystem
//! Contains traits for a virtual filesystem and implementations of some basic filesystems.
use std::{borrow::Cow, ops::Deref, path::Path, sync::Arc};

use memmap2::Mmap;
use stable_deref_trait::StableDeref;
use yoke::Yokeable;

use crate::bytes::read::{ReadError, TrivialClone, ZeroCopyReadAt};

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
    fn open<'rf>(&'rf self, path: &Path) -> std::io::Result<VirtualFile<'rf>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &Path) -> std::io::Result<VirtualMetadata>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn walk_filesystem<'rf>(&'rf self, path: &Path) -> std::io::Result<WalkFs<'rf>>;

    /// Check if `path` exists
    fn exists(&self, path: &Path) -> bool;
}

pub struct WalkFs<'a> {
    pub paths: Vec<&'a Path>,
}

impl ExactSizeIterator for WalkFs<'_> {}

impl<'a: 'b, 'b> WalkFs<'a> {
    pub fn merge(mut self, mut other: WalkFs<'a>) -> WalkFs<'b> {
        let mut paths = Vec::with_capacity(self.paths.capacity() + other.paths.capacity());
        paths.append(&mut self.paths);
        paths.append(&mut other.paths);
        WalkFs { paths }
    }
}

impl<'a> Iterator for WalkFs<'a> {
    type Item = &'a Path;

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
    pub file_size: u64,
    pub created: Result<u64, std::io::ErrorKind>,
}

impl VirtualMetadata {
    /// File size in bytes
    ///
    /// This is the after the file is retrieved from the virtual filesystem.
    /// Therefore, if the filesystem compresses files it might be different from the size on the filesystem.
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    /// Creation time in seconds since the Unix epoch
    ///
    /// # Errors
    /// Can error if creation time is not available or file access failed
    pub fn created(&self) -> std::io::Result<u64> {
        self.created.map_err(|e| e.into())
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
