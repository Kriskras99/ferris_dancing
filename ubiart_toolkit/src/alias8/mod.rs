//! `.alias8` files contain aliases for paths
//!
//! # File structure
//! ## Header
//! |      Size | Description           |
//! |-----------|-----------------------|
//! |       u32 | unk1, always 0x2      |
//! |       u32 | n, Number of aliases  |
//! | EOF - 0x8 | [Alias], see Alias    |
//!
//! ## Alias
//! | Start   | End              | Size | Description                       |
//! |---------|------------------|------|-----------------------------------|
//! | 0x0     | 0x8              |  u32 | l1, lenght of first alias         |
//! | 0x8     | a = 0x8 + l1     |   l1 | alias1, the first alias           |
//! | a       | a + 0x4          |  u32 | l2, length of second alias        |
//! | a + 0x4 | b = a + 0x4 + l2 |   l2 | alias2, the second alias          |
//! | b       | b + 0x4          |  u32 | l3, length of the filename        |
//! | a + 0x4 | c = a + 0x4 + l2 |   l2 | filename, the original filename   |
//! | c       | c + 0x4          |  u32 | l4, length of the path            |
//! | c + 0x4 | d = a + 0x4 + l2 |   l2 | path, the path of the file        |
//! | d       | d + 0x4          |  u32 | stringid, the id of path+filename |
//! | d + 0x4 | d + 0x8          |  u32 | unk1, always 0x1                  |
//! | d + 0xc | d + 0x10         |  u32 | unk2                              |
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]

mod parser;
mod types;
mod writer;

pub use parser::*;
pub use types::*;
pub use writer::*;
