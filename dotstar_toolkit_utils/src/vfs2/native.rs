use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    fs::{self, File},
    io::{self, ErrorKind},
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, Weak},
    time::SystemTime,
};

use memmap2::Mmap;

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata};
use crate::bytes_newer4::read::{ReadError, TrivialClone, ZeroCopyReadAt};

/// A completely in-memory filesystem, storing files as [`Vec`]s.
#[derive(Debug, Default)]
pub struct NativeFs {
    /// Maps paths to the files
    root: PathBuf,
    cache: Mutex<HashMap<PathBuf, Weak<Mmap>>>,
    list: OnceLock<Vec<PathBuf>>,
}

impl NativeFs {
    /// Create a new filesystem
    pub fn new(path: &Path) -> io::Result<Self> {
        Ok(Self {
            root: fs::canonicalize(path)?,
            cache: Mutex::new(HashMap::new()),
            list: OnceLock::new(),
        })
    }

    /// Create a canonical version of `path` with all relative things removed
    ///
    /// # Errors
    /// Will error if the path is outside the root or if the path does not exist
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf> {
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
    fn recursive_file_list(path: &Path, list: &mut Vec<PathBuf>) -> io::Result<()> {
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
    type VirtualFile<'fs> = NativeFile;
    type VirtualMetadata = fs::Metadata;

    fn open(&self, path: &Path) -> io::Result<Self::VirtualFile<'static>> {
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

        Ok(NativeFile { data })
    }

    fn list_files<'fs>(&'fs self, path: &Path) -> io::Result<impl Iterator<Item = &'fs Path>> {
        let path = self.canonicalize(path)?;
        let list = self.list.get_or_try_init::<_, io::Error>(|| {
            let mut list = Vec::new();
            Self::recursive_file_list(&self.root, &mut list)?;
            Ok(list)
        })?;

        Ok(list.iter().filter_map(move |p| p.strip_prefix(&path).ok()))
    }

    fn exists(&self, path: &Path) -> bool {
        Self::canonicalize(self, path).map_or(false, |p| p.exists())
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata> {
        fs::metadata(self.canonicalize(path)?)
    }
}

impl VirtualMetadata for fs::Metadata {
    fn file_size(&self) -> u64 {
        self.len()
    }

    fn created(&self) -> std::io::Result<u64> {
        let time = Self::created(self)?;
        let duration = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| ErrorKind::InvalidData)?;
        Ok(duration.as_secs())
    }
}

#[derive(Debug, Clone)]
pub struct NativeFile {
    data: Arc<Mmap>,
}

impl TrivialClone for NativeFile {}
impl ZeroCopyReadAt for NativeFile {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        self.data.read_null_terminated_string_at(position)
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        self.data.read_slice_at(position, len)
    }
}

impl Deref for NativeFile {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<'fs> VirtualFile<'fs> for NativeFile {
    fn len(&self) -> usize {
        self.data.len()
    }
}
