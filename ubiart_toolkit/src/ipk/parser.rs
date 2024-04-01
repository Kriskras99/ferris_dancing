//! Contains the parser implementation for IPK bundles

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32be, u64be},
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::{test_any, test_eq, TestResult},
};
use nohash_hasher::{BuildNoHashHasher, IntMap};

use super::{
    types::Platform, Bundle, Compressed, Data, IpkFile, Uncompressed, IS_COOKED, MAGIC, SEPARATOR,
    UNK1, UNK2, UNK3, UNK6,
};
use crate::utils::{self, string_id_2, Game, PathId, SplitPath, UniqueGameId};

impl<'de> BinaryDeserialize<'de> for Bundle<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        // Read the header
        let magic = reader.read_at::<u32be>(position)?.into();
        test_eq(&magic, &MAGIC)?;
        let version = reader.read_at::<u32be>(position)?.into();
        let platform = reader.read_at::<Platform>(position)?;
        let base_offset = u64::from(reader.read_at::<u32be>(position)?);
        let num_files = u32::from(reader.read_at::<u32be>(position)?);
        let unk1 = reader.read_at::<u32be>(position)?.into();
        test_any(&unk1, UNK1)?;
        let unk2 = reader.read_at::<u32be>(position)?.into();
        test_any(&unk2, UNK2)?;
        let unk3 = reader.read_at::<u32be>(position)?.into();
        test_any(&unk3, UNK3)?;
        let unk4 = reader.read_at::<u32be>(position)?.into();
        let game_platform = reader.read_at::<UniqueGameId>(position)?;
        let engine_version = reader.read_at::<u32be>(position)?.into();
        let num_files_2 = reader.read_at::<u32be>(position)?.into();

        // Sanity check
        test_eq(&num_files, &num_files_2)?;
        if !platform.matches_game_platform(game_platform) {
            println!("Header: Warning! Platform (0x{:x} ({platform:?})) does not match GamePlatformId (0x{:x} ({game_platform}))!", u32::from(platform), u32::from(game_platform));
        }

        // Prepare for storing a lot of file info
        let mut files = IntMap::with_capacity_and_hasher(
            usize::try_from(num_files)?,
            BuildNoHashHasher::default(),
        );
        for _ in 0..num_files {
            // Read the file information
            let unk6 = reader.read_at::<u32be>(position)?.into();
            test_eq(&unk6, &UNK6)?;
            let size = usize::try_from(reader.read_at::<u32be>(position)?)?;
            let compressed_size = usize::try_from(reader.read_at::<u32be>(position)?)?;
            let timestamp = reader.read_at::<u64be>(position)?.into();
            let offset = reader.read_at::<u64be>(position)?.into();
            // Can't use read_at::<SplitPath> as the filename and path are swapped on JD2014 for the Wii
            let mut filename = reader.read_len_string_at::<u32be>(position)?;
            let mut path = reader.read_len_string_at::<u32be>(position)?;
            assert!(path.ends_with('/'));
            let path_id = reader.read_at::<PathId>(position)?;
            let is_cooked_u32 = reader.read_at::<u32be>(position)?.into();
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

            let path_id_calculated = string_id_2(path.as_ref(), filename.as_ref());
            test_eq(&*path_id, &path_id_calculated).with_context(|| format!("Path ID of {path}{filename} is {path_id_calculated}, but does not match what is in the file: {path_id:?}"))?;

            // Derive info from file information
            let is_cooked = is_cooked_u32 == 0x2;
            let is_compressed = compressed_size != 0;

            // Compute the right offset and size
            let asize = if is_compressed { compressed_size } else { size };
            let mut foff_from = base_offset.checked_add(offset).ok_or_else(|| {
                ReadError::custom(format!(
                    "Cannot add {base_offset} and {offset}, it would overflow!"
                ))
            })?;
            let data = reader.read_slice_at(&mut foff_from, asize)?;

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
                path: SplitPath::new(path, filename)?,
                is_cooked,
                data,
            };

            // Add file to the Vec
            files.insert(path_id, file);
        }

        let header_end = *position;

        if game_platform.platform == utils::Platform::Nx
            && (game_platform.game == Game::JustDance2020
                || game_platform.game == Game::JustDance2021
                || game_platform.game == Game::JustDance2022
                || game_platform.game == Game::JustDanceChina)
        {
            // Make sure the separator is here
            match test_eq(&(header_end + 0x4), &base_offset) {
                TestResult::Ok => {
                    let separator = reader.read_at::<u32be>(position)?.into();
                    test_eq(&separator, &SEPARATOR)?;
                }
                result @ TestResult::Err(_) => result?,
            }
        } else {
            // Make sure the separator is not here
            test_eq(&header_end, &base_offset)
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
}

impl BinaryDeserialize<'_> for Platform {
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        match u32::from(reader.read_at::<u32be>(position)?) {
            0x1 => Ok(Self::X360),
            0x3 => Ok(Self::Ps4),
            0x5 => Ok(Self::Wii),
            0x8 => Ok(Self::WiiU),
            0xB => Ok(Self::Nx),
            value => Err(ReadError::custom(format!("Unknown platform id {value}!"))),
        }
    }
}
