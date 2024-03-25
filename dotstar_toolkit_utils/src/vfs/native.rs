//! # Native Filesystem
//! This implements the virtual filesystem for the local filesystem (aka [`std::fs`])
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, File},
    io::{self, ErrorKind, Result},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, Weak},
    time::SystemTime,
};

use memmap2::Mmap;

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, WalkFs};

/// The native filesystem on this device
pub struct NativeFs {
    /// The root of this filesystem, no operations are allowed outside it
    root: PathBuf,
    /// Cache open files
    cache: Mutex<HashMap<PathBuf, Weak<Mmap>>>,
    list: OnceLock<Vec<PathBuf>>,
}

impl NativeFs {
    /// Create a new native filesystem with `root` as the root
    ///
    /// # Errors
    /// Will error if `root` does not exist
    pub fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            root: fs::canonicalize(root)?,
            cache: Mutex::new(HashMap::new()),
            list: OnceLock::new(),
        })
    }

    /// Create a canonical version of `path` with all relative things removed
    ///
    /// # Errors
    /// Will error if the path is outside the root or if the path does not exist
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
    ///
    /// # Errors
    /// Will error if it cannot read the error or files escape outside the root
    fn recursive_file_list(path: &Path, list: &mut Vec<PathBuf>) -> Result<()> {
        for entry in path.read_dir()?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::recursive_file_list(&path, list)?;
            } else if path.is_file() {
                list.push(path);
            }
        }
        Ok(())
    }
}

impl VirtualFileSystem for NativeFs {
    fn open(&self, path: &Path) -> std::io::Result<VirtualFile<'static>> {
        let path = self.canonicalize(path)?;
        let mut cache = self.cache.lock().unwrap();

        let data = match cache.entry(path) {
            Entry::Occupied(mut entry) => {
                if let Some(data) = entry.get().upgrade() {
                    data
                } else {
                    let file = File::open(entry.key())?;
                    let mmap = unsafe { Mmap::map(&file)? };
                    let data = Arc::new(mmap);
                    entry.insert(Arc::downgrade(&data));
                    data
                }
            }
            Entry::Vacant(entry) => {
                let file = File::open(entry.key())?;
                let mmap = unsafe { Mmap::map(&file)? };
                let data = Arc::new(mmap);
                entry.insert(Arc::downgrade(&data));
                data
            }
        };

        Ok(VirtualFile::Mmap(data))
    }

    fn metadata(&self, path: &Path) -> std::io::Result<VirtualMetadata> {
        let metadata = fs::metadata(self.canonicalize(path)?)?;
        let file_size = metadata.len();
        let created = metadata
            .created()
            .and_then(|st| {
                st.duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|_| ErrorKind::InvalidData.into())
            })
            .map(|d| d.as_secs())
            .map_err(|e| e.kind());
        Ok(VirtualMetadata { file_size, created })
    }

    fn walk_filesystem<'rf>(&'rf self, path: &Path) -> std::io::Result<WalkFs<'rf>> {
        let path = self.canonicalize(path)?;
        let list = self.list.get_or_try_init::<_, io::Error>(|| {
            let mut list = Vec::new();
            Self::recursive_file_list(&self.root, &mut list)?;
            Ok(list)
        })?;
        Ok(WalkFs {
            paths: list
                .iter()
                .filter_map(|p| p.strip_prefix(&path).ok())
                .collect(),
        })
    }

    fn exists(&self, path: &Path) -> bool {
        Self::canonicalize(self, path).is_ok()
    }
}
