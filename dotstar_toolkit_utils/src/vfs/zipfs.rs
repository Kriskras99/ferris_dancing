//! # Zip-backed Filesystem
//!
//! TODO: Create a custom zip implementation based on the ReadAt trait, maybe using rc-zip
use std::{
    collections::{hash_map::Entry, HashMap},
    io::{Error, ErrorKind, Read, Seek},
    sync::{Arc, Mutex, OnceLock, Weak},
};

use zip::{result::ZipError, ZipArchive};

use super::{VirtualFile, VirtualFileSystem, VirtualMetadata, VirtualPath, VirtualPathBuf, WalkFs};

/// A filesystem backed by a zip-file
#[derive(Debug)]
pub struct ZipFs<R: Read + Seek> {
    /// Maps paths to the files
    zip: Mutex<ZipArchive<R>>,
    /// Cache open files
    cache: Mutex<HashMap<VirtualPathBuf, Weak<Vec<u8>>>>,
    /// Cache the file paths
    list: OnceLock<Vec<VirtualPathBuf>>,
}

impl<R: Read + Seek + Send> ZipFs<R> {
    /// Create a new filesystem
    pub fn new(zipfile: R) -> Result<Self, Error> {
        Ok(Self {
            zip: Mutex::new(ZipArchive::new(zipfile)?),
            cache: Mutex::new(HashMap::new()),
            list: OnceLock::new(),
        })
    }

    #[allow(
        clippy::significant_drop_in_scrutinee,
        clippy::significant_drop_tightening,
        reason = "It's fine here"
    )]
    fn open_zipfile(&self, path: &str) -> std::io::Result<Vec<u8>> {
        let mut lock = self.zip.lock().map_err(|e| Error::other(e.to_string()))?;
        let data = match lock.by_name(path) {
            Ok(mut zipfile) => {
                // Preallocate compressed size as it is at least this big
                let mut vec =
                    Vec::with_capacity(usize::try_from(zipfile.size()).map_err(Error::other)?);
                zipfile.read_to_end(&mut vec).map_err(Error::other)?;
                Ok(vec)
            }
            Err(ZipError::FileNotFound) => Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path}"),
            )),
            Err(error) => Err(Error::other(error)),
        };
        data
    }

    fn get_list(&self) -> std::io::Result<&Vec<VirtualPathBuf>> {
        let paths = self.list.get_or_try_init(|| {
            Ok::<_, Error>(
                self.zip
                    .lock()
                    .map_err(|e| Error::other(e.to_string()))?
                    .file_names()
                    .map(VirtualPathBuf::from)
                    .collect(),
            )
        })?;
        Ok(paths)
    }
}

impl<R: Read + Seek + Send> VirtualFileSystem for ZipFs<R> {
    fn open<'fs>(&'fs self, path: &VirtualPath) -> std::io::Result<VirtualFile<'fs>> {
        let path = path.clean();

        #[allow(
            clippy::significant_drop_in_scrutinee,
            reason = "Can't reduce it further"
        )]
        let data = match self.cache.lock().unwrap().entry(path) {
            Entry::Occupied(mut entry) => {
                if let Some(data) = entry.get().upgrade() {
                    data
                } else {
                    let data = Arc::new(self.open_zipfile(entry.key().as_str())?);
                    entry.insert(Arc::downgrade(&data));
                    data
                }
            }
            Entry::Vacant(entry) => {
                let data = Arc::new(self.open_zipfile(entry.key().as_str())?);
                entry.insert(Arc::downgrade(&data));
                data
            }
        };

        Ok(VirtualFile::Vec(data))
    }

    #[allow(
        clippy::significant_drop_in_scrutinee,
        clippy::significant_drop_tightening,
        reason = "It's fine here"
    )]
    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata> {
        let path = path.clean();
        let mut lock = self.zip.lock().map_err(|e| Error::other(e.to_string()))?;
        let file_size = match lock.by_name(path.as_str()) {
            Ok(zipfile) => Ok(zipfile.size()),
            Err(ZipError::FileNotFound) => Err(Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path}"),
            )),
            Err(error) => Err(Error::other(error)),
        }?;
        Ok(VirtualMetadata {
            file_size,
            created: Err(ErrorKind::Unsupported),
        })
    }

    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>> {
        let paths = self.get_list()?;
        let path = path.clean();
        Ok(WalkFs {
            paths: paths
                .iter()
                .map(VirtualPathBuf::as_path)
                .filter(|p| p.starts_with(&path))
                .collect(),
        })
    }

    fn exists(&self, path: &VirtualPath) -> bool {
        let path = path.clean();
        self.get_list().is_ok_and(|v| v.contains(&path))
    }
}
