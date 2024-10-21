use std::{
    collections::hash_map::Entry,
    io::ErrorKind,
    sync::{Arc, Mutex, OnceLock, Weak},
};

use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserializeExt as _,
    vfs::{VirtualFile, VirtualFileSystem, VirtualMetadata, VirtualPath, VirtualPathBuf, WalkFs},
};
use nohash_hasher::IntMap;
use tracing::instrument;
use yoke::Yoke;

use super::Bundle;
use crate::utils::{PathId, UniqueGameId};

pub struct IpkFilesystem<'fs> {
    bundle: Yoke<Bundle<'static>, VirtualFile<'fs>>,
    cache: Mutex<IntMap<PathId, Weak<Vec<u8>>>>,
    list: OnceLock<Vec<VirtualPathBuf>>,
}

impl<'fs> IpkFilesystem<'fs> {
    #[must_use]
    pub fn engine_version(&self) -> u32 {
        self.bundle.get().engine_version
    }

    #[must_use]
    pub fn unique_game_id(&self) -> UniqueGameId {
        self.bundle.get().game_platform
    }

    #[must_use]
    pub fn unk4(&self) -> u32 {
        self.bundle.get().unk4
    }

    /// Create a new virtual filesystem from the IPK file at `path`.
    #[instrument(skip(fs))]
    pub fn new(fs: &'fs dyn VirtualFileSystem, path: &VirtualPath) -> Result<Self, std::io::Error> {
        let file = fs.open(path)?;
        let bundle = Yoke::try_attach_to_cart(file, |data: &[u8]| Bundle::deserialize(data))
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))?;
        Ok(Self {
            bundle,
            cache: Mutex::new(IntMap::default()),
            list: OnceLock::new(),
        })
    }
}

impl VirtualFileSystem for IpkFilesystem<'_> {
    #[allow(
        clippy::significant_drop_in_scrutinee,
        reason = "Guard is needed in the entire match"
    )]
    #[instrument(skip(self))]
    fn open(&self, path: &VirtualPath) -> std::io::Result<VirtualFile> {
        let mut path = path.clean();
        if path.as_str().starts_with('/') {
            let mut string = path.into_string();
            string.remove(0);
            path = VirtualPathBuf::from(string);
        }
        let path_id = PathId::from(&path);
        let file = self.bundle.get().files.get(&path_id).ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path:?}, file not found!"),
            )
        })?;
        match &file.data {
            super::Data::Uncompressed(data) => Ok(VirtualFile::Slice(data.data.as_ref())),
            super::Data::Compressed(data) => {
                let mut cache = self.cache.lock().unwrap();
                match cache.entry(path_id) {
                    Entry::Occupied(mut entry) => {
                        if let Some(arc) = entry.get().upgrade() {
                            Ok(VirtualFile::Vec(arc))
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
                            Ok(VirtualFile::Vec(arc))
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
                        Ok(VirtualFile::Vec(arc))
                    }
                }
            }
        }
    }

    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata> {
        let mut path = path.clean();
        if path.as_str().starts_with('/') {
            let mut string = path.into_string();
            string.remove(0);
            path = VirtualPathBuf::from(string);
        }
        let path_id = PathId::from(&path);
        let file = self.bundle.get().files.get(&path_id);
        match file {
            Some(file) => Ok(VirtualMetadata {
                file_size: file.data.len(),
                created: Ok(file.timestamp),
            }),
            None => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not get metadata for {path:?}, file not found!"),
            )),
        }
    }

    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>> {
        let path = path.clean();
        let list = self.list.get_or_init(|| {
            self.bundle
                .get()
                .files
                .values()
                .map(|f| &f.path)
                .map(VirtualPathBuf::from)
                .map(|p| p.clean())
                .collect()
        });
        if path == VirtualPath::new("/") {
            Ok(WalkFs::new(
                list.iter().map(VirtualPathBuf::as_path).collect(),
            ))
        } else {
            Ok(WalkFs::new(
                list.iter()
                    .filter(|p| p.starts_with(&path))
                    .map(VirtualPathBuf::as_path)
                    .collect(),
            ))
        }
    }

    fn exists(&self, path: &VirtualPath) -> bool {
        let mut path = path.clean();
        if path.as_str().starts_with('/') {
            let mut string = path.into_string();
            string.remove(0);
            path = VirtualPathBuf::from(string);
        }
        self.bundle.get().files.contains_key(&PathId::from(path))
    }
}
