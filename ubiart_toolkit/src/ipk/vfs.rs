#![allow(clippy::module_name_repetitions)]

use std::{
    io::{self, Cursor, ErrorKind},
    ops::Deref,
    path::Path,
};

use anyhow::Error;
use dotstar_toolkit_utils::vfs::{
    VirtualFile, VirtualFileInner, VirtualFileMetadata, VirtualFileSystem,
};
use flate2::read::ZlibDecoder;
use memmap2::{Mmap, MmapMut};
use stable_deref_trait::StableDeref;
use yoke::Yoke;

use crate::utils::{path_id, GamePlatform};

use super::IpkFile;

pub struct VfsIpkFilesystem<'f> {
    ipk: crate::ipk::BundleOwned<VirtualFile<'f>>,
}

impl VfsIpkFilesystem<'_> {
    #[must_use]
    pub fn game_platform(&self) -> GamePlatform {
        self.ipk.game_platform()
    }

    #[must_use]
    pub fn engine_version(&self) -> u32 {
        self.ipk.engine_version()
    }

    #[must_use]
    pub fn unk4(&self) -> u32 {
        self.ipk.unk4()
    }
}

impl<'f> VfsIpkFilesystem<'f> {
    /// Create a new virtual filesystem from the IPK file at `path`.
    ///
    /// # Errors
    /// Will error if the parsing of the IPK file fails or the file fails to open
    pub fn new(fs: &'f dyn VirtualFileSystem, path: &Path) -> Result<VfsIpkFilesystem<'f>, Error> {
        let ipk_file = fs.open(path)?;
        let yoke = Yoke::try_attach_to_cart(ipk_file, |data: &[u8]| super::parse(data))?;
        let ipk = crate::ipk::BundleOwned::from(yoke);
        Ok(Self { ipk })
    }
}

pub enum VfsIpkFile<'a> {
    Uncompressed(&'a [u8]),
    Compressed(Mmap),
}

impl Deref for VfsIpkFile<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            VfsIpkFile::Uncompressed(data) => data,
            VfsIpkFile::Compressed(data) => data,
        }
    }
}

impl<'f> VirtualFileInner for VfsIpkFile<'f> {}

unsafe impl StableDeref for VfsIpkFile<'_> {}

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

    fn created(&self) -> io::Result<u64> {
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
        let file = self
            .ipk
            .get_file(&path_id)
            .ok_or_else(|| io::Error::from(ErrorKind::NotFound))?;
        match file.data {
            super::Data::Uncompressed(data) => Ok(VirtualFile::from(data.data)),
            super::Data::Compressed(data) => {
                let mut mmap = MmapMut::map_anon(data.uncompressed_size)?;
                let mut decoder = ZlibDecoder::new(data.data);
                let mut cursor = Cursor::new(&mut *mmap);
                io::copy(&mut decoder, &mut cursor)?;
                let mmap = mmap.make_read_only()?;
                let trait_object: Box<dyn VirtualFileInner> = Box::new(mmap);
                Ok(VirtualFile::from(trait_object))
            }
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        let path_id = path_id(path);
        let file = self.ipk.get_file(&path_id);
        match file {
            Some(file) => Ok(Box::new(VfsIpkMetadata::from(file))),
            None => Err(ErrorKind::NotFound.into()),
        }
    }

    fn list_files(&self, path: &Path) -> io::Result<Vec<String>> {
        let mut files = self.ipk.list_files();
        let path = path.strip_prefix("/").unwrap_or(path);
        files.retain(|s| path.to_str().is_some_and(|p| s.starts_with(p)));
        Ok(files)
    }

    fn exists(&self, path: &Path) -> bool {
        self.ipk.get_file(&path_id(path)).is_some()
    }
}
