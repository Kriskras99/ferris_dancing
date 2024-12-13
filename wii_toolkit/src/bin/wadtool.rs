#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{fs::File, path::PathBuf};

use clap::Parser;
use dotstar_toolkit_utils::bytes::read::BinaryDeserializeExt as _;
use wii_toolkit::wad::types::WadArchive;

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
    let wad_archive = WadArchive::deserialize(&file).unwrap();

    println!("{wad_archive:#?}");

    // create_dir_all(&destination).expect("Could not create directory!");

    // match wad_archive.archive() {
    //     WadArchive::Installable(installable) => {
    //         for content in &installable.content {
    //             let cid = content.metadata.content_id;
    //             let index = content.metadata.index;
    //             let cmd_type = u16::from(content.metadata.content_type);
    //             let filename = format!("{cid:08x}.{index:04x}.{cmd_type:04x}.app");
    //             let filepath = destination.join(&filename);
    //             let mut file = File::create(filepath).expect("Could not create file!");
    //             let buffer = content.decrypt().expect("Could not decrypt file!");

    //             let mut hasher = Sha1::new();
    //             hasher.update(&buffer);
    //             let result = hasher.finalize();

    //             assert!(
    //                 content.metadata.sha1_hash == result.as_slice(),
    //                 "SHA-1 hashes don't match!"
    //             );

    //             file.write_all(&buffer).expect("Could not write to file!");
    //         }
    //     }
    //     WadArchive::Backup(_) => todo!(),
    // };
}
