#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt as _;
use wii_toolkit::u8a::types::U8Archive;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The file to parse
    source: PathBuf,
    /// The directory to extract all files too
    destination: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let source = cli.source;
    // let destination = cli.destination.unwrap_or_else(|| {
    //     source
    //         .parent()
    //         .expect("No parent directory for source!")
    //         .to_path_buf()
    // });

    let file = File::open(source).unwrap();

    let u8a = U8Archive::deserialize(&file).unwrap();

    println!("{u8a:#?}");

    // create_dir_all(&destination).expect("Could not create directory!");

    // for packedfile in u8a.files {
    //     let mut path = destination.clone();
    //     path.extend(&packedfile.path);
    //     create_dir_all(&path).unwrap();
    //     path.push(packedfile.name);
    //     let mut file = File::create(path.join(packedfile.name)).expect("Could not create file!");
    //     file.write_all(packedfile.data)
    //         .expect("Could not write to file!");
    // }
}
