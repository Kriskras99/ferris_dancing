#![feature(lint_reasons)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(try_blocks)]
#![allow(
    clippy::too_many_lines,
    reason = "Parsers and writers have a lot of lines"
)]
#![allow(
    clippy::similar_names,
    reason = "Doesn't work very well with unk{n} naming convention"
)]
#![allow(clippy::multiple_crate_versions, reason = "Broken by serde_with")]
#![allow(
    clippy::option_if_let_else,
    reason = "Significantly less readable than the original"
)]
#![allow(clippy::doc_markdown, reason = "Not reliable enough")]
#![allow(
    clippy::missing_errors_doc,
    reason = "The ParserError and WriterError are descriptive enough"
)]
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
pub mod json_types;
pub mod loc8;
pub mod msm;
pub mod secure_fat;

pub mod utils;
