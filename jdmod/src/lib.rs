// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! # JDMod
//! Application for modding Just Dance games.
//! The goal is to be able to import every Just Dance game ever made and to be made, and be able to export to Just Dance 2022 Switch.
//!
//! Currently supported are Just Dance 2017-2022 for the Switch.
//! It can import and export songs, playlists, quests/objectives, avatars, aliases, portraitborders, gacha machine, and search labels.

use clap::ValueEnum;

pub mod build;
pub mod bundle;
// pub mod check;
pub mod export;
pub mod extract;
pub mod import;
pub mod new;
pub mod types;
pub mod unlock;
pub mod utils;

/// Strategies for resolving file conflicts
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum FileConflictStrategy {
    /// Overwrite the file
    OverwriteSilent,
    /// Overwrite the file and print a warning
    OverwriteWithWarning,
    /// Do not overwrite the file and return an error
    Error,
}
