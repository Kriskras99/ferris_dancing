// Everything should be documented
#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]

//! # JDMod
//! Application for modding Just Dance games.
//! The goal is to be able to import every Just Dance game ever made and to be made, and be able to export to Just Dance 2022 Switch.
//!
//! Currently supported are Just Dance 2017-2022 for the Switch.
//! It can import and export songs, playlists, quests/objectives, avatars, aliases, portraitborders, gacha machine, and search labels.

use std::process::ExitCode;

use bundle::Bundle;
// use check::Check;
use clap::{Parser, Subcommand, ValueEnum};
use export::Build;
use extract::Extract;
use import::Import;
use new::New;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod build;
mod bundle;
// mod check;
mod export;
mod extract;
mod import;
mod new;
mod types;
mod utils;

/// The command line interface generated with Clap derive
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Commands that can be executed with this application
    #[command(subcommand)]
    commands: Commands,
}

/// Commands that can be executed with this application
#[derive(Subcommand, Clone)]
enum Commands {
    /// Create a new mod
    New(New),
    /// Import Just Dance files
    Import(Import),
    /// Extract Just Dance files
    Extract(Extract),
    /// Export the mod
    Export(Build),
    // /// Check the completeness of the mod
    // Check(Check),
    /// Bundle files into a .ipk
    Bundle(Bundle),
}

/// Strategies for resolving file conflicts
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FileConflictStrategy {
    /// Overwrite the file
    OverwriteSilent,
    /// Overwrite the file and print a warning
    OverwriteWithWarning,
    /// Do not overwrite the file and return an error
    Error,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(true);
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let result = match cli.commands {
        Commands::New(data) => new::main(&data),
        Commands::Import(data) => import::main(&data),
        Commands::Extract(data) => extract::main(data),
        Commands::Export(data) => export::main(&data),
        // Commands::Check(data) => check::main(&data),
        Commands::Bundle(data) => bundle::main(&data),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:#?}");
            ExitCode::FAILURE
        }
    }
}
