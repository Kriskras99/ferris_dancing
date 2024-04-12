//! A [`VirtualFileSystem`] implementation for [`SecureFat`]
//!
//! It will load the secure_fat.gf file and any IPK bundles listed therein plus the patch file.
use std::{collections::HashMap, io::ErrorKind};

use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserialize,
    vfs::{VirtualFile, VirtualFileSystem, VirtualMetadata, VirtualPath, WalkFs},
};

use super::{BundleId, SecureFat};
use crate::{
    ipk::vfs::IpkFilesystem,
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
    pub const fn unique_game_id(&self) -> UniqueGameId {
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

    /// Create a new virtual filesystem from a secure_fat.gf at `path`
    pub fn new(fs: &'f dyn VirtualFileSystem, path: &VirtualPath) -> std::io::Result<Self> {
        let sfat_file = fs.open(path).map_err(|error| {
            std::io::Error::other(format!("Failed to open {path:?}: {error:?}"))
        })?;
        let sfat = SecureFat::deserialize(&sfat_file).map_err(|error| {
            std::io::Error::other(format!("Failed to parse secure_fat.gf: {error:?}"))
        })?;
        if sfat.bundle_count() == 0 {
            return Err(std::io::Error::other(
                "secure_fat.gf does not have any IPKs",
            ));
        }
        let mut bundles = HashMap::with_capacity(sfat.bundle_count());
        let parent = path
            .parent()
            .ok_or_else(|| std::io::Error::from(ErrorKind::InvalidData))?;
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            let filename = super::bundle_name_to_filename(name, sfat.game_platform().platform);
            let path = parent.with_file_name(&filename);
            let ipk = IpkFilesystem::new(fs, &path).map_err(|error| {
                std::io::Error::other(format!("Failed to parse {path:?}: {error:?}"))
            })?;
            bundles.insert(*bundle_id, ipk);
        }
        let filename = super::bundle_name_to_filename("patch", sfat.game_platform().platform);
        let path = parent.with_file_name(filename);
        let patch = IpkFilesystem::new(fs, &path).ok();
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

impl<'fs> VirtualFileSystem for SfatFilesystem<'fs> {
    fn open<'f>(&'f self, path: &VirtualPath) -> std::io::Result<VirtualFile<'f>> {
        let path = path.clean();
        let path_id = path_id(&path);
        if let Some(file) = self.patch.as_ref().and_then(|p| p.open(&path).ok()) {
            Ok(file)
        } else {
            match self.sfat.get_bundle_ids(&path_id) {
                Some(ids) => {
                    for id in ids {
                        let bundle = self.bundles.get(id);
                        let file = bundle.and_then(|p| p.open(&path).ok());
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

    fn metadata(&self, path: &VirtualPath) -> std::io::Result<VirtualMetadata> {
        let path = path.clean();
        let path_id = path_id(&path);
        if let Some(metadata) = self.patch.as_ref().and_then(|p| p.metadata(&path).ok()) {
            Ok(metadata)
        } else {
            match self.sfat.get_bundle_ids(&path_id) {
                Some(ids) => {
                    for id in ids {
                        let bundle = self.bundles.get(id);
                        let metadata = bundle.and_then(|p| p.metadata(&path).ok());
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

    fn walk_filesystem<'rf>(&'rf self, path: &VirtualPath) -> std::io::Result<WalkFs<'rf>> {
        let path = path.clean();
        let mut walker = WalkFs::default();
        for bundle in self.bundles.values() {
            walker.merge(&bundle.walk_filesystem(&path)?);
        }
        if let Some(bundle) = self.patch.as_ref() {
            walker.merge(&bundle.walk_filesystem(&path)?);
        }
        Ok(walker)
    }

    fn exists(&self, path: &VirtualPath) -> bool {
        let path = path.clean();
        let path_id = path_id(&path);
        Some(true) == self.patch.as_ref().map(|p| p.exists(&path))
            || Some(true)
                == self.sfat.get_bundle_ids(&path_id).map(|bids| {
                    bids.iter()
                        .map(|bid| self.bundles.get(bid).map(|p| p.exists(&path)))
                        .any(|b| b == Some(true))
                })
    }
}
