#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read_to_vec;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use ubiart_toolkit::{cooked, utils::UniqueGameId};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        .without_time();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let path = cli.source;

    let data = read_to_vec(&path).unwrap();
    let tpl = match cooked::tpl::parse(&data, UniqueGameId::WIIU2015, false) {
        Ok(tpl) => tpl,
        Err(e) => panic!("{path:?}: {e:?}"),
    };

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &tpl).unwrap();
    }
}
