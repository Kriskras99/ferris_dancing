//! # Symlink Filesystem
//! This filesystem creates a new view of it's backing filesystem
use std::{
    collections::{hash_map::Entry, HashMap},
    io::{Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use path_clean::PathClean;

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, WalkFs};

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
    #[tracing::instrument(skip(self))]
    pub fn add_file(&mut self, orig_path: PathBuf, new_path: PathBuf) -> Result<()> {
        let clean_new_path = new_path.clean();
        if !self.backing_fs.exists(&orig_path) {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Failed to find original file '{orig_path:?}' while adding symlink to '{clean_new_path:?}'"),
            ));
        }
        match self.mapping.entry(clean_new_path) {
            Entry::Occupied(entry) => {
                if entry.get() != &orig_path {
                    return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    format!("Mapping already exists for '{:?}' and does not match! Existing symlink points to: '{:?}', new symlink would point to: '{orig_path:?}'", entry.key(), entry.get()),
                ));
                }
            }
            Entry::Vacant(entry) => {
                tracing::trace!("Adding {:?}", entry.key());
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

    fn metadata(&self, path: &Path) -> std::io::Result<VirtualMetadata> {
        let actual_path = self
            .mapping
            .get(path)
            .ok_or_else(|| Error::from(ErrorKind::NotFound))?;
        self.backing_fs.metadata(actual_path)
    }

    fn walk_filesystem<'rf>(&'rf self, path: &Path) -> std::io::Result<WalkFs<'rf>> {
        Ok(WalkFs {
            paths: self
                .mapping
                .keys()
                .filter(|p| p.starts_with(path))
                .map(PathBuf::as_path)
                .collect(),
        })
    }

    fn exists(&self, path: &Path) -> bool {
        self.mapping.contains_key(path)
    }
}
