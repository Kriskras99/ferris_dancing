//! # Native Filesystem
//! This implements the virtual filesystem for the local filesystem (aka [`std::fs`])
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, File},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, Weak},
    time::SystemTime,
};

use memmap2::Mmap;
use tracing::instrument;

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, VirtualPath, VirtualPathBuf, WalkFs};

/// The native filesystem on this device
pub struct NativeFs {
    /// The root of this filesystem, no operations are allowed outside it
    root: PathBuf,
    /// Cache open files
    cache: Mutex<HashMap<PathBuf, Weak<Mmap>>>,
    /// Cache the file paths
    list: OnceLock<Vec<VirtualPathBuf>>,
}

impl NativeFs {
    /// Create a new native filesystem with `root` as the root
    ///
    /// # Errors
    /// Will error if `root` does not exist
    #[instrument]
    pub fn new(root: &Path) -> std::io::Result<Self> {
        tracing::trace!("Created NativeFs");
        Ok(Self {
            root: root.canonicalize()?,
            cache: Mutex::new(HashMap::new()),
            list: OnceLock::new(),
        })
    }

    /// Create a canonical version of `path` with all relative things removed
    ///
    /// # Errors
    /// Will error if the path is outside the root or if the path does not exist
    fn canonicalize(&self, path: &VirtualPath) -> std::io::Result<PathBuf> {
        let mut clean = path.clean().into_string();
        if clean.starts_with('/') {
            clean.remove(0);
        }
        let path = self.root.join(clean);
        let path = path.canonicalize()?;
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
    fn recursive_file_list(root: &Path, current: &Path, list: &mut Vec<VirtualPathBuf>) -> std::io::Result<()> {
        for entry in current.read_dir()?.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::recursive_file_list(root, &path, list)?;
            } else if path.is_file() {
                let path = path.strip_prefix(root).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
                let path = VirtualPathBuf::try_from(path).map_err(|e| std::io::Error::new(ErrorKind::Other, e))?.clean();
                list.push(path);
            }
        }
        Ok(())
    }
}

impl VirtualFileSystem for NativeFs {
    fn open(&self, path: &VirtualPath) -> std::io::Result<VirtualFile<'static>> {
        let path = self.canonicalize(path)?;

        #[allow(
            clippy::significant_drop_in_scrutinee,
            reason = "Can't reduce it further"
        )]
        let data = match self.cache.lock().unwrap().entry(path) {
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

    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata> {
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

    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>> {
        let list = self.list.get_or_try_init::<_, io::Error>(|| {
            let mut list = Vec::new();
            Self::recursive_file_list(&self.root, &self.root, &mut list)?;
            Ok(list)
        })?;

        let paths = if path == "/" {
            list.iter()
                .map(VirtualPathBuf::as_path)
                .collect()
        } else {
            list.iter()
                .filter(|p| p.starts_with(&path))
                .map(VirtualPathBuf::as_path)
                .collect()
        };
        Ok(WalkFs { paths })
    }

    fn exists(&self, path: &VirtualPath) -> bool {
        Self::canonicalize(self, path).is_ok_and(|p| p.exists())
    }
}
