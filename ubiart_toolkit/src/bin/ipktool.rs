#![allow(
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    reason = "Tool not a library"
)]

use std::{
    collections::HashSet,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use tracing::{debug, info, warn};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use dotstar_toolkit_utils::{
    bytes::read::BinaryDeserializeExt as _,
    vfs::{native::NativeFs, VirtualFileSystem, VirtualPathBuf},
};
use dotstar_toolkit_utils::bytes::read::BinaryDeserialize;
use ubiart_toolkit::{
    ipk::{self, Bundle},
    utils::{
        errors::{ParserError, WriterError},
        UniqueGameId,
    },
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools, reason = "CLI")]
struct Cli {
    source: PathBuf,
    destination: Option<PathBuf>,
    #[arg(short, long)]
    extract: bool,
    #[arg(short, long)]
    compress: bool,
    #[arg(short, long)]
    list: bool,
    #[arg(long, default_value_t = true)]
    check: bool,
    #[arg(long, default_value_t = false)]
    overwrite: bool,
    #[arg(long, default_value_t = false)]
    lax: bool,
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

    let source = &cli.source;

    if cli.compress {
        let destination = cli.destination.unwrap_or_else(|| {
            let mut temp = source.to_str().unwrap().to_owned();
            temp.push_str(".ipk");
            PathBuf::from(temp)
        });
        create_ipk(source, &destination).unwrap();
    } else {
        let file = File::open(source).unwrap();
        let ipk = Bundle::deserialize_with(&file, cli.lax).unwrap();

        if cli.check {
            check_ipk(&ipk, source, cli.lax);
        }

        if cli.list {
            list_ipk(&ipk);
        }

        if cli.extract {
            let destination = cli.destination.unwrap_or_else(|| {
                source
                    .parent()
                    .expect("No parent!")
                    .join(source.file_stem().unwrap())
            });
            unpack_ipk(&ipk, &destination, cli.overwrite).unwrap();
        }
    }
}

pub fn list_ipk(ipk: &Bundle) {
    let mut dirs = HashSet::new();
    let mut n: usize = 0;
    for fil in ipk.files.values() {
        let path = &fil.path;
        info!(
            "{path} [0x{:x}] ({} | {})",
            u32::from(path.id()),
            if fil.is_cooked { 'C' } else { 'U' },
            fil.timestamp
        );
        dirs.insert(fil.path.parent());
        n += 1;
    }
    info!("{} directories, {n} files", dirs.len());
    info!(
        "Version: 0x{:x}, Engine: 0x{:x}, U4: 0x{:x}, GP: {:?}",
        ipk.version, ipk.engine_version, ipk.unk4, ipk.game_platform,
    );
}

/// Extract a IPK bundle to destination
pub fn unpack_ipk(ipk: &Bundle, destination: &Path, overwrite: bool) -> Result<(), ParserError> {
    create_dir_all(destination)?;

    for fil in ipk.files.values() {
        let path = &destination.join(fil.path.parent());
        create_dir_all(path)?;
        let filepath = &path.join(fil.path.filename());
        if overwrite || !filepath.exists() {
            let mut file = File::create(filepath)?;
            match &fil.data {
                ipk::Data::Uncompressed(unc) => {
                    // Copy all the packed data into the file
                    file.write_all(unc.data.as_ref())?;
                }
                ipk::Data::Compressed(data) => {
                    let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                    let mut decompress = flate2::Decompress::new(true);
                    decompress
                        .decompress_vec(
                            data.data.as_ref(),
                            &mut vec,
                            flate2::FlushDecompress::Finish,
                        )
                        .unwrap();
                    file.write_all(&vec)?;
                }
            }
        } else {
            warn!("File already exists!: {filepath:?}");
        }
    }
    Ok(())
}

pub fn check_ipk(ipk: &Bundle, filename: &Path, lax: bool) {
    info!("GamePlatform: {:#?}", ipk.game_platform);
    if ipk.version != 5 {
        if lax {
            debug!("{filename:?}: Unknown IPK version!: 0x{:x}", ipk.version);
        } else {
            warn!("{filename:?}: Unknown IPK version!: 0x{:x}", ipk.version);
        }
    }

    for packed_file in ipk.files.values() {
        if packed_file.is_cooked && !packed_file.path.contains("itf_cooked") {
            if lax {
                debug!(
                    "  Metadata says cooked but PackedFile does not have 'itf_cooked' in path!: {} {}",
                    packed_file.is_cooked, packed_file.path
                );

            } else {
                info!(
                    "  Metadata says cooked but PackedFile does not have 'itf_cooked' in path!: {} {}",
                    packed_file.is_cooked, packed_file.path
                );

            }
        } else if !packed_file.is_cooked && packed_file.path.contains("itf_cooked") {
            if lax {
                debug!(
                    "  Metadata says not cooked but PackedFile does have 'itf_cooked' in path!: {} {}",
                    packed_file.is_cooked, packed_file.path
                );

            } else {
                info!(
                    "  Metadata says not cooked but PackedFile does have 'itf_cooked' in path!: {} {}",
                    packed_file.is_cooked, packed_file.path
                );

            }
        }
    }
}

/// Create a IPK bundle from all files and directories in `source`
pub fn create_ipk(source: &Path, destination: &Path) -> Result<(), WriterError> {
    let vfs = NativeFs::new(source)?;
    let root = VirtualPathBuf::from("");
    let file_list = vfs.walk_filesystem(&root)?;
    let files: Vec<_> = file_list.collect();
    let mut file = File::create(destination)?;
    ipk::write(
        &mut file,
        &mut 0,
        ipk::Options {
            compression: ipk::CompressionEffort::Best,
            game_platform: UniqueGameId::try_from(0x1DDB_2268)?,
            unk4: 0x937D0,
            engine_version: 0x4FD39,
        },
        &vfs,
        &files,
    )
    .unwrap();
    Ok(())
}
