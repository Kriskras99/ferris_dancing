//! A writer for the secure_fat.gf file format

use dotstar_toolkit_utils::bytes::{
    primitives::u32be,
    write::{BinarySerialize, WriteAt, WriteError},
};

use super::{SecureFat, MAGIC, UNK1};

impl BinarySerialize for SecureFat {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        sfat: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        // Write the header
        writer.write_at::<u32be>(position, MAGIC)?;
        writer.write_at::<u32be>(position, u32::from(sfat.game_platform()))?;
        writer.write_at::<u32be>(position, UNK1)?;

        // Write the amount of path IDs
        writer.write_at::<u32be>(position, u32::try_from(sfat.path_count())?)?;
        for (path_id, bundle_ids) in sfat.path_ids_and_bundle_ids() {
            // Write the path ID
            writer.write_at::<u32be>(position, u32::from(*path_id))?;
            // Write the amount of bundle IDs
            writer.write_at::<u32be>(position, u32::try_from(bundle_ids.len())?)?;
            // Write all the bundle ids
            for bundle_id in bundle_ids {
                writer.write_at::<u8>(position, u8::from(*bundle_id))?;
            }
        }

        // Write the amount of bundles
        writer.write_at::<u32be>(position, u32::try_from(sfat.bundle_count())?)?;
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            // write the bundle id
            writer.write_at::<u8>(position, u8::from(*bundle_id))?;
            // Write the length of the name and the name
            writer.write_len_string_at::<u32be>(position, name)?;
        }

        // Writing finished sucessfully
        Ok(())
    }
}
