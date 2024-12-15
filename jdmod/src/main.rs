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

// use jdmod::check::Check;
use clap::{Parser, Subcommand};
use jdmod::{
    bundle::Bundle, export::Build, extract::Extract, import::Import, new::New, unlock::Unlock,
};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

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
    /// Unlock all songs, avatars, etc…
    Unlock(Unlock),
}

fn main() -> ExitCode {
    // take_hook() returns the default hook in case when a custom one is not set
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        std::process::exit(1);
    }));

    let cli = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        .without_time();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let result = match cli.commands {
        Commands::New(data) => jdmod::new::main(&data),
        Commands::Import(data) => jdmod::import::main(&data),
        Commands::Extract(data) => jdmod::extract::main(data),
        Commands::Export(data) => jdmod::export::main(&data),
        // Commands::Check(data) => jdmod::check::main(&data),
        Commands::Bundle(data) => jdmod::bundle::main(&data),
        Commands::Unlock(data) => jdmod::unlock::main(&data),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:#?}");
            ExitCode::FAILURE
        }
    }
}
