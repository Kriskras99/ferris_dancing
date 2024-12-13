//! Contains the types that describe the useful information in a secure_fat.gf file

use std::ops::Deref;

use dotstar_toolkit_utils::bytes::read::{BinaryDeserialize, ReadAtExt, ReadError};
use nohash_hasher::{BuildNoHashHasher, IntMap, IsEnabled};

use crate::utils::{PathId, Platform, UniqueGameId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BundleId(u8);
impl BundleId {
    /// Increment the `BundleId` and return a new higher instance.
    ///
    /// # Panics
    /// Will panic if the increment would overflow the bundle id
    #[must_use]
    pub const fn increment(&self) -> Self {
        Self(
            self.0
                .checked_add(1)
                .expect("Increment would overflow BundleId!"),
        )
    }
}
impl Deref for BundleId {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<u8> for BundleId {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl From<BundleId> for u8 {
    fn from(value: BundleId) -> Self {
        value.0
    }
}
impl IsEnabled for BundleId {}

impl BinaryDeserialize<'_> for BundleId {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Ok(Self(reader.read_at::<u8>(position)?))
    }
}

/// Representation of secure_fat.gf
pub struct SecureFat {
    pub(super) game_platform: UniqueGameId,
    pub(super) path_id_to_bundle_ids: IntMap<PathId, Vec<BundleId>>,
    // TODO: Replace String with Cow, prerequisite: IntoOwned trait&derive
    pub(super) bundle_id_to_bundle_name: IntMap<BundleId, String>,
}

impl SecureFat {
    /// Create a new (empty) secure fat
    #[must_use]
    pub fn with_capacity(game_platform: UniqueGameId, capacity: usize) -> Self {
        Self {
            game_platform,
            path_id_to_bundle_ids: IntMap::with_capacity_and_hasher(
                capacity,
                BuildNoHashHasher::default(),
            ),
            bundle_id_to_bundle_name: IntMap::with_capacity_and_hasher(
                256,
                BuildNoHashHasher::default(),
            ),
        }
    }

    /// Add a new bundle with a name, returns the bundle id.
    ///
    /// # Panics
    /// Will panic if a bundle with that name already exists
    #[must_use]
    pub fn add_bundle(&mut self, name: String) -> BundleId {
        assert!(
            !self.bundle_id_to_bundle_name.values().any(|v| v == &name),
            "Bundle with name {name} already exists!"
        );
        let bundle_id = if self.bundle_id_to_bundle_name.is_empty() {
            BundleId::from(0)
        } else {
            let max = self
                .bundle_id_to_bundle_name
                .keys()
                .max()
                .unwrap_or_else(|| unreachable!());
            max.increment()
        };
        self.bundle_id_to_bundle_name.insert(bundle_id, name);
        bundle_id
    }

    /// Add path ids to a bundle
    pub fn add_path_ids_to_bundle(
        &mut self,
        bundle_id: BundleId,
        path_ids: impl IntoIterator<Item = PathId>,
    ) {
        for path_id in path_ids {
            match self.path_id_to_bundle_ids.entry(path_id) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(bundle_id);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(vec![bundle_id]);
                }
            }
        }
    }

    /// Returns the game platform
    #[must_use]
    pub const fn game_platform(&self) -> UniqueGameId {
        self.game_platform
    }

    /// Returns the number of bundle IDs
    #[must_use]
    pub fn bundle_count(&self) -> usize {
        self.bundle_id_to_bundle_name.len()
    }

    /// Get the bundle name for a bundle ID
    #[must_use]
    pub fn get_bundle_name(&self, bundle_id: &BundleId) -> Option<&str> {
        self.bundle_id_to_bundle_name
            .get(bundle_id)
            .map(String::as_str)
    }

    /// Get the bundle ids for a path ID
    #[must_use]
    pub fn get_bundle_ids(&self, path_id: &PathId) -> Option<&Vec<BundleId>> {
        self.path_id_to_bundle_ids.get(path_id)
    }

    /// An iterator visiting all bundle IDs with their respective name
    pub fn bundle_ids_and_names(&self) -> impl Iterator<Item = (&BundleId, &str)> {
        self.bundle_id_to_bundle_name
            .iter()
            .map(|(b, s)| (b, s.as_str()))
    }

    /// An iterator visiting all path IDs with their respective bundle ids
    pub fn path_ids_and_bundle_ids(&self) -> impl Iterator<Item = (&PathId, &Vec<BundleId>)> + '_ {
        self.path_id_to_bundle_ids.iter()
    }

    /// Returns the number of path IDs
    #[must_use]
    pub fn path_count(&self) -> usize {
        self.path_id_to_bundle_ids.len()
    }
}

/// Convert a bundle name to a filename based on the platform
#[must_use]
pub fn bundle_name_to_filename(name: &str, ugi: UniqueGameId) -> String {
    match ugi.platform {
        Platform::Nx => {
            format!("{name}_nx.ipk")
        }
        Platform::Win => {
            format!("{name}_pc.ipk")
        }
        Platform::Wii => {
            format!("{name}_wii.ipk")
        }
        Platform::WiiU => {
            format!("{name}_wiiu.ipk")
        }
        _ => todo!("Unsupported platform: {}", ugi.platform),
    }
}
