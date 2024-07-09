//! # Secure File Access Table (secure_fat.gf)
//!
//! This file is used by the UbiArt engine to quickly find files in the various [IPK bundle].
//! The file contains two maps. The first one maps a path ID to bundle IDs. The second one maps
//! a bundle ID to a name.
//!
//! ## Path IDs
//! Instead of using full paths in these files, 32-bit hashes of the paths are used. The implementation of this
//! hash is known as the [UbiArt CRC] implementation. Before calculating the hash of the path, the path is first
//! transformed into all caps.
//!
//! ## Bundle names
//! The bundle names that are contained in these files do not correspond 1:1 to the filename of the IPK bundle.
//! Instead, based on the platform, the capitalisation and postfix need to be added and changed. The .ipk
//! extension is also missing from the name. The [`bundle_name_to_filename`] function can be used to get the
//! filename.
//!
//! ## Limitations
//! A secure_fat.gf can only reference 256 bundles, as the bundle ID is only u8. This means that on FAT32 or similar
//! filesystems the game is limited to 1 TiB as a individual bundle is limited to 4 GiB.
//!
//! ## File structure
//! | Offset     | Type     | Description                                           |
//! | ---------- | -------- | ----------------------------------------------------- |
//! | 0          | b'USFT'  | Magic number                                          |
//! | 4          | u32      | Unique ID for the game, platform, and version         |
//! | 8          | u32      | Unknown, always 1                                     |
//! | 12         | u32      | The amount of path IDs (= p)                          |
//! | 16 + p     | u32      | path ID                                               |
//! | 16 + p + 4 | u32      | The amount of bundles this path can be found in (= q) |
//! | 16 + p + 8 | [u8; q]  | Bundle IDs where this path can be found               |
//! | n          | u32      | The amount of bundle IDs ( = b)                       |
//! | n + b + 4  | u8       | Bundle ID                                             |
//! | n + b + 5  | u32      | Length of the name in bytes ( = l)                    |
//! | n + b + 9  | [str; l] | Name                                                  |
//!
//! [IPK bundle]: crate::ipk::Bundle
//! [UbiArt CRC]: crate::utils::string_id
mod parser;
mod types;
pub mod vfs;
mod writer;

pub use types::*;

/// Probably short for UbiSoft FileTable
const MAGIC: u32 = u32::from_be_bytes(*b"USFT");
/// Example files where this value is not 0x1 are very welcome!
const UNK1: u32 = 0x1;
