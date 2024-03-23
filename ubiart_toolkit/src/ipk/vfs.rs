use std::{io::ErrorKind, path::Path, sync::{Arc, Weak}};

use dotstar_toolkit_utils::vfs2::{VirtualFile, VirtualFileSystem};
use nohash_hasher::IntMap;
use yoke::Yoke;

use super::IpkFile;
use crate::utils::{path_id, PathId};

pub struct IpkFilesystem<'fs, VF: VirtualFile<'fs>> {
    ipk: Yoke<IntMap<PathId, IpkFile<'static>>, VF>,
    cache: IntMap<PathId, Weak<[u8]>>,
    engine_version: u32,
    unk4: u32,
}

pub enum IpkFileS<'fs> {
    Uncompressed(&'fs [u8]),
    Compressed(Arc<[u8]>),
}

impl<'fs, Fs: VirtualFileSystem> IpkFilesystem<'fs, Fs::VirtualFile<'fs>> {
    #[must_use]
    pub fn engine_version(&self) -> u32 {
        self.engine_version
    }

    #[must_use]
    pub fn unk4(&self) -> u32 {
        self.unk4
    }
    
    /// Create a new virtual filesystem from the IPK file at `path`.
    pub fn new(
        fs: &'fs Fs,
        path: &Path,
        lax: bool,
    ) -> Result<Self, std::io::Error> {
        let ipk_file = fs.open(path)?;
        let mut engine_version = 0;
        let mut unk4 = 0;
        let ipk: Yoke<super::Bundle<'_>, VirtualFile<'_>> =
            Yoke::try_attach_to_cart(ipk_file, |data: &[u8]| {
                let bundle = super::parse(data, lax)?;
                engine_version = bundle.engine_version;
                unk4 = bundle.unk4;
                bundle.files
            }).map_err(|e| std::io::Error::other(format!("Parsing of IPK failed: {e:?}")))?;
        Ok(Self {
            ipk,
            cache: IntMap::default(),
            engine_version,
            unk4,
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

// impl VirtualFileMetadata for IpkMetadata {
//     fn file_size(&self) -> u64 {
//         self.size
//     }

//     fn created(&self) -> std::io::Result<u64> {
//         Ok(self.timestamp)
//     }
// }

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

impl<'fs, Vf: VirtualFile<'fs>> VirtualFileSystem for IpkFilesystem<'fs, Vf> {
    type VirtualFile<'f> = IpkFileS<'f>;

    fn open(&self, path: &Path) -> std::io::Result<Self::VirtualFile<'fs>> {
        let path_id = path_id(path);
        let file = self.ipk.get().get(&path_id).ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path:?}, file not found!"),
            )
        })?;
        match &file.data {
            super::Data::Uncompressed(data) => Ok(data.data),
            super::Data::Compressed(data) => {
                let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                let mut decompress = flate2::Decompress::new(true);
                decompress.decompress_vec(data.data, &mut vec, flate2::FlushDecompress::Finish)?;
                Ok(VirtualFile::from(vec))
            }
        }
    }

    // fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
    //     let path_id = path_id(path);
    //     let file = self.ipk.get().files.get(&path_id);
    //     match file {
    //         Some(file) => Ok(Box::new(IpkMetadata::from(file))),
    //         None => Err(std::io::Error::new(
    //             ErrorKind::NotFound,
    //             format!("Could not get metadata for {path:?}, file not found!"),
    //         )),
    //     }
    // }

    fn list_files(&self, path: &Path) -> std::io::Result<impl Iterator<Item = &'fs Path>> {
        self.ipk.get().files()

        if let Some(path) = path.to_str() {
            Ok(self
                .ipk
                .get()
                .files
                .values()
                .map(|f| &f.path)
                .map(ToString::to_string)
                .filter_map(|s| {
                    let ss = s.strip_prefix('/').unwrap_or(s.as_str());
                    ss.starts_with(path).then(|| ss.to_string())
                })
                .collect())
        } else {
            Ok(Vec::new())
        }
    }

    fn exists(&self, path: &Path) -> bool {
        self.ipk.get().files.get(&path_id(path)).is_some()
    }
}
