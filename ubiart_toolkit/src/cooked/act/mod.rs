//! `.act.ckd` files contain the serialized form of an actor
//!
//! # File structure
//! ## Header
//! | Start    | End           |      Size | Description                         |
//! |----------|---------------|-----------|-------------------------------------|
//! | 0x0      | 0x4           |       u32 | unk0, always 0x1                    |
//! | 0x4      | 0x8           |       u32 | unk1                                |
//! | 0x8      | 0x10          |       u64 | unk2                                |
//! | 0x10     | 0x18          |       u64 | unk3, always 0x0                    |
//! | 0x18     | 0x20          |       u64 | unk4, always 0x0 or 0x1             |
//! | 0x20     | 0x24          |       u32 | unk5, always 0x0                    |
//! | 0x24     | 0x2c          |       u64 | unk6, always 0x0                    |
//! | 0x2c     | 0x34          |       u64 | unk7, always 0xFFFF_FFFF            |
//! | 0x34     | 0x38          |       u32 | unk8, always 0x0                    |
//! | 0x38     | 0x3c          |       u32 | l1, length of tpl file string       |
//! | 0x3c     | a = 0x3c + l1 |       u32 | tpl, template for the actor         |
//! | a        | a + 0x4       |       u32 | l2, length of tpl_dir string        |
//! | a + 0x4  | b = a + l2    |       u32 | tpl_dir, directory of template file |
//! | b        | b + 0x4       |       u32 | string_id, string_id(tpl_dir + tpl) |
//! | b + 0x4  | b + 0xc       |       u64 | unk9, always 0                      |
//! | b + 0xc  | b + 0x10      |       u32 | unk10                               |
//! | b + 0x10 | b + 0x14      |       u32 | tpl_id, unique id per template      |
//!
//! Depending on the template there is additional data, mostly containing file paths

mod parser;
mod types;
mod writer;

pub use types::*;
pub use writer::*;
