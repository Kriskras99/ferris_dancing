use std::{fs::File, path::PathBuf};

use clap::Parser;
use memmap2::Mmap;
use ubiart_toolkit::secure_fat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    list: bool,
    #[arg(long, default_value_t = false)]
    header: bool,
    /// Ignore mistakes in the file format (useful for modded files)
    #[arg(long, default_value_t = false)]
    lax: bool,
}

fn main() {
    let cli = Cli::parse();

    let file = File::open(cli.source).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let sfat = secure_fat::parse(&mmap, cli.lax).unwrap();

    if cli.header {
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            println!("BundleId: {:x}, Name: {name}", u8::from(*bundle_id));
        }
    }

    if cli.list {
        for (path_id, bundle_ids) in sfat.path_ids_and_bundle_ids() {
            let bundle_names: Vec<_> = bundle_ids
                .iter()
                .map(|b| sfat.get_bundle_name(b).unwrap())
                .collect();
            println!("0x{:08x}: {bundle_names:?}", u32::from(*path_id));
        }
    }
}
