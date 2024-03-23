//! # Overlay Filesystem
//! Implements a filesystem that overlays two filesystems, preferring the upper filesystem for operations.
use std::{
    io::{ErrorKind, Result}, path::Path, sync::Arc
};

use super::{VirtualFile, VirtualFileMetadata, VirtualFileSystem};

/// A filesystem that overlays two filesystems, preferring the upper filesystem for operations.
pub struct OverlayFs<'fs> {
    /// The upper filesystem, checked first for paths
    upper: &'fs dyn VirtualFileSystem,
    /// The lower filesystem, checked if upper doesn't know about a path
    lower: &'fs dyn VirtualFileSystem,
}

impl<'fs> OverlayFs<'fs> {
    /// Create a new overlay from two filesystems
    pub fn new(upper: &'fs dyn VirtualFileSystem, lower: &'fs dyn VirtualFileSystem) -> Self {
        Self { upper, lower }
    }
}

impl VirtualFileSystem for OverlayFs<'_> {
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<Arc<VirtualFile<'fs>>> {
        if let Ok(file) = self.upper.open(path) {
            Ok(file)
        } else if let Ok(file) = self.lower.open(path) {
            Ok(file)
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        if let Ok(metadata) = self.upper.metadata(path) {
            Ok(metadata)
        } else if let Ok(metadata) = self.lower.metadata(path) {
            Ok(metadata)
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn list_files(&self, path: &Path) -> Result<Vec<String>> {
        let mut paths = self.upper.list_files(path)?;
        paths.append(&mut self.lower.list_files(path)?);
        paths.sort_unstable();
        paths.dedup();
        Ok(paths)
    }

    fn exists(&self, path: &Path) -> bool {
        self.upper.exists(path) || self.lower.exists(path)
    }
}
