//! # Overlay Filesystem
//! Implements a filesystem that overlays two filesystems, preferring the upper filesystem for operations.

use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, VirtualPath, VirtualPathBuf, WalkFs};

/// A filesystem that overlays two filesystems, preferring the upper filesystem for operations.
pub struct CaseInsensitiveFs<'fs> {
    /// The wrapped filesystem
    fs: &'fs dyn VirtualFileSystem,
    /// Map of lowercase paths to the original path name
    paths: HashMap<VirtualPathBuf, &'fs VirtualPath>,
}

impl<'fs> CaseInsensitiveFs<'fs> {
    /// Create a new overlay from two filesystems
    pub fn new(fs: &'fs dyn VirtualFileSystem) -> std::io::Result<Self> {
        let walk_dir = fs.walk_filesystem(VirtualPath::new(""))?;
        let mut paths = HashMap::new();
        for path in walk_dir {
            let lowercase = path.to_string().to_lowercase();
            if let Some(existing_path) = paths.insert(lowercase.into(), path) {
                return Err(Error::new(ErrorKind::AlreadyExists, format!("Underlying filesystem has two paths that match when case-insenstive: {path}, {existing_path}")));
            }
        }

        Ok(Self { fs, paths })
    }
}

impl VirtualFileSystem for CaseInsensitiveFs<'_> {
    fn open<'fs>(&'fs self, path: &VirtualPath) -> std::io::Result<VirtualFile<'fs>> {
        let lower_path = path.to_string().to_lowercase();
        if let Some(path) = self.paths.get(&VirtualPathBuf::from(lower_path)) {
            self.fs.open(path)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path:?}, file not found!"),
            ))
        }
    }

    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata> {
        let lower_path = path.to_string().to_lowercase();
        if let Some(path) = self.paths.get(&VirtualPathBuf::from(lower_path)) {
            self.fs.metadata(path)
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not get metadata for {path:?}, file not found!"),
            ))
        }
    }

    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>> {
        let lower_path = path.to_string().to_lowercase();
        let paths = self
            .paths
            .keys()
            .map(VirtualPathBuf::as_path)
            .filter(|path| path.starts_with(&lower_path))
            .collect();
        Ok(WalkFs::new(paths))
    }

    fn exists(&self, path: &VirtualPath) -> bool {
        let lower_path = path.to_string().to_lowercase();
        self.paths.contains_key(&VirtualPathBuf::from(lower_path))
    }
}
