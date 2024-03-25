#![feature(try_trait_v2)]
#![feature(error_generic_member_access)]
#![feature(lint_reasons)]
#![feature(try_blocks)]
#![feature(once_cell_try)]
// #![deny(missing_docs, reason = "Everything should be documented")]
// #![deny(
//     clippy::missing_docs_in_private_items,
//     reason = "Everything should be documented"
// )]
// #![deny(
//     clippy::arithmetic_side_effects,
//     reason = "If an overflow or underflow occurs it's a good indication that something broke"
// )]
#![allow(
    clippy::option_if_let_else,
    reason = "Significantly less readable than the original"
)]
#![allow(clippy::doc_markdown, reason = "Not reliable enough")]

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
