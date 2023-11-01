#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
// If an integer doesn't fit it's a good indication that something broke
#![deny(clippy::as_conversions)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::get_unwrap)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::missing_assert_message)]
// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
// If an overflow or underflow occurs it's a good indication that something broke
#![deny(clippy::arithmetic_side_effects)]
// Significantly less readable than the original
#![allow(clippy::option_if_let_else)]
// Not reliable enough
#![allow(clippy::doc_markdown)]

//! # .* Toolkit Utils
//! This library contains various utilities for writing parsers.
//! It contains three sections:
//! 1. [`bytes`]: contains Byteorder like functions for reading integers and strings from byte slices.
//! 2. [`testing`]: contains alternatives to the `assert!` family that return `Result`s instead of panicking.
//! 3. [`vfs`]: contains traits for a virtual filesystem and some basic filesystems that allow for parsing without extracting
//! 
//! ## Features
//! This crate has no features that can be enabled
//!

pub mod bytes;
pub mod testing;
pub mod vfs;
