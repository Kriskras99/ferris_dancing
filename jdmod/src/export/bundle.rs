//! # Bundle
//! Contains the code for bundling files into .ipk files
use std::{
    fs::File,
    path::Path,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex,
    },
};

use anyhow::{anyhow, Error};
use crossbeam::channel::{Receiver, Sender};
use dotstar_toolkit_utils::{
    testing::test,
    vfs::{
        layeredfs::OverlayFs, native::NativeFs, symlinkfs::SymlinkFs, vecfs::VecFs,
        VirtualFileSystem,
    },
};
use ubiart_toolkit::{
    ipk::{self, vfs::IpkFilesystem},
    secure_fat::{self, SecureFat},
    utils::PathId,
};

use super::FilesToAdd;
use crate::{build::BuildFiles, types::Config};

/// Maximum file size for FAT32
const MAX_BUNDLE_SIZE_FAT32: u64 = 4_294_967_295;

/// Receives files in `rx` and bundles them into .ipk files at `destination`
pub fn bundle<'fs: 'bf, 'bf>(
    bundle_vfs: &IpkFilesystem<'_>,
    patch_vfs: &IpkFilesystem<'_>,
    native_vfs: &'fs NativeFs,
    rx_files: &Receiver<FilesToAdd>,
    tx_bundle_job: Sender<(Arc<Mutex<SecureFat>>, BuildFiles<'bf>)>,
    config: Config,
    destination: &Path,
) -> Result<(), Error> {
    // Make sure the destination directory actually exists
    test(destination.exists())
        .with_context(|| format!("Destination directory {destination:?} does not exist!"))?;

    // For files that go into the main (logic) bundle
    let mut bundle_files = BuildFiles {
        generated_files: VecFs::with_capacity(1000),
        static_files: SymlinkFs::with_capacity(native_vfs, 1000),
    };

    // For files that are only used when playing a song
    let mut song_files = BuildFiles {
        generated_files: VecFs::with_capacity(1000),
        static_files: SymlinkFs::with_capacity(native_vfs, 500),
    };

    let mut sfat = Arc::new(Mutex::new(SecureFat::with_capacity(
        config.game_platform,
        30_000,
    )));

    // Make sure that bundle_nx.ipk is the first bundle, so loading goes faster
    let main_bundle_id = { sfat.lock().unwrap().add_bundle("bundle".into()) };

    // Loop while the channel we receive files on is still open
    loop {
        match rx_files.recv() {
            Ok(FilesToAdd::Bundle(new_bundle_files)) => {
                bundle_files.merge(new_bundle_files)?;
            }
            Ok(FilesToAdd::Song(new_song_files)) => {
                // Check if the song bundle would be too big if this song is added
                if song_files.size()? + new_song_files.size()? >= MAX_BUNDLE_SIZE_FAT32 {
                    // Save this bundle and create a new one
                    tx_bundle_job.send((sfat.clone(), song_files)).unwrap();
                    song_files = BuildFiles {
                        generated_files: VecFs::with_capacity(1000),
                        static_files: SymlinkFs::with_capacity(native_vfs, 500),
                    };
                }
                for (path, content) in new_song_files.generated_files {
                    // Check if the file belongs to the main bundle
                    if path.ends_with("_cover_generic.act.ckd")
                        || path.ends_with("_cover_online.act.ckd")
                        || path.ends_with("_cover_generic.tga.ckd")
                        || path.ends_with("_cover_online.tga.ckd")
                        || path.ends_with("songdesc.act.ckd")
                        || path.ends_with("songdesc.tpl.ckd")
                    {
                        // Add the file to the main bundle
                        bundle_files.generated_files.add_file(path, content)?;
                    } else {
                        // Add the file to the song bundle
                        song_files.generated_files.add_file(path, content)?;
                    }
                }
                for (new_path, orig_path) in new_song_files.static_files {
                    // Check if the file belongs to the main bundle
                    if new_path.ends_with("_phone.png") {
                        // Add the file to the main bundle
                        bundle_files.static_files.add_file(orig_path, new_path)?;
                    } else {
                        // Add the file to the song bundle
                        song_files.static_files.add_file(orig_path, new_path)?;
                    }
                }
            }
            Err(_) => break,
        }
    }

    // Save last song bundle
    tx_bundle_job.send((sfat.clone(), song_files)).unwrap();
    // Other threads will keep waiting for jobs until this channel is closed/dropped
    drop(tx_bundle_job);

    println!("Bundling jobs done!");

    // Create empty patch file
    let mut patch_file = File::create(destination.join("patch_nx.ipk"))?;
    ipk::write(
        &mut patch_file,
        &mut 0,
        ipk::Options {
            compression: ipk::CompressionEffort::Best,
            game_platform: config.game_platform,
            unk4: config.ipk_unk4,
            engine_version: config.engine_version,
        },
        native_vfs,
        &[],
    )?;

    // Create main bundle
    println!("Creating main bundle");
    let bundle_files_vfs =
        OverlayFs::new(&bundle_files.generated_files, &bundle_files.static_files);
    let patched_bundle_vfs = OverlayFs::new(patch_vfs, bundle_vfs);
    let vfs = OverlayFs::new(&bundle_files_vfs, &patched_bundle_vfs);
    let filenames: Vec<_> = vfs.walk_filesystem("".as_ref())?.collect();

    ipk::create(
        destination.join("bundle_nx.ipk"),
        ipk::Options {
            compression: ipk::CompressionEffort::Best,
            game_platform: config.game_platform,
            unk4: config.ipk_unk4,
            engine_version: config.engine_version,
        },
        &vfs,
        &filenames,
    )?;

    {
        // Link all the file paths to the bundle
        sfat.lock()
            .unwrap()
            .add_path_ids_to_bundle(main_bundle_id, filenames.into_iter().map(PathId::from));
    }

    println!("Waiting for bundle threads to finish");

    // Wait until all bundles are added
    let sfat = loop {
        sfat = match Arc::try_unwrap(sfat) {
            Ok(unwrapped) => break unwrapped.into_inner().unwrap(),
            Err(still_shared) => still_shared,
        }
    };

    // Create secure_fat.gf
    println!("Creating secure_fat.gf");
    secure_fat::create(destination.join("secure_fat.gf"), &sfat)?;

    Ok(())
}

/// Atomic counter for the song bundle nummer
static BUNDLE_N: AtomicU8 = AtomicU8::new(0);

/// Create the nth bundle file with songs
pub fn save_songs_bundle(
    sfat: &Arc<Mutex<SecureFat>>,
    bundle_files: &BuildFiles,
    config: Config,
    destination: &Path,
) -> Result<(), Error> {
    let bundle_n = BUNDLE_N.fetch_add(1, Ordering::AcqRel);
    if bundle_n == u8::MAX {
        return Err(anyhow!("Too many bundles! "));
    }

    let name = format!("songs_{bundle_n}");
    println!("Creating bundle {name}");

    let overlay_vfs = OverlayFs::new(&bundle_files.generated_files, &bundle_files.static_files);
    let files = overlay_vfs.walk_filesystem("".as_ref())?;
    let filenames: Vec<_> = files.collect::<Vec<_>>();

    ipk::create(
        destination.join(format!("{name}_nx.ipk")),
        ipk::Options {
            compression: ipk::CompressionEffort::Best,
            game_platform: config.game_platform,
            unk4: config.ipk_unk4,
            engine_version: config.engine_version,
        },
        &overlay_vfs,
        &filenames,
    )?;

    {
        let mut sfat = sfat.lock().unwrap();
        // Add the bundle to the sfat
        let bundle_id = sfat.add_bundle(name.clone());
        // Link all the file paths to the bundle
        sfat.add_path_ids_to_bundle(bundle_id, filenames.into_iter().map(PathId::from));
    }

    println!("Finished bundle {name}");

    Ok(())
}
