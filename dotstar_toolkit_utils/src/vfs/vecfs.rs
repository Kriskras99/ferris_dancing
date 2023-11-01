//! # Vec-backed Filesystem
//! This is a completely in-memory filesystem, storing files as [`Vec`]s.
use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
    ops::Deref,
    path::Path,
};

use stable_deref_trait::StableDeref;

use super::{VirtualFile, VirtualFileInner, VirtualFileMetadata, VirtualFileSystem};

/// A completely in-memory filesystem, storing files as [`Vec`]s.
#[derive(Debug, Clone, Default)]
pub struct VecFs {
    /// Maps paths to the files
    files: HashMap<String, Vec<u8>>,
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
    /// # Panics
    /// Will panic if there is already a file for `path`
    pub fn add_file(&mut self, path: String, mut content: Vec<u8>) {
        assert!(
            !self.files.contains_key(&path),
            "Path '{path}' already exists!"
        );
        content.shrink_to_fit();
        self.files.insert(path, content);
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
    /// # Panics
    /// Will panic if the total size does not fit in a `u64`
    pub fn size(&self) -> u64 {
        u64::try_from(self.files.values().map(Vec::len).sum::<usize>())
            .expect("Filesystem is bigger than the size of the universe!")
    }
}

impl IntoIterator for VecFs {
    type Item = (String, Vec<u8>);

    type IntoIter = std::collections::hash_map::IntoIter<String, Vec<u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

impl VirtualFileSystem for VecFs {
    fn open<'fs>(&'fs self, path: &Path) -> std::io::Result<VirtualFile<'fs>> {
        if let Some(file) = path.to_str().and_then(|s| self.files.get(s)) {
            let trait_object: &dyn VirtualFileInner = file;
            Ok(VirtualFile::Borrowed(trait_object))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        if let Some(file) = path.to_str().and_then(|s| self.files.get(s)) {
            Ok(Box::new(VecMetadata {
                file_size: u64::try_from(file.len()).expect("File is bigger than the universe!"),
            }))
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn list_files(&self, path: &Path) -> Result<Vec<String>> {
        let path = path.to_str().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("Path is not valid utf-8: {path:?}"),
            )
        })?;
        Ok(self
            .files
            .keys()
            .filter(|s| s.starts_with(path))
            .cloned()
            .collect())
    }

    fn exists(&self, path: &Path) -> bool {
        path.to_str().and_then(|s| self.files.get(s)).is_some()
    }
}

/// Metadata about a file in this filesystem
#[derive(Clone)]
pub struct VecMetadata {
    /// The size of the file
    file_size: u64,
}

impl VirtualFileMetadata for VecMetadata {
    fn file_size(&self) -> u64 {
        self.file_size
    }

    fn created(&self) -> Result<u64> {
        Err(ErrorKind::Unsupported.into())
    }
}

/// A file in this filesystem
pub struct VecFile<'f> {
    /// A reference to the file in the filesystem
    inner: &'f Vec<u8>,
}

impl<'f> Deref for VecFile<'f> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

unsafe impl<'f> StableDeref for VecFile<'f> {}
