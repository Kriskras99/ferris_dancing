mod parser;
mod types;
pub mod vfs;
mod writer;

pub use types::*;
pub use writer::*;

/*
IPK limits:
Max number of files in archive is u32::MAX (4.294.967.295)
Max size of individual files is 4 GiB
On FAT32 the file size of the archive is also max 4 GiB
On NTFS and other 64-bit filesystems the max archive size is 16 EiB

 */

const MAGIC: u32 = 0x50EC_12BA;
const UNK1: &[u32; 2] = &[0x0, 0x1];
const UNK2: &[u32; 2] = &[0x0, 0x1];
const UNK3: &[u32; 2] = &[0x0, 0x1];
const UNK6: u32 = 0x1;
const IS_COOKED: &[u32; 2] = &[0x0, 0x2];
const SEPARATOR: u32 = 0x0;
