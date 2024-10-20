#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt as _;
use ubiart_toolkit::msm::MovementSpaceMove;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the .msm file
    source: PathBuf,
    /// Where to save the parsed (JSON) file
    output: Option<PathBuf>,
    /// Don't print information about the file
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.source).unwrap();
    let msm = MovementSpaceMove::deserialize(&file).unwrap();

    if !cli.quiet {
        println!("name: {}", msm.name);
        println!("map: {}", msm.map);
        println!("device: {}", msm.device);
        println!("unk2: {}", msm.version);
        println!("unk3: {}", msm.unk3);
        println!("unk4: {}", msm.unk4);
        println!("unk5: {}", msm.unk5);
        println!("unk6: {:?}", msm.unk6);
        println!("unk7: {:?}", msm.unk7);
        println!("unk10: {:?}", msm.unk10);
        println!("unk11: {}", msm.unk11);
        println!("unk13: {}", msm.unk13);
        println!("unk14: {}", msm.unk14);
        println!("unk15: {}", msm.unk15);
        println!("Pairs: {}", msm.data.len());

        let x_min = msm
            .data
            .iter()
            .map(|point| point.0)
            .reduce(f32::min)
            .unwrap();
        let x_max = msm
            .data
            .iter()
            .map(|point| point.0)
            .reduce(f32::max)
            .unwrap();
        let y_min = msm
            .data
            .iter()
            .map(|point| point.1)
            .reduce(f32::min)
            .unwrap();
        let y_max = msm
            .data
            .iter()
            .map(|point| point.1)
            .reduce(f32::max)
            .unwrap();
        println!("x_min: {} (0x{:x})", x_min, x_min.to_bits());
        println!("x_max: {} (0x{:x})", x_max, x_max.to_bits());
        println!("y_min: {} (0x{:x})", y_min, y_min.to_bits());
        println!("y_max: {} (0x{:x})", y_max, y_max.to_bits());
    }

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, &msm).unwrap();
    }
}
