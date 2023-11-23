//! A parser for the secure_fat.gf file format

use anyhow::Error;
use byteorder::BigEndian;
use dotstar_toolkit_utils::testing::{test, test_le, test_not};
use nohash_hasher::{BuildNoHashHasher, IntMap};

use crate::utils::{
    bytes::{read_string_at, read_u32_at, read_u8_at},
    GamePlatform, PathId,
};

use super::{BundleId, SecureFat, MAGIC, UNK1};

/// Parse a bytearray-like source as a secure_fat.gf
///
/// This will parse the source from start to end.
///
/// # Errors
/// This function will error when it encounters the following:
/// - Unexpected values (i.e. wrong magic)
/// - Invalid UTF-8 (i.e. in bundle names)
/// - Source has an unexpected size (i.e. not enough bytes, or too many bytes)
pub fn parse(src: &[u8]) -> Result<SecureFat, Error> {
    // Keep track of where we are
    let mut pos = 0;
    // Read the header
    let magic = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&magic, &MAGIC)?;
    let game_platform = GamePlatform::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let unk1 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&unk1, &UNK1)?;

    // Read how many path IDs there are and prepare a map
    let path_id_count = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let mut path_id_to_bundle_ids =
        IntMap::with_capacity_and_hasher(path_id_count, BuildNoHashHasher::default());

    for _ in 0..path_id_count {
        // Read path ID
        let path_id = PathId::from(read_u32_at::<BigEndian>(src, &mut pos)?);

        // Read how many bundles this path is in
        let bundle_count = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
        test_not(&bundle_count, &0).context("Bundle count is zero!")?;

        // Read the bundle ids
        let mut bundle_ids = Vec::with_capacity(bundle_count);
        for _ in 0..bundle_count {
            bundle_ids.push(BundleId::from(read_u8_at(src, &mut pos)?));
        }

        // Add to the map
        path_id_to_bundle_ids.insert(path_id, bundle_ids);
    }

    // Read how many bundles there are and prepare a map
    let bundle_count = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    test_le(&bundle_count, &0xff)?;
    let mut bundle_id_to_bundle_name =
        IntMap::with_capacity_and_hasher(bundle_count, BuildNoHashHasher::default());

    for _ in 0..bundle_count {
        // Read the bundle ID
        let bundle_id = BundleId::from(read_u8_at(src, &mut pos)?);

        // Read the name
        let name = String::from(read_string_at::<BigEndian>(src, &mut pos)?);

        // Add to the map
        bundle_id_to_bundle_name.insert(bundle_id, name);
    }

    // Make sure we're at the end of the file
    test(&src.len(), &pos)?;

    Ok(SecureFat {
        game_platform,
        path_id_to_bundle_ids,
        bundle_id_to_bundle_name,
    })
}
