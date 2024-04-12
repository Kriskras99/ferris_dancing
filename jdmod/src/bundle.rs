//! # Extract
//! Code for extracting a UbiArt archive (ipk or gf)
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Error};
use clap::Args;
use dotstar_toolkit_utils::vfs::{native::NativeFs, VirtualFileSystem, VirtualPath};
use ubiart_toolkit::{
    ipk,
    secure_fat::{self, SecureFat},
    utils::{PathId, UniqueGameId},
};

use crate::types::Config;

/// The maximum size of a file for the FAT32 filesystem
const MAX_BUNDLE_SIZE_FAT32: u64 = 4_294_967_295;

/// Extract a UbiArt archive
#[derive(Args, Clone)]
pub struct Bundle {
    /// Directory to bundle
    source: PathBuf,
    /// Directory to put the bundled files
    destination: PathBuf,
    /// Config file to use (instead of manually specifying the values)
    #[arg(short, long = "config")]
    config_path: Option<PathBuf>,
    /// The GamePlatform version to use
    #[arg(long)]
    game_platform: Option<u32>,
    /// The engine version to use
    #[arg(long)]
    engine_version: Option<u32>,
    /// The IPK unk4 value to use
    #[arg(long)]
    ipk_unk4: Option<u32>,
    /// Only create a patch_nx.ipk
    #[arg(long)]
    patch: bool,
}

/// Bundle the files at `source` to `destination`
pub fn main(data: &Bundle) -> Result<(), Error> {
    let config = if let Some(config_path) = &data.config_path {
        let file = File::open(config_path)?;
        let config: Config = serde_json::from_reader(file)?;
        config
    } else {
        let gp = data
            .game_platform
            .ok_or_else(|| anyhow!("Missing --game-platform or --config"))?;
        let ev = data
            .engine_version
            .ok_or_else(|| anyhow!("Missing --engine-version or --config"))?;
        let iu = data
            .ipk_unk4
            .ok_or_else(|| anyhow!("Missing --ipk-unk4 or --config"))?;
        Config {
            game_platform: UniqueGameId::try_from(gp)?,
            engine_version: ev,
            ipk_unk4: iu,
        }
    };
    bundle(&data.source, &data.destination, &config, data.patch)
}

/// Bundle the files at `source` into .ipks at `destination`
///
/// If `patch` is specified, only a "patch_nx.ipk" file is created.
pub fn bundle(
    source: &Path,
    destination: &Path,
    config: &Config,
    patch: bool,
) -> Result<(), Error> {
    // Check the source directory
    if !source.exists() {
        bail!("Source directory does not exist! {source:?}");
    } else if !source.is_dir() {
        bail!("Source is not a directory! {source:?}");
    }
    // Create the destination directory
    if destination.exists() {
        if !destination.is_dir() {
            bail!("Destination directory is not a directory! {destination:?}");
        } else if destination.read_dir()?.next().is_some() {
            bail!("Destination directory is not empty! {destination:?}");
        }
    } else {
        std::fs::create_dir(destination)?;
    }

    let vfs = NativeFs::new(source)?;
    let files = vfs.walk_filesystem("".as_ref())?;
    let file_count = files.len();
    let filenames: Vec<_> = files.collect::<Vec<_>>();

    if patch {
        let file_path = destination.join("patch_nx.ipk");
        ipk::create(
            &file_path,
            ipk::Options {
                compression: ipk::CompressionEffort::Best,
                game_platform: config.game_platform,
                unk4: config.ipk_unk4,
                engine_version: config.engine_version,
            },
            &vfs,
            &filenames,
        )?;
        if file_path.metadata()?.len() >= MAX_BUNDLE_SIZE_FAT32 {
            println!("Warning! patch_nx.ipk file is bigger than 4 GB and therefore not compatible with the FAT32 filesystem.");
        }
    } else {
        // The main bundle that contains all the logic, but no songs
        let mut main_bundle_entries = Vec::new();
        // The size of the main bundle, if this ends up larger than `MAX_BUNDLE_SIZE_FAT32` then FAT32 is not supported
        let mut main_bundle_size = 0;
        // Track the total size of the mod
        let mut total_size = 0u64;
        // Collects all the files that belong to a song along with the total size of the song
        let mut song_bundles: HashMap<String, (u64, Vec<&VirtualPath>)> = HashMap::new();

        // Extract common string
        let path_cache_maps = "cache/itf_cooked/nx/world/maps/";
        let path_maps = "world/maps/";

        for path in filenames {
            let file_size = vfs.metadata(path.as_ref())?.file_size();
            total_size += file_size;
            /*
            Files that start with word/maps/{lower_map_name}/ and cache/itf_cooked/nx/world/{lower_map_name}
            generally do not go into bundle_nx.ipk except for the phone images (_cover_phone.jpg, _coach_{n}_phone.png),
            song description (songdesc.tpl.ckd, songdesc.act.ckd), and the textures and actors (_cover_{generic,online}.{tga,act}.ckd)
            */
            // Check if the file belongs to a song or the main bundle
            if (!path.starts_with(path_cache_maps) && !path.starts_with(path_maps))
                || (path.starts_with(path_maps) && path.ends_with("_phone.png"))
                || (path.starts_with(path_cache_maps)
                    && (path.ends_with("_cover_generic.act.ckd")
                        || path.ends_with("_cover_online.act.ckd")
                        || path.ends_with("_cover_generic.tga.ckd")
                        || path.ends_with("_cover_online.tga.ckd")
                        || path.ends_with("/songdesc.act.ckd")
                        || path.ends_with("/songdesc.tpl.ckd")))
            {
                // Add the file to the main bundle
                main_bundle_size += file_size;
                main_bundle_entries.push(path);
            } else {
                let path_str = path.as_str();
                // Extract the map name from the path
                let mut map_name = if path_str.starts_with(path_cache_maps) {
                    path_str.replace(path_cache_maps, "")
                } else if path_str.starts_with(path_maps) {
                    path_str.replace(path_maps, "")
                } else {
                    bail!("File doesn't belong anywhere!");
                };
                map_name.truncate(
                    map_name
                        .find('/')
                        .ok_or_else(|| anyhow!("Invalid path! {path_str}"))?,
                );
                // If the song already exists add the information
                match song_bundles.entry(map_name) {
                    Entry::Occupied(mut entry) => {
                        let (size, paths) = entry.get_mut();
                        *size += file_size;
                        paths.push(path);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert((file_size, vec![path]));
                    }
                };
            }
        }

        // Sort the songs by size, so we can greedily divide the data as much as possible
        // The biggest bundles will be at the back of the vector
        let mut sorted_sizes: Vec<(&str, u64)> = song_bundles
            .iter()
            .map(|(key, val)| (key.as_ref(), val.0))
            .collect();
        sorted_sizes.sort_by_key(|(_, filesize)| *filesize);

        // Collects the bundles to be packed
        let mut other_bundles_entries = vec![(0, Vec::new()); 1];

        // As the vec is read from the back, this index will be substracted from the current length
        let mut index = 1;
        // Go over all the song bundles
        while !sorted_sizes.is_empty() {
            // The amount of remaining songs
            let current_len = sorted_sizes.len();
            // Extract the current bundle for easy access
            let current_bundle = other_bundles_entries
                .last_mut()
                .unwrap_or_else(|| unreachable!());
            // Try all songs
            while index <= current_len {
                // Extract the current song from the vec
                let current_sorted = sorted_sizes[current_len - index];
                // Check if it would fit in the bundle
                if current_sorted.1 + current_bundle.0 <= MAX_BUNDLE_SIZE_FAT32 {
                    // Remove the `DirEntry` from the entries
                    let entry = song_bundles
                        .get(current_sorted.0)
                        .unwrap_or_else(|| unreachable!());
                    // Record the new bundle size
                    current_bundle.0 += current_sorted.1;
                    // Add all the files
                    current_bundle.1.extend_from_slice(&entry.1);
                    // Also remove it from the vec, otherwise we loop indefinitely
                    sorted_sizes.remove(current_len - index);
                    // Start over
                    break;
                }
                // Didn't fit, try the next one
                index += 1;
            }
            // If all songs were tried, start a new bundle
            if index > current_len {
                other_bundles_entries.push((0, Vec::new()));
                index = 1;
            }
        }

        // Save some memory
        drop(sorted_sizes); // is empty now but still has capacity
        drop(song_bundles);

        println!(
            "Total size: {total_size} bytes ({} GB)",
            total_size / 1024 / 1024 / 1024
        );
        println!(
            "Main bundle: {} files, {main_bundle_size} bytes",
            main_bundle_entries.len()
        );
        for (i, (size, files)) in other_bundles_entries.iter().enumerate() {
            println!("Bundle {i}: {} files, {size} bytes", files.len());
        }

        let mut sfat = SecureFat::with_capacity(config.game_platform, file_count);

        println!("Creating bundle_nx.ipk");
        ipk::create(
            destination.join("bundle_nx.ipk"),
            ipk::Options {
                compression: ipk::CompressionEffort::Best,
                game_platform: config.game_platform,
                unk4: config.ipk_unk4,
                engine_version: config.engine_version,
            },
            &vfs,
            &main_bundle_entries,
        )?;

        let bundle_id = sfat.add_bundle(String::from("bundle"));
        sfat.add_path_ids_to_bundle(
            bundle_id,
            main_bundle_entries.iter().map(|path| PathId::from(*path)),
        );

        for (i, (_, entries)) in other_bundles_entries.iter().enumerate() {
            let name = format!("songs_{i}_nx.ipk");
            println!("Creating {name}");
            ipk::create(
                destination.join(&name),
                ipk::Options {
                    compression: ipk::CompressionEffort::Best,
                    game_platform: config.game_platform,
                    unk4: config.ipk_unk4,
                    engine_version: config.engine_version,
                },
                &vfs,
                entries,
            )?;

            let bundle_id = sfat.add_bundle(name);
            sfat.add_path_ids_to_bundle(bundle_id, entries.iter().map(|path| PathId::from(*path)));
        }

        println!("Creating patch_nx.ipk");
        ipk::create(
            destination.join("patch_nx.ipk"),
            ipk::Options {
                compression: ipk::CompressionEffort::Best,
                game_platform: config.game_platform,
                unk4: config.ipk_unk4,
                engine_version: config.engine_version,
            },
            &vfs,
            &[],
        )?;

        println!("Creating secure_fat.gf");
        secure_fat::create(destination.join("secure_fat.gf"), &sfat)?;
    }

    Ok(())
}
