//! `.alias8` files contain aliases for paths
//!
//! # File structure
//! ## Header
//! | Pos | Size | Type    | Id            | Description                   |
//! |-----|------|---------|---------------|-------------------------------|
//! | 0x0 | 4    | `u32be` | `unk1`        | Always 0x2                    |
//! | 0x4 | 4    | `u32be` | `num_aliases` | Number of aliases in the file |
//! | 0x8 | ...  | `Alias` | `aliases`     | Repeated `num_aliases` times  |
//!
//! ## Alias
//! | Pos | Size         | Type        | Id           | Description                                 |
//! |-----|--------------|-------------|--------------|---------------------------------------------|
//! | 0x0 | 4            | `u32be`     | `len_alias1` | The length of the first alias               |
//! | 0x4 | `len_alias1` | `String`    | `alias1`     | The first alias                             |
//! | ... | 4            | `u32be`     | `len_alias2` | The lenth of the second alias               |
//! | ... | `len_alias2` | `String`    | `alias2`     | The second alias                            |
//! | ... | ...          | `SplitPath` | `path`       | The path the aliases point to               |
//! | ... | 2            | `u16be`     | `unk2`       | Always 0xFFFF                               |
//! | ... | 2            | `u16be`     | `unk3`       | Unknown, possible values in [`Alias::UNK3`] |
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_panics_doc)]

mod parser;
mod types;
mod writer;

pub use parser::*;
pub use types::*;
pub use writer::*;
