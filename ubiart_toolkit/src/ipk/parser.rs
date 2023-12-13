//! Contains the parser implementation for IPK bundles

use std::borrow::Cow;

use anyhow::anyhow;
use byteorder::BigEndian;
use dotstar_toolkit_utils::testing::TestResult;
use dotstar_toolkit_utils::testing::{test, test_any};
use nohash_hasher::{BuildNoHashHasher, IntMap};

use super::{
    types::Platform, Bundle, Compressed, Data, IpkFile, Uncompressed, IS_COOKED, MAGIC, SEPARATOR,
    UNK1, UNK2, UNK3, UNK6,
};
use crate::utils::{
    self,
    bytes::{read_string_at, read_u32_at, read_u64_at},
    string_id_2, Game, GamePlatform, PathId, SplitPath,
};

/// Parse a bytearray-like source as a IPK bundle
///
/// This will parse the source from start to end.
///
/// # Errors
/// This function will error when it encounters the following:
/// - Unexpected values (i.e. wrong magic)
/// - Invalid UTF-8 (i.e. in paths)
/// - Source has an unexpected size (i.e. not enough bytes, or too many bytes)
pub fn parse(src: &[u8], lax: bool) -> Result<Bundle, anyhow::Error> {
    // Keep track of where we are
    let mut pos = 0;
    // Read the header
    let magic = read_u32_at::<BigEndian>(src, &mut pos)?;
    test(&magic, &MAGIC)?;
    let version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let platform = match (
        Platform::try_from(read_u32_at::<BigEndian>(src, &mut pos)?),
        lax,
    ) {
        (Ok(platform), _) => platform,
        (Err(_), true) => Platform::Nx,
        (err @ Err(_), false) => err?,
    };
    let base_offset = usize::try_from(read_u32_at::<BigEndian>(src, &mut pos)?)?;
    let num_files = read_u32_at::<BigEndian>(src, &mut pos)?;
    let unk1 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk1, UNK1).lax(lax)?;
    let unk2 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk2, UNK2).lax(lax)?;
    let unk3 = read_u32_at::<BigEndian>(src, &mut pos)?;
    test_any(&unk3, UNK3).lax(lax)?;
    let unk4 = read_u32_at::<BigEndian>(src, &mut pos)?;
    let game_platform = match (
        GamePlatform::try_from(read_u32_at::<BigEndian>(src, &mut pos)?),
        lax,
    ) {
        (Ok(game_platform), _) => game_platform,
        (Err(_), true) => GamePlatform {
            game: Game::JustDance2022,
            platform: utils::Platform::Nx,
            id: 0x1DDB_2268,
        },
        (err @ Err(_), false) => err?,
    };
    let engine_version = read_u32_at::<BigEndian>(src, &mut pos)?;
    let num_files_2 = read_u32_at::<BigEndian>(src, &mut pos)?;

    // Sanity check
    test(&num_files, &num_files_2).lax(lax)?;
    if !platform.matches_game_platform(game_platform) {
        println!("Header: Warning! Platform (0x{:x} ({platform:?})) does not match GamePlatformId (0x{:x} ({game_platform}))!", u32::from(platform), u32::from(game_platform));
    }

    // Prepare for storing a lot of file info
    let mut files =
        IntMap::with_capacity_and_hasher(usize::try_from(num_files)?, BuildNoHashHasher::default());
    for _ in 0..num_files {
        // Read the file information
        let unk6 = read_u32_at::<BigEndian>(src, &mut pos)?;
        test(&unk6, &UNK6).lax(lax)?;
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
        if game_platform.game == Game::JustDance2014
            && game_platform.platform == crate::utils::Platform::Wii
        {
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
        match test(&(header_end + 0x4), &base_offset) {
            TestResult::Ok => {
                let separator = read_u32_at::<BigEndian>(src, &mut pos)?;
                test(&separator, &SEPARATOR).lax(lax)?;
            }
            result @ TestResult::Err(_) => result.lax(lax)?,
        }
    } else {
        // Make sure the separator is not here
        test(&header_end, &base_offset)
            .context("Found unexpected separator between header and files!")
            .lax(lax)?;
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
