#![feature(try_blocks)]
// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// If an overflow or underflow occurs it's a good indication of a broken file
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

pub mod u8a; // wii .app files
pub mod wad; // wii .wad files

/// Round address to the next boundary
///
/// # Panics
/// Will panic if the rounding would overflow
fn round_to_boundary(n: u32) -> u32 {
    n.checked_add(0x3F)
        .map(|n| n & (!0x3F))
        .expect("Overflow occurred!")
}
