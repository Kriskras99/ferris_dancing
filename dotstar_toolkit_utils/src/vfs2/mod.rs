use std::{ops::Deref, path::Path};

use crate::bytes_newer4::read::ZeroCopyReadAtExt;

pub mod layeredfs;
pub mod native;
pub mod symlinkfs;
pub mod vecfs;

/// Represents the operations that can be done on a readonly filesystem
pub trait VirtualFileSystem: Sync {
    type VirtualFile<'fs>: VirtualFile<'fs>
    where
        Self: 'fs;

    type VirtualMetadata: VirtualMetadata;

    /// Open a file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<Self::VirtualFile<'fs>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn list_files<'fs>(&'fs self, path: &Path) -> std::io::Result<impl Iterator<Item = &'fs Path>>;

    /// Check if `path` exists
    fn exists(&self, path: &Path) -> bool;
}

pub trait VirtualFile<'fs>: Sync + ZeroCopyReadAtExt + Deref<Target = [u8]> {
    fn len(&self) -> usize;
}

pub trait VirtualMetadata {
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
