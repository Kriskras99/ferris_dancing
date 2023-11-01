#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// If an integer doesn't fit it's a good indication of a broken file
#![deny(clippy::as_conversions)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::missing_assert_message)]
// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// If an overflow or underflow occurs it's a good indication of a broken file
#![deny(clippy::arithmetic_side_effects)]

//! # Dolphin Toolkit
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
