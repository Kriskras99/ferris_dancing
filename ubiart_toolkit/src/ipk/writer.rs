use std::{fs::File, io::BufWriter, num::NonZeroU64, path::Path};

use dotstar_toolkit_utils::vfs::{VirtualFile, VirtualFileSystem};
use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32be, u64be},
        write::{BinarySerialize, WriteError, ZeroCopyWriteAt},
    },
    testing::test,
};

use super::{Platform, MAGIC};
use crate::utils::{self, Game, SplitPath, UniqueGameId};

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub compression: CompressionEffort,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            compression: CompressionEffort::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompressionEffort {
    None,
    Best,
    #[cfg(feature = "zopfli")]
    Zopfli(ZopfliOptions),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZopfliOptions {
    pub iteration_count: NonZeroU64,
    pub iterations_without_improvement: NonZeroU64,
}

impl Default for ZopfliOptions {
    fn default() -> Self {
        Self {
            iteration_count: NonZeroU64::new(15).unwrap(),
            iterations_without_improvement: NonZeroU64::MIN,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReducedMetadata<'a> {
    pub size: u64,
    pub compressed: u64,
    pub offset: u64,
    pub timestamp: u64,
    pub path: &'a str,
}

const STATIC_HEADER_SIZE: usize = 0x30;

/// Create a secure_fat.gf file at the path
pub fn create<P: AsRef<Path>>(
    path: P,
    game_platform: UniqueGameId,
    unk4: u32,
    engine_version: u32,
    options: Options,
    vfs: &impl VirtualFileSystem,
    files: &[&str],
) -> Result<(), WriteError> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write(
        &mut writer,
        &mut 0,
        game_platform,
        unk4,
        engine_version,
        options,
        vfs,
        files,
    )
}

/// Create an .ipk file with the specified files.
pub fn write(
    writer: &mut (impl ZeroCopyWriteAt + ?Sized),
    position: &mut u64,
    game_platform: UniqueGameId,
    unk4: u32,
    engine_version: u32,
    options: Options,
    vfs: &impl VirtualFileSystem,
    files: &[&str],
) -> Result<(), WriteError> {
    let start = *position;

    // Calculate the size of the header, starting with the static size
    let mut base_offset = STATIC_HEADER_SIZE;

    // On NX, JD2020-JD2022 have a 4 null bytes between the header and the content of the files
    if game_platform.platform == utils::Platform::Nx
        && (game_platform.game == Game::JustDance2020
            || game_platform.game == Game::JustDance2021
            || game_platform.game == Game::JustDance2022)
    {
        base_offset += 0x4;
    }

    // Add the static metadata size for every file plus the length of the path
    for path in files {
        base_offset += 0x2C + path.len(); // metadata size + path length
    }

    // Start writing the header
    writer.write_at(position, &u32be::from(MAGIC))?;
    writer.write_at(position, &u32be::from(0x5))?; // version
    writer.write_at(position, &Platform::from(game_platform.platform))?;
    writer.write_at(position, &u32be::try_from(base_offset)?)?;
    writer.write_at(position, &u32be::try_from(files.len())?)?;
    writer.write_at(position, &u32be::from(0x0))?; // unk1
    writer.write_at(position, &u32be::from(0x0))?; // unk2
    writer.write_at(position, &u32be::from(0x0))?; // unk3
    writer.write_at(position, &u32be::from(unk4))?;
    writer.write_at(position, &game_platform)?;
    writer.write_at(position, &u32be::from(engine_version))?;
    writer.write_at(position, &u32be::try_from(files.len())?)?;

    // Skip the file metadata for now, as it depends on compression results
    let base_offset = u64::try_from(base_offset)?;
    *position = base_offset;

    // For keeping track of the relevant metadata that needs to be written to the header
    let mut reduced_metadata = Vec::with_capacity(files.len());

    // Write the content of all files, while filling `reduced_metadata`
    for path in files {
        // The offset from the start of the file
        // NB: the metadata stores the offset relevant to the end of the header
        let raw_offset = *position - start;
        // This is presumably a timestamp, but the values don't add up. So we use a static value.
        let timestamp = 132_761_939_258_059_932;

        // Open the file and get the size
        let file = vfs.open(path.as_ref())?;
        let size = u64::try_from(file.len())?;

        // File content can be stored compressed.
        // Skip compression for small files, and already compressed files.
        let compressed = if size < 2048
            || path.ends_with("jpg")
            || path.ends_with("webm")
            || path.ends_with("ogg")
            || path.ends_with("png")
        {
            // Skip compression for already compressed files and small files
            writer.write_slice_at(position, &file)?;
            // No compression thus compressed size is 0
            0
        } else {
            // Compress if enabled
            match options.compression {
                CompressionEffort::None => {
                    // Caller has disabled compression, so write uncompressed content
                    writer.write_slice_at(position, &file)?;
                    // No compression thus compressed size is 0
                    0
                }
                CompressionEffort::Best => {
                    todo!()
                    // // Compress with flate2
                    // let mut encoder = ZlibEncoder::new(writer, Compression::best());
                    // encoder.write_all(&file)?;
                    // writer = encoder.finish()?;
                    // // Return compressed size
                    // writer.stream_position()? - raw_offset
                }
                #[cfg(feature = "zopfli")]
                CompressionEffort::Zopfli(provided_options) => {
                    todo!()
                    // // TODO: impl From<ZopfliOptions> for zopfli::Options
                    // let options = zopfli::Options {
                    //     iteration_count: provided_options.iteration_count,
                    //     iterations_without_improvement: provided_options
                    //         .iterations_without_improvement,
                    //     ..Default::default()
                    // };
                    // // Zopfli encoder consumes the writer
                    // let mut encoder =
                    //     zopfli::DeflateEncoder::new(options, zopfli::BlockType::default(), writer);
                    // encoder.write_all(&file)?;
                    // // Writer is returned at finish
                    // writer = encoder.finish()?;
                    // // Return compressed size
                    // writer.stream_position()? - raw_offset
                }
            }
        };

        // Save the reduced metadata
        let metadata = ReducedMetadata {
            size,
            compressed,
            offset: raw_offset - base_offset,
            timestamp,
            path,
        };
        reduced_metadata.push(metadata);
    }

    // Go back to the start of the metadata portion of the header
    *position = STATIC_HEADER_SIZE as u64;

    // Write all the metadata
    for metadata in &reduced_metadata {
        // Convert the path into a `SplitPath`
        let path = SplitPath::try_from(metadata.path).unwrap();
        // Write the file metadata
        writer.write_at(position, &u32be::from(0x1))?; // unk1
        writer.write_at(position, &u32be::from(u32::try_from(metadata.size)?))?;
        writer.write_at(position, &u32be::from(u32::try_from(metadata.compressed)?))?;
        writer.write_at(position, &u64be::from(metadata.timestamp))?;
        writer.write_at(position, &u64be::from(metadata.offset))?;
        writer.write_at(position, &path)?;
        if path.path.starts_with("cache/itf_cooked") {
            writer.write_at(position, &u32be::from(0x2))?;
        } else {
            writer.write_at(position, &u32be::from(0))?;
        }
    }

    if game_platform.platform == utils::Platform::Nx
        && (game_platform.game == Game::JustDance2020
            || game_platform.game == Game::JustDance2021
            || game_platform.game == Game::JustDance2022)
    {
        writer.write_at(position, &u32be::from(0x0))?; // unknown seperator between metadata and data
    }

    test(position, &base_offset)?;

    Ok(())
}

impl BinarySerialize for Platform {
    fn serialize_at(
        &self,
        writer: &mut (impl ZeroCopyWriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(), WriteError> {
        writer.write_at(position, &u32be::from(*self as u32))?;
        Ok(())
    }
}
