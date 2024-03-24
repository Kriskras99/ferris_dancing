use std::{
    borrow::Cow,
    collections::hash_map::Entry,
    io::ErrorKind,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, Weak},
};

use dotstar_toolkit_utils::{
    bytes::read::{BinaryDeserialize, ReadError, TrivialClone, ZeroCopyReadAt},
    vfs::{VirtualFile, VirtualFileSystem, VirtualMetadata},
};
use nohash_hasher::IntMap;

use super::{Bundle, IpkFile};
use crate::utils::{path_id, PathId};

pub struct IpkFilesystem<'fs> {
    bundle: Bundle<'fs>,
    cache: Mutex<IntMap<PathId, Weak<Vec<u8>>>>,
    list: OnceLock<Vec<PathBuf>>,
}

#[derive(Debug, Clone)]
pub enum IpkFileS<'fs> {
    Uncompressed(&'fs [u8]),
    Compressed(Arc<Vec<u8>>),
}

impl TrivialClone for IpkFileS<'_> {}

impl<'fs> VirtualFile<'fs> for IpkFileS<'fs> {
    fn len(&self) -> usize {
        match self {
            IpkFileS::Uncompressed(data) => data.len(),
            IpkFileS::Compressed(data) => data.len(),
        }
    }
}

impl Deref for IpkFileS<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            IpkFileS::Uncompressed(data) => data,
            IpkFileS::Compressed(data) => data.as_slice(),
        }
    }
}

impl ZeroCopyReadAt for IpkFileS<'_> {
    fn read_null_terminated_string_at<'rf>(
        &'rf self,
        position: &mut u64,
    ) -> Result<Cow<'rf, str>, ReadError> {
        match self {
            IpkFileS::Uncompressed(data) => data.read_null_terminated_string_at(position),
            IpkFileS::Compressed(data) => data.read_null_terminated_string_at(position),
        }
    }

    fn read_slice_at<'rf>(
        &'rf self,
        position: &mut u64,
        len: usize,
    ) -> Result<Cow<'rf, [u8]>, ReadError> {
        match self {
            IpkFileS::Uncompressed(data) => data.read_slice_at(position, len),
            IpkFileS::Compressed(data) => data.read_slice_at(position, len),
        }
    }
}

impl<'fs> IpkFilesystem<'fs> {
    #[must_use]
    pub fn engine_version(&self) -> u32 {
        self.bundle.engine_version
    }

    #[must_use]
    pub fn unk4(&self) -> u32 {
        self.bundle.unk4
    }

    /// Create a new virtual filesystem from the IPK file at `path`.
    pub fn new(file: &'fs impl VirtualFile) -> Result<Self, std::io::Error> {
        let bundle = Bundle::deserialize(file).unwrap();
        Ok(Self {
            bundle,
            cache: Mutex::new(IntMap::default()),
            list: OnceLock::new(),
        })
    }
}

#[derive(Clone)]
pub struct IpkMetadata {
    pub timestamp: u64,
    pub is_cooked: bool,
    pub is_compressed: bool,
    pub size: u64,
}

impl VirtualMetadata for IpkMetadata {
    fn file_size(&self) -> u64 {
        self.size
    }

    fn created(&self) -> std::io::Result<u64> {
        Ok(self.timestamp)
    }
}

impl From<&IpkFile<'_>> for IpkMetadata {
    fn from(value: &IpkFile<'_>) -> Self {
        Self {
            timestamp: value.timestamp,
            is_cooked: value.is_cooked,
            is_compressed: matches!(value.data, super::Data::Compressed(_)),
            size: value.data.len(),
        }
    }
}

impl<'fs> VirtualFileSystem for IpkFilesystem<'fs> {
    type VirtualFile<'f> = IpkFileS<'f> where Self: 'f;
    type VirtualMetadata = IpkMetadata;

    fn open<'rf>(&'rf self, path: &Path) -> std::io::Result<Self::VirtualFile<'rf>> {
        let path_id = path_id(path);
        let file = self.bundle.files.get(&path_id).ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path:?}, file not found!"),
            )
        })?;
        match &file.data {
            super::Data::Uncompressed(data) => Ok(IpkFileS::Uncompressed(data.data.as_ref())),
            super::Data::Compressed(data) => {
                let mut cache = self.cache.lock().unwrap();
                match cache.entry(path_id) {
                    Entry::Occupied(mut entry) => {
                        if let Some(arc) = entry.get().upgrade() {
                            Ok(IpkFileS::Compressed(arc))
                        } else {
                            let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                            let mut decompress = flate2::Decompress::new(true);
                            decompress.decompress_vec(
                                data.data.as_ref(),
                                &mut vec,
                                flate2::FlushDecompress::Finish,
                            )?;
                            let arc = Arc::new(vec);
                            entry.insert(Arc::downgrade(&arc));
                            Ok(IpkFileS::Compressed(arc))
                        }
                    }
                    Entry::Vacant(entry) => {
                        let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                        let mut decompress = flate2::Decompress::new(true);
                        decompress.decompress_vec(
                            data.data.as_ref(),
                            &mut vec,
                            flate2::FlushDecompress::Finish,
                        )?;
                        let arc = Arc::new(vec);
                        entry.insert(Arc::downgrade(&arc));
                        Ok(IpkFileS::Compressed(arc))
                    }
                }
            }
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata> {
        let path_id = path_id(path);
        let file = self.bundle.files.get(&path_id);
        match file {
            Some(file) => Ok(IpkMetadata::from(file)),
            None => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not get metadata for {path:?}, file not found!"),
            )),
        }
    }

    fn list_files<'rf>(&'rf self, path: &Path) -> std::io::Result<impl Iterator<Item = &'rf Path>> {
        let list = self.list.get_or_init(|| {
            self.bundle
                .files
                .values()
                .map(|f| &f.path)
                .map(|p| {
                    let mut pb = PathBuf::with_capacity(p.len());
                    pb.push(p.path.as_ref());
                    pb.push(p.filename.as_ref());
                    pb
                })
                .collect()
        });

        Ok(list
            .iter()
            .filter(move |p| p.starts_with(path))
            .map(PathBuf::as_path))
    }

    fn exists(&self, path: &Path) -> bool {
        self.bundle.files.get(&path_id(path)).is_some()
    }
}
