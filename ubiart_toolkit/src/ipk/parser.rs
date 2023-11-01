//! Contains the parser implementation for IPK bundles

use std::{borrow::Cow, fs, path::Path};

use anyhow::{anyhow, Context, Error};
use byteorder::BigEndian;
use memmap2::Mmap;
use nohash_hasher::{BuildNoHashHasher, IntMap};
use yoke::Yoke;

use crate::utils::{
    self,
    bytes::{read_string_at, read_u32_at, read_u64_at},
    string_id_2,
    testing::{test, test_any},
    Game, GamePlatform, PathId, SplitPath,
};

use super::{
    types::Platform, Bundle, BundleOwned, Compressed, Data, IpkFile, Uncompressed, IS_COOKED,
    MAGIC, SEPARATOR, UNK1, UNK2, UNK3, UNK6,
};

/// Check if the source is likely to be a IPK bundle
///
/// This is currently done by checking for the magic number.
#[must_use]
pub fn can_parse(source: [u8; 4]) -> bool {
    read_u32_at::<BigEndian>(&source, &mut 0).unwrap_or_else(|_| unreachable!()) == MAGIC
}

/// Open the file at the given path and parse it as a secure_fat.gf
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<BundleOwned<Mmap>, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let yoke = Yoke::try_attach_to_cart(mmap, |data: &[u8]| parse(data));
    Ok(BundleOwned::from(yoke?))
}

/// Parse a bytearray-like source as a IPK bundle
///
/// This will parse the source from start to end.
///
/// # Errors
/// This function will error when it encounters the following:
/// - Unexpected values (i.e. wrong magic)
/// - Invalid UTF-8 (i.e. in paths)
/// - Source has an unexpected size (i.e. not enough bytes, or too many bytes)
pub fn parse(src: &[u8]) -> Result<Bundle, anyhow::Error> {
    // Keep track of where we are
    let mut pos = 0;
    // Read the header
    let magic = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&magic, &MAGIC)?;
    let version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let platform = Platform::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let base_offset = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let num_files = read_u32_at::<BigEndian>(src, &mut pos)?;
    let unk1 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk1, UNK1)?;
    let unk2 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk2, UNK2)?;
    let unk3 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk3, UNK3)?;
    let unk4 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let game_platform = GamePlatform::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let engine_version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let num_files_2 = read_u32_at::<BigEndian>(src, &mut pos)?;

    // Sanity check
    test(&num_files, &num_files_2)?;
    if !platform.matches_game_platform(game_platform) {
        println!("Header: Warning! Platform (0x{:x} ({platform:?})) does not match GamePlatformId (0x{:x} ({game_platform}))!", u32::from(platform), u32::from(game_platform));
    }

    // Prepare for storing a lot of file info
    let mut files =
        IntMap::with_capacity_and_hasher(usize::try_from(num_files)?, BuildNoHashHasher::default());
    for _ in 0..num_files {
        // Read the file information
        let unk6 = read_u32_at::<BigEndian>(src, &mut pos)?;
        test(&unk6, &UNK6)?;
        let size = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
        let compressed_size = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
        let timestamp = read_u64_at::<BigEndian>(src, &mut pos)?;
        let offset = usize::try_from(read_u64_at::<BigEndian>(src, &mut pos)?)?;
        let mut filename = read_string_at::<BigEndian>(src, &mut pos)?;
        let mut path = read_string_at::<BigEndian>(src, &mut pos)?;
        let path_id = PathId::from(read_u32_at::<BigEndian>(src, &mut pos)?);
        let is_cooked_u32 = read_u32_at::<BigEndian>(src, &mut pos)?;
        test_any(&is_cooked_u32, IS_COOKED)?;

        // This is swapped for one game
        if game_platform.game == Game::JustDance2014 && platform == Platform::Wii {
            (filename, path) = (path, filename);
        } else if path.contains('.') || filename.contains('/') {
            println!("Warning! Had to switch path and name! {game_platform:?}");
            (filename, path) = (path, filename);
        }

        let path_id_calculated = string_id_2(path, filename);
        test(&*path_id, &path_id_calculated).with_context(|| format!("Path ID of {path}{filename} is {path_id_calculated}, but does not match what is in the file: {path_id:?}"))?;

        // Derive info from file information
        let is_cooked = is_cooked_u32 == 0x2;
        let is_compressed = compressed_size != 0;

        // Compute the right offset and size
        let asize = if is_compressed { compressed_size } else { size };
        let foff_from = base_offset
            .checked_add(offset)
            .ok_or_else(|| anyhow!("Cannot add {base_offset} and {offset}, it would overflow!"))?;
        let foff_to = foff_from
            .checked_add(asize)
            .ok_or_else(|| anyhow!("Cannot add {foff_from} and {asize}, it would overflow!"))?;
        let data = &src[foff_from..foff_to];

        let data = if is_compressed {
            Data::Compressed(Compressed {
                uncompressed_size: size,
                data,
            })
        } else {
            Data::Uncompressed(Uncompressed { data })
        };
        let file = IpkFile {
            timestamp,
            path: SplitPath {
                path: Cow::Borrowed(path),
                filename: Cow::Borrowed(filename),
            },
            is_cooked,
            data,
        };

        // Add file to the Vec
        files.insert(path_id, file);
    }

    let header_end = pos;

    if game_platform.platform == utils::Platform::Nx
        && (game_platform.game == Game::JustDance2020
            || game_platform.game == Game::JustDance2021
            || game_platform.game == Game::JustDance2022
            || game_platform.game == Game::JustDanceChina)
    {
        // Make sure the separator is here
        test(&(header_end + 0x4), &base_offset)?;
        let separator = read_u32_at::<BigEndian>(src, &mut pos)?;
        test(&separator, &SEPARATOR)?;
    } else {
        // Make sure the separator is not here
        test(&header_end, &base_offset)
            .context("Found unexpected separator between header and files!")?;
    };

    Ok(Bundle {
        version,
        platform,
        unk4,
        engine_version,
        game_platform,
        files,
    })
}

/// Parse a bytearray-like source as a IPK bundle
///
/// This will parse the source from start to end.
///
/// # Errors
/// This function will error when it encounters the following:
/// - Unexpected values (i.e. wrong magic)
/// - Invalid UTF-8 (i.e. in paths)
/// - Source has an unexpected size (i.e. not enough bytes, or too many bytes)
pub fn parse_lax(src: &[u8]) -> Result<Vec<IpkFile<'_>>, anyhow::Error> {
    // Keep track of where we are
    let mut pos = 0;
    // Read the header
    let magic = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&magic, &MAGIC)?;
    let _version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _platform = read_u32_at::<BigEndian>(src, &mut pos)?;
    let base_offset = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let num_files = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _unk1 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _unk2 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _unk3 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _unk4 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let game_platform = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _engine_version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let _num_files_2 = read_u32_at::<BigEndian>(src, &mut pos)?;

    // Prepare for storing a lot of file info
    let mut files = Vec::with_capacity(usize::try_from(num_files)?);
    for _ in 0..num_files {
        // Read the file information
        let _unk6 = read_u32_at::<BigEndian>(src, &mut pos)?;
        let size = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
        let compressed_size = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
        let timestamp = read_u64_at::<BigEndian>(src, &mut pos)?;
        let offset = usize::try_from(read_u64_at::<BigEndian>(src, &mut pos)?)?;
        let mut filename = read_string_at::<BigEndian>(src, &mut pos)?;
        let mut path = read_string_at::<BigEndian>(src, &mut pos)?;
        let path_id = PathId::from(read_u32_at::<BigEndian>(src, &mut pos)?);
        let is_cooked_u32 = read_u32_at::<BigEndian>(src, &mut pos)?;
        test_any(&is_cooked_u32, IS_COOKED)?;

        // This is swapped for one game
        if path.contains('.') || filename.contains('/') {
            println!("Warning! Had to switch path and name! {game_platform:?}");
            (filename, path) = (path, filename);
        }

        let path_id_calculated = string_id_2(path, filename);
        test(&*path_id, &path_id_calculated).with_context(|| format!("Path ID of {path}{filename} is {path_id_calculated}, but does not match what is in the file: {path_id:?}"))?;

        // Derive info from file information
        let is_cooked = is_cooked_u32 == 0x2;
        let is_compressed = compressed_size != 0;

        // Compute the right offset and size
        let asize = if is_compressed { compressed_size } else { size };
        let foff_from = base_offset
            .checked_add(offset)
            .ok_or_else(|| anyhow!("Cannot add {base_offset} and {offset}, it would overflow!"))?;
        let foff_to = foff_from
            .checked_add(asize)
            .ok_or_else(|| anyhow!("Cannot add {foff_from} and {asize}, it would overflow!"))?;
        let data = &src[base_offset + offset..foff_to];

        let data = if is_compressed {
            Data::Compressed(Compressed {
                uncompressed_size: size,
                data,
            })
        } else {
            Data::Uncompressed(Uncompressed { data })
        };
        let file = IpkFile {
            timestamp,
            path: SplitPath {
                path: Cow::Borrowed(path),
                filename: Cow::Borrowed(filename),
            },
            is_cooked,
            data,
        };

        // Add file to the Vec
        files.push(file);
    }

    Ok(files)
}
