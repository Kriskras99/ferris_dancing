use std::{fs::File, path::PathBuf};

use clap::Parser;
use ubiart_toolkit::msm;

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

    let moves = msm::open(&cli.source).unwrap();
    let msm = moves.msm();

    if !cli.quiet {
        println!("name: {}", msm.name);
        println!("map: {}", msm.map);
        println!("device: {}", msm.device);
        println!("points: {}", msm.points);
        println!("unk3: {:x}", msm.unk3);
        println!("unk4: {:x}", msm.unk4);
        println!("unk5: {:x}", msm.unk5);
        println!("unk6: {:x}", msm.unk6);
        println!("unk7: {:x}", msm.unk7);
        println!("unk10: {:x}", msm.unk10);
        println!("unk14: {:x}", msm.unk14);
        println!("unk15: {:x}", msm.unk15);
    }

    if let Some(path) = cli.output {
        let file = File::create(path).unwrap();
        serde_json::to_writer_pretty(file, msm).unwrap();
    }
}
