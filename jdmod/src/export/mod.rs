//! # Export
//! Builds the mod into a format that Just Dance 2022 can understand and then bundles it into .ipk files
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{bail, Error};
use clap::Args;
use dotstar_toolkit_utils::vfs::symlinkfs::SymlinkFs;
use dotstar_toolkit_utils::vfs::vecfs::VecFs;
use dotstar_toolkit_utils::vfs::{layeredfs::OverlayFs, native::Native};
use ubiart_toolkit::ipk::vfs::IpkFilesystem;
use ubiart_toolkit::utils::Game;
use ubiart_toolkit::utils::Platform;

use crate::build::BuildFiles;
use crate::build::BuildState;
use crate::types::song::SongDirectoryTree;
use crate::types::Config;
use crate::{build, types::DirectoryTree};

mod bundle;

/// Build the mod at `source` to `destination`
#[derive(Args, Clone)]
pub struct Build {
    /// The mod directory
    source: PathBuf,
    /// Directory to put the bundles
    destination: PathBuf,
    /// Create a patch file instead of a bundle file
    #[arg(long, default_value_t = false)]
    patch: bool,
}

/// Files that need to be added to bundle
pub enum FilesToAdd<'a> {
    /// Files belonging to a song
    Song(BuildFiles<'a>),
    /// Files belonging to the game itself
    Bundle(BuildFiles<'a>),
}

/// Wrapper around [`export`]
pub fn main(cli: &Build) -> Result<(), anyhow::Error> {
    export(&cli.source, &cli.destination, cli.patch)
}

/// Builds the mod into a format that Just Dance 2022 can understand and then bundles it into .ipk files
pub fn export(source: &Path, destination: &Path, patch: bool) -> Result<(), Error> {
    // Check the directory structure
    let dir_tree = DirectoryTree::new(source);
    if !dir_tree.exists() {
        bail!("Mod directory does not exist or is missing vital subdirectories!");
    }
    // Create the export directory
    if destination.exists() {
        if !destination.is_dir() {
            bail!("Destination directory is not a directory! {destination:?}");
        } else if destination.read_dir()?.next().is_some() {
            bail!("Destination directory is not empty! {destination:?}");
        }
    } else {
        std::fs::create_dir(destination)?;
    }

    // Load bundle_nx.ipk and patch_nx.ipk to use as a base
    let base_native_vfs = Native::new(dir_tree.base())?;
    let bundle_nx_vfs = IpkFilesystem::new(&base_native_vfs, "bundle_nx.ipk".as_ref(), false)?;
    let patch_nx_vfs = IpkFilesystem::new(&base_native_vfs, "patch_nx.ipk".as_ref(), false)?;
    let patched_base_vfs = OverlayFs::new(&patch_nx_vfs, &bundle_nx_vfs);

    /*
     * 1 thread bundles build files into ipks. It will only bundle song files until the channel is dropped
     * 1 thread starts building localisation then receives song jobs
     * 1 thread starts building gameconfig then receives song jobs
     * available_cpus-3 threads receive song jobs
     *
     * After all song jobs are completed, song database will be built on main thread
     * After bundle thread receives song database it will built the bundle_nx.ipk
     */

    // Setup the (read-only) build state
    let config: Config =
        serde_json::from_reader(File::open(dir_tree.dot_mod().join("config.json"))?)?;
    let platform = Platform::Nx;
    let game = Game::JustDance2022;
    let build_state = BuildState {
        patched_base_vfs: &patched_base_vfs,
        dirs: &dir_tree,
        platform,
        game,
        engine_version: config.engine_version,
    };

    let native_vfs = Native::new(&std::env::current_dir()?)?;

    // Get a list of all songs in the directory
    let mut paths: Vec<_> = std::fs::read_dir(build_state.dirs.songs())?
        .filter_map(Result::ok)
        .map(|d| std::fs::DirEntry::path(&d))
        .filter(|p| p.is_dir())
        .collect();
    // Sort them, so we go through them alphabetically. This way the user can see how far we are in building the songs.
    paths.sort();

    let ncpus = usize::from(std::thread::available_parallelism()?);

    let (tx_name, rx_name) = crossbeam::channel::unbounded();
    let (tx_job, rx_job) = crossbeam::channel::unbounded();
    let (tx_files, rx_files) = crossbeam::channel::unbounded();

    for path in paths {
        let dirs = SongDirectoryTree::new(&path);
        if dirs.exists() {
            tx_job.send(dirs)?;
        } else {
            println!("Warning! Path '{path:?}' has a incomplete directory structure, skipping!");
        }
    }

    drop(tx_job);

    std::thread::scope(|s| {
        let build_state = &build_state;
        let native_vfs = &native_vfs;
        std::thread::Builder::new()
            .name("Bundle".to_string())
            .spawn_scoped(s, || {
                bundle::bundle(
                    &bundle_nx_vfs,
                    &patch_nx_vfs,
                    native_vfs,
                    &rx_files,
                    config,
                    destination,
                    patch,
                )
                .unwrap();
            })
            .unwrap();

        {
            // New scope so the channel variables can be shadowed
            let tx_name = tx_name.clone();
            let tx_files = tx_files.clone();
            let rx_job = rx_job.clone();
            std::thread::Builder::new()
                .name("Localisation + Songs".to_string())
                .spawn_scoped(s, move || {
                    // Build translations
                    let mut build_files = BuildFiles {
                        generated_files: VecFs::with_capacity(30),
                        static_files: SymlinkFs::new(native_vfs),
                    };
                    build::localisation::build(build_state, &mut build_files).unwrap();

                    loop {
                        if let Ok(job) = rx_job.recv() {
                            let mut bf = BuildFiles {
                                generated_files: VecFs::with_capacity(100),
                                static_files: SymlinkFs::with_capacity(native_vfs, 50),
                            };
                            let songname = build::song::build(build_state, &mut bf, job).unwrap();
                            tx_name.send(songname).unwrap();
                            tx_files.send(FilesToAdd::Song(bf)).unwrap();
                        } else {
                            // Otherwise the rx_name.iter() will never stop
                            drop(tx_name);
                            println!("No more song available to build! Breaking!");
                            break;
                        }
                    }

                    // Collect and sort all the song names
                    // Sorting is not necessary, but makes the files easier to read when debugging
                    let mut song_names: Vec<_> = rx_name.iter().collect();
                    song_names.sort_unstable();

                    // Build the various song databases and scenes
                    build::song_database(build_state, &mut build_files, &song_names).unwrap();
                    tx_files.send(FilesToAdd::Bundle(build_files)).unwrap();
                })
                .unwrap();
        }

        {
            // New scope so the channel variables can be shadowed
            let tx_name = tx_name.clone();
            let tx_files = tx_files.clone();
            let rx_job = rx_job.clone();
            std::thread::Builder::new()
                .name("Gameconfig + Songs".to_string())
                .spawn_scoped(s, move || {
                    // Build config files
                    let mut build_files = BuildFiles {
                        generated_files: VecFs::with_capacity(1000),
                        static_files: SymlinkFs::new(native_vfs),
                    };
                    build::gameconfig::build(build_state, &mut build_files).unwrap();
                    tx_files.send(FilesToAdd::Bundle(build_files)).unwrap();

                    loop {
                        if let Ok(job) = rx_job.recv() {
                            let mut bf = BuildFiles {
                                generated_files: VecFs::with_capacity(100),
                                static_files: SymlinkFs::with_capacity(native_vfs, 50),
                            };
                            let songname = build::song::build(build_state, &mut bf, job).unwrap();
                            tx_name.send(songname).unwrap();
                            tx_files.send(FilesToAdd::Song(bf)).unwrap();
                        } else {
                            println!("No more song available to build! Exiting thread!");
                            return;
                        }
                    }
                })
                .unwrap();
        }

        // Only start as many threads as there are cpus (excluding the main thread, which will be waiting and doing nothing the entire time)
        for i in 0..ncpus.saturating_sub(3) {
            let tx_name = tx_name.clone();
            let rx_job = rx_job.clone();
            let tx_files = tx_files.clone();
            std::thread::Builder::new()
                .name(format!("Songs ({i})"))
                .spawn_scoped(s, move || loop {
                    if let Ok(job) = rx_job.recv() {
                        let mut bf = BuildFiles {
                            generated_files: VecFs::with_capacity(100),
                            static_files: SymlinkFs::with_capacity(native_vfs, 50),
                        };
                        let songname = build::song::build(build_state, &mut bf, job).unwrap();
                        tx_name.send(songname).unwrap();
                        tx_files.send(FilesToAdd::Song(bf)).unwrap();
                    } else {
                        println!("No more song available to build! Exiting thread!");
                        return;
                    }
                })
                .unwrap();
        }

        // If these aren't dropped before the scope ends, the threads will infinitely wait for more jobs
        drop(tx_files);
        drop(tx_name);
        drop(rx_job);
    });

    println!("Done!");

    Ok(())
}
