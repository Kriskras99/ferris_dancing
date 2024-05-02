//! A parser for the secure_fat.gf file format

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    testing::{test_eq, test_le, test_ne},
};
use nohash_hasher::{BuildNoHashHasher, IntMap};

use super::{BundleId, SecureFat, MAGIC, UNK1};
use crate::utils::{PathId, UniqueGameId};

impl BinaryDeserialize<'_> for SecureFat {
    fn deserialize_at(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        // Read the header
        let magic = reader.read_at::<u32be>(position)?.into();
        test_eq(&magic, &MAGIC)?;
        let game_platform = reader.read_at::<UniqueGameId>(position)?;
        let unk1 = reader.read_at::<u32be>(position)?.into();
        test_eq(&unk1, &UNK1)?;

        // Read how many path IDs there are and prepare a map
        let path_id_count = usize::try_from(reader.read_at::<u32be>(position)?)?;
        let mut path_id_to_bundle_ids =
            IntMap::with_capacity_and_hasher(path_id_count, BuildNoHashHasher::default());

        for _ in 0..path_id_count {
            // Read path ID
            let path_id = reader.read_at::<PathId>(position)?;

            // Read how many bundles this path is in
            let bundle_count = usize::try_from(reader.read_at::<u32be>(position)?)?;
            test_ne(&bundle_count, &0).context("Bundle count is zero!")?;

            // Read the bundle ids
            let mut bundle_ids = Vec::with_capacity(bundle_count);
            for _ in 0..bundle_count {
                bundle_ids.push(BundleId::from(reader.read_at::<u8>(position)?));
            }

            // Add to the map
            path_id_to_bundle_ids.insert(path_id, bundle_ids);
        }

        // Read how many bundles there are and prepare a map
        let bundle_count = usize::try_from(reader.read_at::<u32be>(position)?)?;
        test_le(&bundle_count, &0xFF)?;
        let mut bundle_id_to_bundle_name =
            IntMap::with_capacity_and_hasher(bundle_count, BuildNoHashHasher::default());

        for _ in 0..bundle_count {
            // Read the bundle ID
            let bundle_id = BundleId::from(reader.read_at::<u8>(position)?);

            // Read the name
            let name = String::from(reader.read_len_string_at::<u32be>(position)?);

            // Add to the map
            bundle_id_to_bundle_name.insert(bundle_id, name);
        }

        Ok(Self {
            game_platform,
            path_id_to_bundle_ids,
            bundle_id_to_bundle_name,
        })
    }
}
