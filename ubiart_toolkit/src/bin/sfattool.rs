use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Error};
use clap::Parser;

use memmap2::Mmap;
use ubiart_toolkit::secure_fat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    #[arg(short, long, default_value_t = false)]
    list: bool,
    #[arg(short, long, default_value_t = false)]
    header: bool,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let file = File::open(cli.source)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let sfat = secure_fat::parse(&mmap)?;

    if cli.header {
        for (bundle_id, name) in sfat.bundle_ids_and_names() {
            println!("BundleId: {:x}, Name: {name}", u8::from(*bundle_id));
        }
    }

    if cli.list {
        for (path_id, bundle_ids) in sfat.path_ids_and_bundle_ids() {
            let bundle_names: Vec<_> = bundle_ids
                .iter()
                .map(|b| {
                    sfat.get_bundle_name(b)
                        .ok_or_else(|| anyhow!("Unknown bundle id!"))
                })
                .collect::<Result<_, _>>()?;
            println!("0x{:08x}: {bundle_names:?}", u32::from(*path_id));
        }
    }

    Ok(())
}
