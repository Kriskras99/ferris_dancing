#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(try_blocks)]
#![feature(once_cell_try)]
// Would be nice to document everything, but enabling these adds 4000 errors
// #![deny(missing_docs)]
// #![deny(clippy::missing_docs_in_private_items)]

//! # UbiArt Toolkit
//! A library for parsing and writing various UbiArt files.
//!
//! ## Features
//! This crate has the following feature that can be enabled:
//! - `full_json_types`: Enable all JSON types (will increase compile times)
//! - `zopfli`: Enable compression with the Zopfli algorithm when creating IPK bundles
//!

pub mod alias8;
pub mod cooked;
pub mod ipk;
pub mod loc8;
pub mod msm;
pub mod secure_fat;
pub mod shared_json_types;

pub mod utils;
