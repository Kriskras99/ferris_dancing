//! # Native Filesystem
//! This implements the virtual filesystem for the local filesystem (aka [`std::fs`])
use std::{
    fs::{self, OpenOptions},
    io::{self, Error, ErrorKind, Result},
    path::{Path, PathBuf},
    time::SystemTime,
};

use memmap2::Mmap;

use super::{VirtualFile, VirtualFileInner, VirtualFileMetadata, VirtualFileSystem};

/// The native filesystem on this device
pub struct Native {
    /// The root of this filesystem, no operations are allowed outside it
    root: PathBuf,
}

impl Native {
    /// Create a new native filesystem with `root` as the root
    ///
    /// # Errors
    /// Will error if `root` does not exist
    pub fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            root: fs::canonicalize(root)?,
        })
    }

    /// Create a canonical version of `path` with all relative things removed
    fn canonicalize(&self, path: &Path) -> Result<PathBuf> {
        let path = if path.starts_with(&self.root) {
            fs::canonicalize(path)?
        } else {
            fs::canonicalize(self.root.join(path))?
        };
        if path.starts_with(&self.root) {
            Ok(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{path:?} is outside root {:?}", self.root),
            ))
        }
    }

    /// Recursive way to get a full file list for a directory
    fn recursive_file_list(&self, path: &Path, list: &mut Vec<String>) -> Result<()> {
        for entry in path.read_dir()?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                self.recursive_file_list(&path, list)?;
            } else if path.is_file() {
                list.push(
                    path.strip_prefix(&self.root)
                        .map_err(|_| Error::from(ErrorKind::InvalidInput))?
                        .to_str()
                        .ok_or(ErrorKind::InvalidInput)?
                        .to_string(),
                );
            }
        }
        Ok(())
    }
}

impl VirtualFileSystem for Native {
    fn open(&self, path: &Path) -> std::io::Result<VirtualFile<'static>> {
        let path = Self::canonicalize(self, path)?;
        let mmap = unsafe { Mmap::map(&OpenOptions::new().read(true).open(path)?)? };
        let trait_object: Box<dyn VirtualFileInner> = Box::new(mmap);
        Ok(VirtualFile::from(trait_object))
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        let path = Self::canonicalize(self, path)?;
        Ok(Box::new(fs::metadata(path)?))
    }

    fn list_files(&self, path: &Path) -> Result<Vec<String>> {
        let path = Self::canonicalize(self, path)?;
        let mut paths = Vec::new();
        self.recursive_file_list(&path, &mut paths)?;
        Ok(paths)
    }

    fn exists(&self, path: &Path) -> bool {
        Self::canonicalize(self, path).map_or(false, |p| p.exists())
    }
}

impl VirtualFileMetadata for fs::Metadata {
    fn file_size(&self) -> u64 {
        self.len()
    }

    fn created(&self) -> Result<u64> {
        let time = Self::created(self)?;
        let duration = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ErrorKind::InvalidData)?;
        Ok(duration.as_secs())
    }
}

impl VirtualFileInner for Mmap {}
