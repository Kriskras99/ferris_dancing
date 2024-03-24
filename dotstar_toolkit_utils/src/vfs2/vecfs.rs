use std::{
    borrow::Cow,
    collections::HashMap,
    io::{self, ErrorKind},
    ops::Deref,
    path::{Path, PathBuf},
};

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata};
use crate::bytes_newer4::read::{ReadError, TrivialClone, ZeroCopyReadAt};

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
    pub fn add_file(&mut self, path: impl Into<PathBuf>, content: Vec<u8>) -> io::Result<()> {
        let path = path.into();
        self._add_file(path, content)
    }

    /// Inner part of [`Self::add_file`] to prevent monomorphization blowing up
    ///
    /// # Errors
    /// Will return an error if the file already exists
    fn _add_file(&mut self, path: PathBuf, mut content: Vec<u8>) -> io::Result<()> {
        if self.files.contains_key(&path) {
            return Err(std::io::ErrorKind::AlreadyExists.into());
        }
        content.shrink_to_fit();
        self.files.insert(path, content);
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
    pub fn size(&self) -> io::Result<u64> {
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
    type VirtualFile<'fs> = VecFile<'fs>;
    type VirtualMetadata = VecMetadata;

    fn open<'fs>(&'fs self, path: &Path) -> io::Result<Self::VirtualFile<'fs>> {
        if let Some(file) = self.files.get(path) {
            Ok(VecFile {
                data: file.as_slice(),
            })
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }

    fn list_files<'fs>(&'fs self, path: &Path) -> io::Result<impl Iterator<Item = &'fs Path>> {
        Ok(self
            .files
            .keys()
            .filter(move |p| p.starts_with(path))
            .map(|p| p.as_path()))
    }

    fn exists(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata> {
        if let Some(file) = self.files.get(path) {
            Ok(VecMetadata {
                file_size: u64::try_from(file.len()).expect("File is bigger than u64!"),
            })
        } else {
            Err(ErrorKind::NotFound.into())
        }
    }
}

/// Metadata about a file in this filesystem
#[derive(Debug, Clone, Copy)]
pub struct VecMetadata {
    /// The size of the file
    file_size: u64,
}

impl VirtualMetadata for VecMetadata {
    fn file_size(&self) -> u64 {
        self.file_size
    }

    fn created(&self) -> io::Result<u64> {
        Err(ErrorKind::Unsupported.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VecFile<'fs> {
    data: &'fs [u8],
}

impl TrivialClone for VecFile<'_> {}
impl<'fs> ZeroCopyReadAt for VecFile<'fs> {
    fn read_null_terminated_string_at(
        &self,
        position: &mut u64,
    ) -> Result<Cow<'fs, str>, ReadError> {
        self.data.read_null_terminated_string_at(position)
    }

    fn read_slice_at(&self, position: &mut u64, len: usize) -> Result<Cow<'fs, [u8]>, ReadError> {
        self.data.read_slice_at(position, len)
    }
}

impl<'fs> Deref for VecFile<'fs> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'fs> VirtualFile<'fs> for VecFile<'fs> {
    fn len(&self) -> usize {
        self.data.len()
    }
}
