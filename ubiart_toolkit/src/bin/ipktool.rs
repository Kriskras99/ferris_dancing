#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
use std::{
    collections::HashSet,
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;
use dotstar_toolkit_utils::vfs::{native::Native, VirtualFileSystem};
use memmap2::Mmap;
use ubiart_toolkit::{
    ipk::{self, Bundle},
    utils::{
        errors::{ParserError, WriterError},
        GamePlatform, PathId,
    },
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
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

    let source = &cli.source;

    if cli.compress {
        let destination = cli.destination.unwrap_or({
            let mut temp = source.to_str().unwrap().to_owned();
            temp.push_str(".ipk");
            PathBuf::from(temp)
        });
        create_ipk(source, &destination).unwrap();
    } else {
        let file = File::open(source).unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        let ipk = ipk::parse(&mmap, cli.lax).unwrap();

        if cli.check {
            check_ipk(&ipk, source);
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
        let mut path = String::with_capacity(fil.path.path.len() + fil.path.filename.len());
        path.push_str(&fil.path.path);
        path.push_str(&fil.path.filename);
        println!(
            "{} [0x{:x}] ({} | {})",
            path,
            u32::from(PathId::from(path.as_str())),
            if fil.is_cooked { 'C' } else { 'U' },
            fil.timestamp
        );
        dirs.insert(fil.path.path.as_ref());
        n += 1;
    }
    println!("{} directories, {n} files", dirs.len());
    println!(
        "Version: 0x{:x}, Engine: 0x{:x}, U4: 0x{:x}, GP: {:?}",
        ipk.version, ipk.engine_version, ipk.unk4, ipk.game_platform,
    );
}

/// Extract a IPK bundle to destination
pub fn unpack_ipk(ipk: &Bundle, destination: &Path, overwrite: bool) -> Result<(), ParserError> {
    create_dir_all(destination)?;

    for fil in ipk.files.values() {
        let path = &destination.join(fil.path.path.as_ref());
        create_dir_all(path)?;
        let filepath = &path.join(fil.path.filename.as_ref());
        if overwrite || !filepath.exists() {
            let mut file = File::create(filepath)?;
            match &fil.data {
                ipk::Data::Uncompressed(unc) => {
                    // Copy all the packed data into the file
                    file.write_all(unc.data)?;
                }
                ipk::Data::Compressed(data) => {
                    let mut vec = Vec::with_capacity(data.uncompressed_size + 1);
                    let mut decompress = flate2::Decompress::new(true);
                    decompress
                        .decompress_vec(data.data, &mut vec, flate2::FlushDecompress::Finish)
                        .unwrap();
                    file.write_all(&vec)?;
                }
            }
        } else {
            println!("File already exists!: {filepath:?}");
        }
    }
    Ok(())
}

pub fn check_ipk(ipk: &Bundle, filename: &Path) {
    println!("GamePlatform: {:#?}", ipk.game_platform);
    if ipk.version != 5 {
        println!("{filename:?}: Unknown IPK version!: 0x{:x}", ipk.version);
    }

    for packed_file in ipk.files.values() {
        if packed_file.is_cooked && !packed_file.path.path.contains("itf_cooked") {
            println!(
                "  Metadata says cooked but PackedFile does not have 'itf_cooked' in path!: {} {}",
                packed_file.is_cooked, packed_file.path
            );
        } else if !packed_file.is_cooked && packed_file.path.path.contains("itf_cooked") {
            println!(
                "  Metadata says not cooked but PackedFile does have 'itf_cooked' in path!: {} {}",
                packed_file.is_cooked, packed_file.path
            );
        }
    }
}

/// Create a IPK bundle from all files and directories in `source`
pub fn create_ipk(source: &Path, destination: &Path) -> Result<(), WriterError> {
    let vfs = Native::new(source)?;
    let file_list = vfs.list_files(&PathBuf::from(""))?;
    let files: Vec<_> = file_list.iter().map(String::as_str).collect();
    let file = File::create(destination)?;
    ipk::write(
        file,
        GamePlatform::try_from(0x1DDB_2268)?,
        0x937D0,
        0x4FD39,
        ipk::Options {
            compression: ipk::CompressionEffort::Best,
        },
        &vfs,
        &files,
    )?;
    Ok(())
}
