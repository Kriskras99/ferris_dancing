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
