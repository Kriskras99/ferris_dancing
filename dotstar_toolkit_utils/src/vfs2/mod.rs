use std::path::Path;

use crate::bytes_newer4::read::ZeroCopyReadAtExt;

pub mod vecfs;
pub mod layeredfs;

/// Represents the operations that can be done on a readonly filesystem
pub trait VirtualFileSystem: Sync {
    type VirtualFile<'fs>: VirtualFile<'fs> where Self: 'fs;

    /// Open a file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<Self::VirtualFile<'fs>>;

    /// Get the metadata for the file at `path`
    ///
    /// # Errors
    /// Can error if the file does not exist or if file access failed
    // fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>>;

    /// List all files at `path` and deeper
    ///
    /// # Errors
    /// Can error if the directory does not exist or if directory access failed
    fn list_files<'fs>(&'fs self, path: &Path) -> std::io::Result<impl Iterator<Item = &'fs Path>>;

    /// Check if `path` exists
    fn exists(&self, path: &Path) -> bool;
}

pub trait VirtualFile<'fs>: Sync + ZeroCopyReadAtExt {}
