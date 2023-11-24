#![allow(clippy::module_name_repetitions)]

use std::{io::ErrorKind, path::Path};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{
    VirtualFile, VirtualFileMetadata, VirtualFileSystem,
};
use yoke::Yoke;

use crate::utils::path_id;

use super::IpkFile;

pub struct VfsIpkFilesystem<'f> {
    ipk: Yoke<super::Bundle<'static>, VirtualFile<'f>>,
}

impl VfsIpkFilesystem<'_> {
    #[must_use]
    pub fn engine_version(&self) -> u32 {
        self.ipk.get().engine_version
    }

    #[must_use]
    pub fn unk4(&self) -> u32 {
        self.ipk.get().unk4
    }
}

impl<'f> VfsIpkFilesystem<'f> {
    /// Create a new virtual filesystem from the IPK file at `path`.
    ///
    /// # Errors
    /// Will error if the parsing of the IPK file fails or the file fails to open
    pub fn new(
        fs: &'f dyn VirtualFileSystem,
        path: &Path,
        lax: bool,
    ) -> Result<VfsIpkFilesystem<'f>, Error> {
        let ipk_file = fs.open(path)?;
        let ipk: Yoke<super::Bundle<'_>, VirtualFile<'_>> =
            Yoke::try_attach_to_cart(ipk_file, |data: &[u8]| super::parse(data, lax))?;
        Ok(Self { ipk })
    }
}

#[derive(Clone)]
pub struct VfsIpkMetadata {
    pub timestamp: u64,
    pub is_cooked: bool,
    pub is_compressed: bool,
    pub size: u64,
}

impl VirtualFileMetadata for VfsIpkMetadata {
    fn file_size(&self) -> u64 {
        self.size
    }

    fn created(&self) -> std::io::Result<u64> {
        Ok(self.timestamp)
    }
}

impl From<&IpkFile<'_>> for VfsIpkMetadata {
    fn from(value: &IpkFile<'_>) -> Self {
        Self {
            timestamp: value.timestamp,
            is_cooked: value.is_cooked,
            is_compressed: matches!(value.data, super::Data::Compressed(_)),
            size: value.data.len(),
        }
    }
}

impl<'fs> VirtualFileSystem for VfsIpkFilesystem<'fs> {
    fn open<'f>(&'f self, path: &Path) -> std::io::Result<VirtualFile<'f>> {
        let path_id = path_id(path);
        let file = self.ipk.get().files.get(&path_id).ok_or_else(|| {
            std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not open {path:?}, file not found!"),
            )
        })?;
        match file.data {
            super::Data::Uncompressed(data) => Ok(VirtualFile::from(data.data)),
            super::Data::Compressed(data) => {
                let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                let mut decompress = flate2::Decompress::new(true);
                decompress.decompress_vec(data.data, &mut vec, flate2::FlushDecompress::Finish)?;
                Ok(VirtualFile::from(vec))
            }
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        let path_id = path_id(path);
        let file = self.ipk.get().files.get(&path_id);
        match file {
            Some(file) => Ok(Box::new(VfsIpkMetadata::from(file))),
            None => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("Could not get metadata for {path:?}, file not found!"),
            )),
        }
    }

    fn list_files(&self, path: &Path) -> std::io::Result<Vec<String>> {
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
