//! # Bundle
//! Contains the code for bundling files into .ipk files
use std::{fs::File, path::Path};

use anyhow::{anyhow, Error};
use crossbeam::channel::Receiver;
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
pub fn bundle(
    bundle_vfs: &IpkFilesystem<'_>,
    patch_vfs: &IpkFilesystem<'_>,
    native_vfs: &NativeFs,
    rx: &Receiver<FilesToAdd>,
    config: Config,
    destination: &Path,
    patch: bool,
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

    let mut sfat = SecureFat::with_capacity(config.game_platform, 30_000);

    let mut bundle_n = 0;

    // Loop while the channel we receive files on is still open
    loop {
        match rx.recv() {
            Ok(FilesToAdd::Bundle(new_bundle_files)) => {
                bundle_files.merge(new_bundle_files)?;
            }
            Ok(FilesToAdd::Song(new_song_files)) => {
                // Check if the song bundle would be too big if this song is added
                if song_files.size()? + new_song_files.size()? >= MAX_BUNDLE_SIZE_FAT32 {
                    // Save this bundle and create a new one
                    save_songs_bundle(&mut sfat, &song_files, bundle_n, config, destination)?;
                    bundle_n = bundle_n
                        .checked_add(1)
                        .ok_or_else(|| anyhow!("Overflow occurred"))?;
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
    save_songs_bundle(&mut sfat, &song_files, bundle_n, config, destination)?;

    if patch {
        // Create a patch file instead of a bundle file
        println!("Creating patch bundle");
        let bundle_files_vfs =
            OverlayFs::new(&bundle_files.generated_files, &bundle_files.static_files);
        let vfs = OverlayFs::new(&bundle_files_vfs, patch_vfs);
        let files = vfs.walk_filesystem("".as_ref())?;
        let filenames: Vec<_> = files.collect::<Vec<_>>();

        ipk::create(
            destination.join("patch_nx.ipk"),
            config.game_platform,
            config.ipk_unk4,
            config.engine_version,
            ipk::Options::default(),
            &vfs,
            &filenames,
        )?;

        // Add the original bundle to the sfat
        let bundle_id = sfat.add_bundle("bundle".to_string());
        // Link all the original file paths to the bundle
        sfat.add_path_ids_to_bundle(
            bundle_id,
            bundle_vfs.walk_filesystem("".as_ref())?.map(PathId::from),
        );
    } else {
        // Create empty patch file
        let mut patch_file = File::create(destination.join("patch_nx.ipk"))?;
        ipk::write(
            &mut patch_file,
            &mut 0,
            config.game_platform,
            config.ipk_unk4,
            config.engine_version,
            ipk::Options::default(),
            native_vfs,
            &[],
        )?;

        // Create main bundle
        println!("Creating main bundle");
        let bundle_files_vfs =
            OverlayFs::new(&bundle_files.generated_files, &bundle_files.static_files);
        let patched_bundle_vfs = OverlayFs::new(patch_vfs, bundle_vfs);
        let vfs = OverlayFs::new(&bundle_files_vfs, &patched_bundle_vfs);
        let files = vfs.walk_filesystem("".as_ref())?;
        let filenames: Vec<_> = files.collect::<Vec<_>>();

        ipk::create(
            destination.join("bundle_nx.ipk"),
            config.game_platform,
            config.ipk_unk4,
            config.engine_version,
            ipk::Options::default(),
            &vfs,
            &filenames,
        )?;

        // Add the bundle to the sfat
        let bundle_id = sfat.add_bundle("bundle".to_string());
        // Link all the file paths to the bundle
        sfat.add_path_ids_to_bundle(
            bundle_id,
            vfs.walk_filesystem("".as_ref())?.map(PathId::from),
        );
    }

    // Create secure_fat.gf
    println!("Creating secure_fat.gf");
    secure_fat::create(destination.join("secure_fat.gf"), &sfat)?;

    Ok(())
}

/// Create the nth bundle file with songs
fn save_songs_bundle(
    sfat: &mut SecureFat,
    bundle_files: &BuildFiles,
    bundle_n: u8,
    config: Config,
    destination: &Path,
) -> Result<(), Error> {
    let name = format!("songs_{bundle_n}");
    println!("Creating bundle {name}");

    let overlay_vfs = OverlayFs::new(&bundle_files.generated_files, &bundle_files.static_files);
    let files = overlay_vfs.walk_filesystem("".as_ref())?;
    let filenames: Vec<_> = files.collect::<Vec<_>>();

    ipk::create(
        destination.join(format!("{name}_nx.ipk")),
        config.game_platform,
        config.ipk_unk4,
        config.engine_version,
        ipk::Options::default(),
        &overlay_vfs,
        &filenames,
    )?;

    // Add the bundle to the sfat
    let bundle_id = sfat.add_bundle(name);
    // Link all the file paths to the bundle
    sfat.add_path_ids_to_bundle(
        bundle_id,
        overlay_vfs.walk_filesystem("".as_ref())?.map(PathId::from),
    );

    Ok(())
}
