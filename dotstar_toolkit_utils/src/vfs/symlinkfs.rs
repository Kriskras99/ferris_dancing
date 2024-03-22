//! # Symlink Filesystem
//! This filesystem creates a new view of it's backing filesystem
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use super::{VirtualFile, VirtualFileMetadata, VirtualFileSystem};

// TODO: Add type alias for the PathBufs
/// A filesystem that maps paths to a backing filesystem
pub struct SymlinkFs<'fs> {
    /// The backing filesystem
    backing_fs: &'fs dyn VirtualFileSystem,
    /// Mapping of new_paths to paths on the backing filesystem
    mapping: HashMap<PathBuf, PathBuf>,
}

impl SymlinkFs<'_> {
    /// Add a file to this filesystem at path `new_path` that points to `orig_path` on the backing filesystem
    ///
    /// # Errors
    /// Will error if `new_path` already exists and does not point to `orig_path`
    /// Will error if `orig_path` does not exist
    pub fn add_file(&mut self, orig_path: PathBuf, new_path: PathBuf) -> Result<()> {
        if !self.backing_fs.exists(&orig_path) {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Failed to find original file '{orig_path:?}' while adding symlink to '{new_path:?}'"),
            ));
        }
        match self.mapping.entry(new_path) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                if entry.get() != &orig_path {
                    return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("Mapping already exists for '{:?}' and does not match! Existing symlink points to: '{:?}', new symlink would point to: '{orig_path:?}'", entry.key(), entry.get()),
                ));
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(orig_path);
            }
        }
        Ok(())
    }

    /// Merge this filesystem with another filesystem
    ///
    /// # Errors
    /// Will error if the other filesystem contains a conflicting mapping
    pub fn merge(&mut self, other: Self) -> Result<()> {
        for (new_path, orig_path) in other.mapping {
            self.add_file(orig_path, new_path)?;
        }

        Ok(())
    }

    /// Get the size of the entire filesystem
    ///
    /// # Errors
    /// - Backing filesystem fails on retrieving the metadata
    /// - The total size is larger than [`u64::MAX`]
    pub fn size(&self) -> Result<u64> {
        let mut size: u64 = 0;
        for path in self.mapping.values() {
            let metadata = self.backing_fs.metadata(path)?;
            size = size
                .checked_add(metadata.file_size())
                .ok_or_else(|| std::io::Error::other("Overflow occured"))?;
        }
        Ok(size)
    }
}

impl IntoIterator for SymlinkFs<'_> {
    type Item = (PathBuf, PathBuf);

    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        self.mapping.into_iter()
    }
}

impl<'fs> SymlinkFs<'fs> {
    /// Create a new filesystem with `backing_fs` as it's backend
    pub fn new(backing_fs: &'fs dyn VirtualFileSystem) -> Self {
        Self {
            backing_fs,
            mapping: HashMap::new(),
        }
    }

    /// Create a new filesystem with `backing_fs` as it's backend, with initial capacity for `capacity` files
    pub fn with_capacity(backing_fs: &'fs dyn VirtualFileSystem, capacity: usize) -> Self {
        Self {
            backing_fs,
            mapping: HashMap::with_capacity(capacity),
        }
    }
}

impl VirtualFileSystem for SymlinkFs<'_> {
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<VirtualFile<'fs>> {
        let actual_path = self
            .mapping
            .get(path)
            .ok_or_else(|| Error::from(ErrorKind::NotFound))?;
        self.backing_fs.open(actual_path)
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        let actual_path = self
            .mapping
            .get(path)
            .ok_or_else(|| Error::from(ErrorKind::NotFound))?;
        self.backing_fs.metadata(actual_path)
    }

    fn list_files(&self, path: &Path) -> Result<Vec<String>> {
        let mut files = Vec::with_capacity(self.mapping.keys().len());
        for file in self.mapping.keys() {
            if file.starts_with(path) {
                files.push(file.to_str().map(str::to_string).ok_or_else(|| {
                    Error::new(
                        ErrorKind::InvalidInput,
                        format!("Path is not valid utf-8: {file:?}"),
                    )
                })?);
            }
        }
        Ok(files)
    }

    fn exists(&self, path: &Path) -> bool {
        self.mapping.contains_key(path)
    }
}
