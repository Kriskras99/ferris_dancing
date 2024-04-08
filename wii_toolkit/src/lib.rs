#![feature(try_blocks)]
#![feature(lint_reasons)]
// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::arithmetic_side_effects)]

//! # Wii Toolkit
//!
//! This is a library for parsing Nintendo Wii files.
//! Currently supported are:
//! | File format | Extension | Supported                        |
//! | U8          | .app      | yes                              |
//! | WAD         | .wad      | Partially, only Installable WADs |
//!
//! ## Features
//! This crate has no features that can be enabled
//!

use dotstar_toolkit_utils::bytes::primitives::u32be;

pub mod u8a; // wii .app files
pub mod wad; // wii .wad files

/// Round address to the next boundary
///
/// # Panics
/// Will panic if the rounding would overflow
fn round_to_boundary(n: u32be) -> u32be {
    n.checked_add(u32be::from(0x3F))
        .map(|n| n & (!u32be::from(0x3F)))
        .expect("Overflow occurred!")
}
