//! A [`VirtualFileSystem`] implementation for [`SecureFat`]
//!
//! It will load the secure_fat.gf file and any IPK bundles listed therein plus the patch file.
use std::{
    collections::HashMap,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserialize,
    vfs::{VirtualFile, VirtualFileSystem},
};

use super::{BundleId, SecureFat};
use crate::{
    ipk::vfs::{IpkFileS, IpkFilesystem, IpkMetadata},
    utils::{path_id, UniqueGameId},
};

pub struct SfatFilesystem<'f> {
    sfat: SecureFat,
    bundles: HashMap<BundleId, IpkFilesystem<'f>>,
    patch: Option<IpkFilesystem<'f>>,
}

impl<'f> SfatFilesystem<'f> {
    /// Get the `GamePlatform` value for this secure_fat.gf file
    #[must_use]
    pub const fn game_platform(&self) -> UniqueGameId {
        self.sfat.game_platform()
    }

    /// Get the highest engine version from the connected IPKs
    pub fn engine_version(&self) -> u32 {
        if let Some(patch) = &self.patch {
            patch.engine_version()
        } else {
            self.bundles
                .values()
                .map(IpkFilesystem::engine_version)
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
                .map(IpkFilesystem::unk4)
                .max()
                .unwrap_or_else(|| unreachable!())
        }
    }

    pub fn new1(
        file: &impl VirtualFile,
    ) -> std::io::Result<(SecureFat, HashMap<BundleId, PathBuf>, PathBuf)> {
        let sfat = SecureFat::deserialize(file).unwrap();
        if sfat.bundle_count() == 0 {
            return Err(std::io::Error::other(
                "secure_fat.gf does not have any IPKs",
            ));
        }
        let mut mappings = HashMap::new();
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            let filename = super::bundle_name_to_filename(name, sfat.game_platform().platform);
            let path = PathBuf::from(filename);
            mappings.insert(*bundle_id, path);
        }
        let patch = PathBuf::from(super::bundle_name_to_filename(
            "patch",
            sfat.game_platform().platform,
        ));
        Ok((sfat, mappings, patch))
    }

    pub fn new2(
        sfat: SecureFat,
        files: HashMap<BundleId, &'f impl VirtualFile>,
        patch: Option<&'f impl VirtualFile>,
    ) -> std::io::Result<Self> {
        if patch.is_none() {
            println!("Warning! No patch file found!");
        }
        let bundles = files
            .into_iter()
            .map(|(b, f)| (b, IpkFilesystem::new(f).unwrap()))
            .collect();
        let patch = patch.map(|f| IpkFilesystem::new(f).unwrap());

        Ok(Self {
            sfat,
            bundles,
            patch,
        })
    }
}

impl<'fs> VirtualFileSystem for SfatFilesystem<'fs> {
    type VirtualFile<'f> = IpkFileS<'f> where Self: 'f;
    type VirtualMetadata = IpkMetadata;

    fn open<'f>(&'f self, path: &Path) -> std::io::Result<Self::VirtualFile<'f>> {
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
                    Err(std::io::Error::new(ErrorKind::NotFound, format!("Could not open {path:?}, file is listed in file table but does not exist in bundle!")))
                }
                None => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    format!("Could not open {path:?}, file not found!"),
                )),
            }
        }
    }

    fn metadata(&self, path: &Path) -> std::io::Result<Self::VirtualMetadata> {
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
                    Err(std::io::Error::new(ErrorKind::NotFound, format!("Could not get metadata for {path:?}, file is listed in file table but does not exist in bundle!")))
                }
                None => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    format!("Could not get metadata for {path:?}, file not found!"),
                )),
            }
        }
    }

    fn list_files<'f>(&'f self, path: &Path) -> std::io::Result<impl Iterator<Item = &'f Path>> {
        let mut paths = Vec::new();
        for bundle in self.bundles.values() {
            paths.extend(bundle.list_files(path)?);
        }
        if let Some(bundle) = self.patch.as_ref() {
            paths.extend(bundle.list_files(path)?);
        }
        paths.sort_unstable();
        paths.dedup();
        paths.retain(|p| self.sfat.path_id_to_bundle_ids.contains_key(&path_id(p)));
        paths.shrink_to_fit();
        Ok(paths.into_iter().filter(move |p| p.starts_with(path)))
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
