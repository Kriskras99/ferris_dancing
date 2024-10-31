//! # Export
//! Builds the mod into a format that Just Dance 2022 can understand and then bundles it into .ipk files
use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

use anyhow::{bail, Error};
use clap::Args;
use crossbeam::channel::TryRecvError;
use dotstar_toolkit_utils::vfs::{
    layeredfs::OverlayFs, native::NativeFs, symlinkfs::SymlinkFs, vecfs::VecFs, VirtualFileSystem,
};
use tracing::instrument;
use ubiart_toolkit::{ipk::vfs::IpkFilesystem, utils::Platform};

use crate::{
    build::{self, BuildFiles, BuildState},
    types::{song::RelativeSongDirectoryTree, Config, DirectoryTree, RelativeDirectoryTree},
};

mod bundle;

/// Build the mod at `source` to `destination`
#[derive(Args, Clone)]
pub struct Build {
    /// The mod directory
    source: PathBuf,
    /// Directory to put the bundles
    destination: PathBuf,
    /// Use n threads
    ///
    /// Note: 3 threads is the minimum, any number below that will be ignored
    #[arg(long)]
    threads: Option<NonZeroUsize>,
}

/// Files that need to be added to bundle
pub enum FilesToAdd<'a> {
    /// Files belonging to a song
    Song(BuildFiles<'a>),
    /// Files belonging to the game itself
    Bundle(BuildFiles<'a>),
}

/// Wrapper around [`export`]
pub fn main(cli: &Build) -> Result<(), Error> {
    export(&cli.source, &cli.destination, cli.threads)
}

/// Builds the mod into a format that Just Dance 2022 can understand and then bundles it into .ipk files
///
/// # Panics
/// Will panic if any of the threads it creates return an error
#[instrument]
pub fn export(
    source: &Path,
    destination: &Path,
    n_threads: Option<NonZeroUsize>,
) -> Result<(), Error> {
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

    // Do everything through a virtual filesystem with the mod directory as the root
    let native_vfs = NativeFs::new(dir_tree.root())?;
    let rel_tree = RelativeDirectoryTree::new();

    // Load bundle_nx.ipk and patch_nx.ipk to use as a base
    let bundle_nx_vfs = IpkFilesystem::new(&native_vfs, &rel_tree.base().join("bundle_nx.ipk"))?;
    let patch_nx_vfs = IpkFilesystem::new(&native_vfs, &rel_tree.base().join("patch_nx.ipk"))?;
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
        serde_json::from_slice(&native_vfs.open(&rel_tree.dot_mod().join("config.json"))?)?;
    let platform = Platform::Nx;
    let build_state = BuildState {
        patched_base_vfs: &patched_base_vfs,
        native_vfs: &native_vfs,
        rel_tree,
        platform,
        engine_version: config.engine_version,
    };

    // Get a list of all songs in the directory
    let mut paths: Vec<_> = build_state
        .native_vfs
        .walk_filesystem(build_state.rel_tree.songs())?
        .filter(|p| p.file_name() == Some("song.json"))
        .filter_map(|p| p.parent())
        .collect();
    // Sort them, so we go through them alphabetically. This way the user can see how far we are in building the songs.
    paths.sort();

    let n_threads = if let Some(n_threads) = n_threads {
        usize::from(n_threads)
    } else {
        usize::from(std::thread::available_parallelism()?)
    };

    let (tx_name, rx_name) = crossbeam::channel::unbounded();
    let (tx_job, rx_job) = crossbeam::channel::unbounded();
    let (tx_files, rx_files) = crossbeam::channel::unbounded();
    let (tx_bundle_job, rx_bundle_job) = crossbeam::channel::unbounded();

    for path in paths {
        let dirs = RelativeSongDirectoryTree::new(path);
        tx_job.send(dirs)?;
    }

    drop(tx_job);

    std::thread::scope(|s| {
        let build_state = &build_state;
        let native_vfs = build_state.native_vfs;
        {
            std::thread::Builder::new()
                .name("Bundle".to_string())
                .spawn_scoped(s, || {
                    bundle::bundle(
                        &bundle_nx_vfs,
                        &patch_nx_vfs,
                        native_vfs,
                        &rx_files,
                        tx_bundle_job,
                        config,
                        destination,
                    )
                    .unwrap();
                })
                .unwrap();
        }
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
                        match rx_job.try_recv() {
                            Ok(job) => {
                                let mut bf = BuildFiles {
                                    generated_files: VecFs::with_capacity(100),
                                    static_files: SymlinkFs::with_capacity(native_vfs, 50),
                                };
                                let songname =
                                    build::song::build(build_state, &mut bf, job).unwrap();
                                tx_name.send(songname).unwrap();
                                tx_files.send(FilesToAdd::Song(bf)).unwrap();
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => {
                                println!("No more songs! Creating song database!");
                                drop(tx_name);
                                break;
                            }
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
            let rx_bundle_job = rx_bundle_job.clone();
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
                        match rx_job.try_recv() {
                            Ok(job) => {
                                let mut bf = BuildFiles {
                                    generated_files: VecFs::with_capacity(100),
                                    static_files: SymlinkFs::with_capacity(native_vfs, 50),
                                };
                                let songname =
                                    build::song::build(build_state, &mut bf, job).unwrap();
                                tx_name.send(songname).unwrap();
                                tx_files.send(FilesToAdd::Song(bf)).unwrap();
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => {
                                println!("No more songs! Only bundling now!");
                                drop(tx_name);
                                drop(tx_files);
                                break;
                            }
                        }
                        match rx_bundle_job.try_recv() {
                            Ok((sfat, bundle_files)) => {
                                bundle::save_songs_bundle(
                                    &sfat,
                                    &bundle_files,
                                    config,
                                    destination,
                                )
                                .unwrap();
                            }
                            Err(TryRecvError::Empty | TryRecvError::Disconnected) => {}
                        }
                    }
                    loop {
                        match rx_bundle_job.try_recv() {
                            Ok((sfat, bundle_files)) => {
                                bundle::save_songs_bundle(
                                    &sfat,
                                    &bundle_files,
                                    config,
                                    destination,
                                )
                                .unwrap();
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => {
                                println!("Finished bundling!");
                                break;
                            }
                        }
                    }
                })
                .unwrap();
        }

        // Only start as many threads as there are cpus (excluding the main thread, which will be waiting and doing nothing the entire time)
        for i in 0..n_threads.saturating_sub(3) {
            let tx_name = tx_name.clone();
            let rx_job = rx_job.clone();
            let tx_files = tx_files.clone();
            let rx_bundle_job = rx_bundle_job.clone();
            std::thread::Builder::new()
                .name(format!("Songs ({i})"))
                .spawn_scoped(s, move || {
                    loop {
                        match rx_job.try_recv() {
                            Ok(job) => {
                                let mut bf = BuildFiles {
                                    generated_files: VecFs::with_capacity(100),
                                    static_files: SymlinkFs::with_capacity(native_vfs, 50),
                                };
                                let songname =
                                    build::song::build(build_state, &mut bf, job).unwrap();
                                tx_name.send(songname).unwrap();
                                tx_files.send(FilesToAdd::Song(bf)).unwrap();
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => {
                                println!("No more songs! Only bundling now!");
                                drop(tx_name);
                                drop(tx_files);
                                break;
                            }
                        }
                        match rx_bundle_job.try_recv() {
                            Ok((sfat, bundle_files)) => {
                                bundle::save_songs_bundle(
                                    &sfat,
                                    &bundle_files,
                                    config,
                                    destination,
                                )
                                .unwrap();
                            }
                            Err(TryRecvError::Empty | TryRecvError::Disconnected) => {}
                        }
                    }
                    loop {
                        match rx_bundle_job.try_recv() {
                            Ok((sfat, bundle_files)) => {
                                bundle::save_songs_bundle(
                                    &sfat,
                                    &bundle_files,
                                    config,
                                    destination,
                                )
                                .unwrap();
                            }
                            Err(TryRecvError::Empty) => {}
                            Err(TryRecvError::Disconnected) => {
                                println!("Finished bundling!");
                                break;
                            }
                        }
                    }
                })
                .unwrap();
        }

        // If these aren't dropped before the scope ends, the threads will infinitely wait for more jobs
        drop(tx_files);
        drop(tx_name);
    });

    println!("Done!");

    Ok(())
}
