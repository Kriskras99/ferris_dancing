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
#![allow(clippy::too_many_lines)]
// Doesn't work very well unk{n}
#![allow(clippy::similar_names)]
// #![allow(clippy::large_enum_variant)] // Broken
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::implicit_hasher)]
// TODO: Remove when new version of Yoke is released
#![allow(clippy::forget_non_drop)]
// Broken by serde_with
#![allow(clippy::multiple_crate_versions)]
// Significantly less readable than the original
#![allow(clippy::option_if_let_else)]
// Not reliable enough
#![allow(clippy::doc_markdown)]
// Would be nice to document everything, but enabling these adds 4000 errors
// #![deny(missing_docs)]
// #![deny(clippy::missing_docs_in_private_items)]

//! # UbiArt Toolkit
//! A library for parsing and writing various UbiArt files.
//! 
//! ## Features
//! This crate has one feature that can be enabled:
//! - `full_json_types`: Enable all JSON types (will increase compile times)
//!

pub mod alias8;
pub mod cooked;
pub mod ipk;
pub mod json_types;
pub mod loc8;
pub mod msm;
pub mod secure_fat;

pub mod utils;
