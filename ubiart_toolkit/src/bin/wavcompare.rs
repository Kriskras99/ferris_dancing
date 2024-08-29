#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use gc_adpcm::SAMPLES_PER_FRAME;
use tracing::{level_filters::LevelFilter, trace};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    left: PathBuf,
    right: PathBuf,
}

pub fn main() {
    let args = Cli::parse();

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
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();

    let left_file = BufReader::new(File::open(args.left).unwrap());
    let left_decoder = hound::WavReader::new(left_file).unwrap();
    let left_spec = left_decoder.spec();
    let left_samples: hound::WavIntoSamples<BufReader<File>, i16> = left_decoder.into_samples();

    let right_file = BufReader::new(File::open(args.right).unwrap());
    let right_decoder = hound::WavReader::new(right_file).unwrap();
    let right_spec = right_decoder.spec();
    let right_samples: hound::WavIntoSamples<BufReader<File>, i16> = right_decoder.into_samples();

    trace!("Left spec: {left_spec:#?}");
    trace!("Right spec: {right_spec:#?}");

    for (index, (left_sample, right_sample)) in left_samples.zip(right_samples).enumerate() {
        let left_sample = left_sample.unwrap();
        let right_sample = right_sample.unwrap();
        trace!("{index:08}: {left_sample} {right_sample}");
        if u32::try_from(index).unwrap() == SAMPLES_PER_FRAME * 4 {
            break;
        }
    }
}
