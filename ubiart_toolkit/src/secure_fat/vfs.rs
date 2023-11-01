#![allow(clippy::module_name_repetitions)]

use std::{io::ErrorKind, path::Path};

use anyhow::{Context, Error};
use dotstar_toolkit_utils::vfs::{VirtualFile, VirtualFileMetadata, VirtualFileSystem};
use nohash_hasher::{BuildNoHashHasher, IntMap};

use crate::{
    ipk::vfs::VfsIpkFilesystem,
    utils::{path_id, GamePlatform},
};

use super::{BundleId, SecureFat};

pub struct VfsSfatFilesystem<'f> {
    sfat: SecureFat,
    bundles: IntMap<BundleId, VfsIpkFilesystem<'f>>,
    patch: Option<VfsIpkFilesystem<'f>>,
}

impl VfsSfatFilesystem<'_> {
    /// Get the `GamePlatform` value for this secure_fat.gf file
    #[must_use]
    pub const fn game_platform(&self) -> GamePlatform {
        self.sfat.game_platform()
    }

    /// Get the highest engine version from the connected IPKs
    pub fn engine_version(&self) -> u32 {
        if let Some(patch) = &self.patch {
            patch.engine_version()
        } else {
            self.bundles
                .values()
                .map(VfsIpkFilesystem::engine_version)
                .max()
                .unwrap_or_else(|| unreachable!())
        }
    }

    /// Get the highest IPK unk4 value from the connected IPKs
    pub fn ipk_unk4(&self) -> u32 {
        if let Some(patch) = &self.patch {
            patch.unk4()
        } else {
            self.bundles
                .values()
                .map(VfsIpkFilesystem::unk4)
                .max()
                .unwrap_or_else(|| unreachable!())
        }
    }
}

impl<'f> VfsSfatFilesystem<'f> {
    /// Create a new virtual filesystem from a secure_fat.gf at `path`
    ///
    /// # Errors
    /// Will error if the file does not exist or if the .gf/.ipk files are invalid
    ///
    /// # Panics
    /// Will panic if the secure_fat.gf file does not reference any IPKs
    pub fn new(fs: &'f dyn VirtualFileSystem, path: &Path) -> Result<Self, Error> {
        let sfat_file = fs
            .open(path)
            .with_context(|| format!("Failed to open {path:?}"))?;
        let sfat = super::parse(&sfat_file)?;
        assert!(
            sfat.bundle_count() >= 1,
            "secure_fat.gf does not have any IPKs"
        );
        let mut bundles =
            IntMap::with_capacity_and_hasher(sfat.bundle_count(), BuildNoHashHasher::default());
        let parent = path
            .parent()
            .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidData))?;
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            let filename = super::bundle_name_to_filename(name, sfat.game_platform().platform);
            let path = parent.with_file_name(&filename);
            let ipk = VfsIpkFilesystem::new(fs, &path)
                .with_context(|| format!("Failed to open or parse {path:?}"))?;
            bundles.insert(*bundle_id, ipk);
        }
        let filename = super::bundle_name_to_filename("patch", sfat.game_platform().platform);
        let path = parent.with_file_name(filename);
        let patch = VfsIpkFilesystem::new(fs, &path).ok();
        if patch.is_none() {
            println!("Warning! No patch file found!");
        }

        Ok(Self {
            sfat,
            bundles,
            patch,
        })
    }
}

impl<'fs> VirtualFileSystem for VfsSfatFilesystem<'fs> {
    fn open<'f>(&'f self, path: &Path) -> std::io::Result<VirtualFile<'f>> {
        let path_id = path_id(path);
        if let Some(file) = self.patch.as_ref().and_then(|p| p.open(path).ok()) {
            Ok(file)
        } else {
            match self.sfat.get_bundle_ids(&path_id) {
                Some(ids) => {
                    for id in ids {
                        let bundle = self.bundles.get(id);
                        let file = bundle.and_then(|p| p.open(path).ok());
                        if let Some(file) = file {
                            return Ok(file);
                        }
                    }
                    println!(
                        "File found in secure_fat.gf but does not exist in the specified bundles!"
                    );
                    Err(ErrorKind::NotFound.into())
                }
                None => Err(ErrorKind::NotFound.into()),
            }
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Box<dyn VirtualFileMetadata>> {
        let path_id = path_id(path);
        if let Some(metadata) = self.patch.as_ref().and_then(|p| p.metadata(path).ok()) {
            Ok(metadata)
        } else {
            match self.sfat.get_bundle_ids(&path_id) {
                Some(ids) => {
                    for id in ids {
                        let bundle = self.bundles.get(id);
                        let metadata = bundle.and_then(|p| p.metadata(path).ok());
                        if let Some(metadata) = metadata {
                            return Ok(metadata);
                        }
                    }
                    println!(
                        "File found in secure_fat.gf but does not exist in the specified bundles!"
                    );
                    Err(ErrorKind::NotFound.into())
                }
                None => Err(ErrorKind::NotFound.into()),
            }
        }
    }

    fn list_files(&self, path: &Path) -> std::io::Result<Vec<String>> {
        let mut paths = Vec::with_capacity(self.sfat.path_count());
        for bundle in self.bundles.values() {
            paths.append(&mut bundle.list_files(path)?);
            paths.sort_unstable();
            paths.dedup();
        }
        paths.retain(|p| self.sfat.path_id_to_bundle_ids.contains_key(&path_id(p)));
        Ok(paths)
    }

    fn exists(&self, path: &Path) -> bool {
        let path_id = path_id(path);
        Some(true) == self.patch.as_ref().map(|p| p.exists(path))
            || Some(true)
                == self.sfat.get_bundle_ids(&path_id).map(|bids| {
                    bids.iter()
                        .map(|bid| self.bundles.get(bid).map(|p| p.exists(path)))
                        .any(|b| b == Some(true))
                })
    }
}
