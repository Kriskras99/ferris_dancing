use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use wii_toolkit::u8a;

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
    let destination = cli.destination.unwrap_or_else(||
        source
            .parent()
            .expect("No parent directory for source!")
            .to_path_buf()
    );

    let u8a = u8a::parser::open(source).expect("Could not parse file!");

    create_dir_all(&destination).expect("Could not create directory!");

    for packedfile in u8a.files() {
        let mut path = destination.clone();
        path.extend(&packedfile.path);
        create_dir_all(&path).unwrap();
        path.push(packedfile.name);
        let mut file = File::create(path.join(packedfile.name)).expect("Could not create file!");
        file.write_all(packedfile.data)
            .expect("Could not write to file!");
    }
}
