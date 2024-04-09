//! # Vec-backed Filesystem
//! This is a completely in-memory filesystem, storing files as [`Vec`]s.
use std::{
    collections::HashMap,
    io::{ErrorKind, Result},
    path::{Path, PathBuf},
};

use path_clean::PathClean;

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, WalkFs};

/// A completely in-memory filesystem, storing files as [`Vec`]s.
#[derive(Debug, Clone, Default)]
pub struct VecFs {
    /// Maps paths to the files
    files: HashMap<PathBuf, Vec<u8>>,
}

impl VecFs {
    /// Create a new filesystem
    #[must_use]
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Create a new filesystem with initial capacity of `capacity` files.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            files: HashMap::with_capacity(capacity),
        }
    }

    /// Add a file to the filesystem
    ///
    /// # Errors
    /// Will return an error if the file already exists
    #[tracing::instrument(skip(self, content))]
    pub fn add_file(&mut self, path: PathBuf, mut content: Vec<u8>) -> std::io::Result<()> {
        let clean_path = path.clean();
        if self.files.contains_key(&clean_path) {
            return Err(std::io::ErrorKind::AlreadyExists.into());
        }
        content.shrink_to_fit();

        tracing::trace!("Adding {clean_path:?} of size {}", content.len());
        self.files.insert(clean_path, content);
        Ok(())
    }

    /// Merge this filesystem with another filesystem.
    ///
    /// This overwrites existing files with files from the other filesystem
    /// if they have matching paths.
    pub fn merge(&mut self, other: Self) {
        self.files.extend(other.files);
    }

    /// Get the size of the entire filesystem
    ///
    /// # Errors
    /// - The total size is larger than [`u64::MAX`]
    pub fn size(&self) -> Result<u64> {
        u64::try_from(self.files.values().map(Vec::len).sum::<usize>())
            .map_err(|_| std::io::Error::other("Overflow occured"))
    }
}

impl IntoIterator for VecFs {
    type Item = (PathBuf, Vec<u8>);

    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, Vec<u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl VirtualFileSystem for VecFs {
    fn open<'fs>(&'fs self, path: &Path) -> Result<VirtualFile<'fs>> {
        let path = path.clean();
        if let Some(file) = self.files.get(&path) {
            Ok(VirtualFile::Slice(file.as_slice()))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<VirtualMetadata> {
        let path = path.clean();
        if let Some(file) = self.files.get(&path) {
            Ok(VirtualMetadata {
                file_size: u64::try_from(file.len()).expect("Overflow"),
                created: Err(ErrorKind::Unsupported),
            })
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn walk_filesystem<'rf>(&'rf self, path: &Path) -> std::io::Result<WalkFs<'rf>> {
        let path = path.clean();
        if path == Path::new(".") {
            Ok(WalkFs {
                paths: self.files.keys().map(PathBuf::as_path).collect(),
            })
        } else {
            Ok(WalkFs {
                paths: self
                    .files
                    .keys()
                    .filter(|p| p.starts_with(&path))
                    .map(PathBuf::as_path)
                    .collect(),
            })
        }
    }

    fn exists(&self, path: &Path) -> bool {
        let path = path.clean();
        self.files.contains_key(&path)
    }
}
