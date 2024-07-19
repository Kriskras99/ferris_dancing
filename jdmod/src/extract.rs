//! # Extract
//! Code for extracting a UbiArt archive (ipk or gf)
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use clap::Args;
use dotstar_toolkit_utils::{
    test_eq,
    vfs::{native::NativeFs, VirtualFileSystem, VirtualPathBuf},
};
use ubiart_toolkit::{ipk, secure_fat};

use crate::FileConflictStrategy;

/// Extract a UbiArt archive
#[derive(Args, Clone)]
pub struct Extract {
    /// File to extract, can be a .ipk or secure_fat.gf
    source: PathBuf,
    /// Specific files to extract
    files: Vec<String>,
    /// Directory to extract to
    #[arg(short, long)]
    destination: Option<PathBuf>,
    /// Determines how file conflicts are handled
    #[arg(value_enum, short, long, default_value_t=FileConflictStrategy::OverwriteWithWarning)]
    conflicts: FileConflictStrategy,
    /// Ignore mistakes in the file format (useful for modded files)
    #[arg(long, default_value_t = false)]
    lax: bool,
}

/// Extract a game archive at `source` to `destination`
///
/// If `files` is specified, it will only extract those files
pub fn main(extract: Extract) -> Result<(), Error> {
    let source = extract.source;
    test_eq!(source.try_exists()?, true, "Source does not exist!")?;
    test_eq!(source.is_file(), true, "Source is not a file!")?;
    let source = source.canonicalize()?;
    let destination = extract.destination.unwrap_or(fs::canonicalize(".")?);
    // Create the export directory
    if destination.exists() && destination.read_dir()?.next().is_some() {
        return Err(anyhow!("Target directory exists and is not empty!"));
    } else if !destination.exists() {
        std::fs::create_dir(&destination)?;
    }
    let files: Vec<&str> = extract.files.iter().map(String::as_str).collect();
    let files = if files.is_empty() {
        None
    } else {
        Some(files.as_slice())
    };
    let conflicts = extract.conflicts;

    let extension = source
        .extension()
        .ok_or_else(|| anyhow!("Source does not have an extension!"))?
        .to_str()
        .ok_or_else(|| anyhow!("Source extension is invalid!"))?;

    match extension {
        "ipk" => extract_ipk(&source, &destination, files, conflicts),
        "gf" => extract_secure_fat(&source, &destination, files, conflicts),
        _ => Err(anyhow!(
            "Unknown file extension '{extension}', expected 'ipk' or 'gf'!"
        )),
    }
}

/// Extract a secure_fat.gf file to a specified location.
///
/// Arguments:
/// * `vfs`: The virtual filesystem which contains the source.
/// * `source`: The source path which should be an secure_fat.gf file.
/// * `destination`: The directory to extract to.
/// * `files`: When provided, only those files are extracted.
/// * `conflicts`: How to handle file conflicts with existing files in `destination`.
pub fn extract_secure_fat(
    source: &Path,
    destination: &Path,
    files: Option<&[&str]>,
    conflicts: FileConflictStrategy,
) -> Result<(), Error> {
    // Split source in directory and filename
    let source_directory = source
        .parent()
        .ok_or_else(|| anyhow!("Source file has no parent directory!"))?;
    let source_filename = VirtualPathBuf::from(
        source
            .file_name()
            .ok_or_else(|| anyhow!("Source does not have a filename!"))?
            .to_str()
            .ok_or_else(|| anyhow!("Source filename is invalid!"))?,
    );
    // Open the sfat as a vfs using the native filesystem as base
    let native_vfs = NativeFs::new(source_directory)?;
    let fat_vfs = secure_fat::vfs::SfatFilesystem::new(&native_vfs, &source_filename)?;
    extract_vfs(&fat_vfs, destination, files, conflicts)
}

/// Extract a .ipk file to a specified location.
///
/// Arguments:
/// * `vfs`: The virtual filesystem which contains the source.
/// * `source`: The source path which should be an .ipk file.
/// * `destination`: The directory to extract to.
/// * `files`: When provided, only those files are extracted.
/// * `conflicts`: How to handle file conflicts with existing files in `destination`.
pub fn extract_ipk(
    source: &Path,
    destination: &Path,
    files: Option<&[&str]>,
    conflicts: FileConflictStrategy,
) -> Result<(), Error> {
    // Split source in directory and filename
    let source_directory = source
        .parent()
        .ok_or_else(|| anyhow!("Source file has no parent directory!"))?;
    let source_filename = VirtualPathBuf::from(
        source
            .file_name()
            .ok_or_else(|| anyhow!("Source does not have a filename!"))?
            .to_str()
            .ok_or_else(|| anyhow!("Source filename is invalid!"))?,
    );

    // Open the sfat as a vfs using the native filesystem as base
    let native_vfs = NativeFs::new(source_directory)?;
    let ipk_vfs = ipk::vfs::IpkFilesystem::new(&native_vfs, &source_filename)?;
    extract_vfs(&ipk_vfs, destination, files, conflicts)?;
    Ok(())
}

/// Extract from a virtual filesystem to a specified location.
///
/// Arguments:
/// * `vfs`: The virtual filesystem which contains the source.
/// * `destination`: The directory to extract to.
/// * `files`: When provided, only those files are extracted, otherwise *all* files are extracted.
/// * `conflicts`: How to handle file conflicts with existing files in `destination`.
pub fn extract_vfs(
    vfs: &dyn VirtualFileSystem,
    destination: &Path,
    files: Option<&[&str]>,
    conflicts: FileConflictStrategy,
) -> Result<(), Error> {
    if let Some(files) = files {
        // Search for the files in the vfs and extract
        for file in files {
            match vfs.open(file.as_ref()) {
                Err(e) => eprintln!("{e:?}"),
                Ok(data) => save_file(&data, &destination.join(file), conflicts)?,
            }
        }
    } else {
        for file in vfs.walk_filesystem("".as_ref())? {
            match vfs.open(file.as_ref()) {
                Err(e) => eprintln!("{e:?}"),
                Ok(data) => save_file(&data, &destination.join(file.as_str()), conflicts)?,
            }
        }
    }
    Ok(())
}

/// Convenience function for writing data to a new file
fn save_file(
    data: &[u8],
    destination: &Path,
    conflicts: FileConflictStrategy,
) -> Result<(), anyhow::Error> {
    fs::create_dir_all(
        destination
            .parent()
            .ok_or_else(|| anyhow!("File should have a parent directory!"))?,
    )?;
    match (destination.exists(), conflicts) {
        (true, FileConflictStrategy::Error) => Err(anyhow!("{destination:?} already exists!")),
        (true, FileConflictStrategy::OverwriteWithWarning) => {
            println!("Warning! Overwriting {destination:?}!");
            let mut file = File::create(destination)?;
            file.write_all(data)?;
            Ok(())
        }
        (_, _) => {
            let mut file = File::create(destination)?;
            file.write_all(data)?;
            Ok(())
        }
    }
}
