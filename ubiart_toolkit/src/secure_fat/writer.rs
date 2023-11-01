//! A writer for the secure_fat.gf file format

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use byteorder::{BigEndian, WriteBytesExt};

use anyhow::Error;

use super::{SecureFat, MAGIC, UNK1};

/// Create a secure_fat.gf file at the path
///
/// # Errors
/// This function errors in various ways:
/// - The file cannot be opened
/// - Something goes wrong with writing to the writer
/// - There are too many bundles (more than 256)
pub fn create<P: AsRef<Path>>(path: P, sfat: &SecureFat) -> Result<(), Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    write(writer, sfat)
}

/// Write a secure_fat.gf file to the writer.
///
/// # Errors
/// This function errors in various ways:
/// - Something goes wrong with writing to the writer
/// - There are too many bundles (more than 256)
pub fn write<W: Write>(mut writer: W, sfat: &SecureFat) -> Result<(), Error> {
    // Write the header
    writer.write_u32::<BigEndian>(MAGIC)?;
    writer.write_u32::<BigEndian>(u32::from(sfat.game_platform()))?;
    writer.write_u32::<BigEndian>(UNK1)?;

    // Write the amount of path IDs
    writer.write_u32::<BigEndian>(u32::try_from(sfat.path_count())?)?;
    for (path_id, bundle_ids) in sfat.path_ids_and_bundle_ids() {
        // Write the path ID
        writer.write_u32::<BigEndian>(u32::from(*path_id))?;
        // Write the amount of bundle IDs
        writer.write_u32::<BigEndian>(u32::try_from(bundle_ids.len())?)?;
        // Write all the bundle ids
        for bundle_id in bundle_ids {
            writer.write_u8(u8::from(*bundle_id))?;
        }
    }

    // Write the amount of bundles
    writer.write_u32::<BigEndian>(u32::try_from(sfat.bundle_count())?)?;
    for (bundle_id, name) in sfat.bundle_ids_and_names() {
        // write the bundle id
        writer.write_u8(u8::from(*bundle_id))?;
        // Write the length of the name
        writer.write_u32::<BigEndian>(u32::try_from(name.len())?)?;
        // Write the name
        writer.write_all(name.as_bytes())?;
    }

    // Writing finished sucessfully
    Ok(())
}
