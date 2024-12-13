use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32be, u64be},
        write::{WriteAt, WriteError},
        CursorAt,
    },
    vfs::{VirtualFileSystem, VirtualPath},
};
use flate2::{write::ZlibEncoder, Compression};
use test_eq::test_eq;
use tracing::instrument;

use super::MAGIC;
use crate::utils::{Game, Platform, SplitPath, UniqueGameId};

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub compression: CompressionEffort,
    pub game_platform: UniqueGameId,
    pub unk4: u32,
    pub engine_version: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            compression: CompressionEffort::None,
            game_platform: UniqueGameId::try_from(0x1DDB_2268).unwrap_or_else(|_| unreachable!()),
            unk4: 0x0009_37D0,
            engine_version: 0x0004_FD39,
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

#[cfg(feature = "zopfli")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZopfliOptions {
    pub iteration_count: std::num::NonZeroU64,
    pub iterations_without_improvement: std::num::NonZeroU64,
}

#[cfg(feature = "zopfli")]
impl Default for ZopfliOptions {
    fn default() -> Self {
        Self {
            iteration_count: std::num::NonZeroU64::new(15).unwrap(),
            iterations_without_improvement: std::num::NonZeroU64::MIN,
        }
    }
}

#[cfg(feature = "zopfli")]
impl From<ZopfliOptions> for zopfli::Options {
    fn from(value: ZopfliOptions) -> Self {
        Self {
            iteration_count: value.iteration_count,
            iterations_without_improvement: value.iterations_without_improvement,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReducedMetadata<'a> {
    pub size: u64,
    pub compressed: u64,
    pub offset: u64,
    pub timestamp: u64,
    pub path: &'a VirtualPath,
}

const STATIC_HEADER_SIZE: usize = 0x30;

/// Create a secure_fat.gf file at the path
pub fn create(
    path: impl AsRef<Path>,
    options: Options,
    vfs: &impl VirtualFileSystem,
    files: &[&VirtualPath],
) -> Result<(), WriteError> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    write(&mut writer, &mut 0, options, vfs, files)
}

/// Create an .ipk file with the specified files.
// TODO: Add deduplication
#[instrument(skip(writer, vfs, files))]
pub fn write(
    mut writer: &mut (impl WriteAt + ?Sized),
    position: &mut u64,
    options: Options,
    vfs: &impl VirtualFileSystem,
    files: &[&VirtualPath],
) -> Result<(), WriteError> {
    // TODO: Make this code position independent
    assert_eq!(
        *position, 0,
        "TODO: This code is not yet position independent!"
    );
    // let static_header_size = *position + u64::try_from(STATIC_HEADER_SIZE)?;
    // Calculate the size of the header, starting with the static size
    let mut base_offset = STATIC_HEADER_SIZE;

    // On NX, JD2020-JD2022 have 4 null bytes between the header and the content of the files
    if options.game_platform.platform == Platform::Nx
        && (options.game_platform.game == Game::JustDance2020
            || options.game_platform.game == Game::JustDance2021
            || options.game_platform.game == Game::JustDance2022)
    {
        base_offset += 0x4;
    }

    // Add the static metadata size for every file plus the length of the path
    for path in files {
        base_offset += 0x2C + path.as_str().len(); // metadata size + path length
    }

    // Start writing the header
    writer.write_at::<u32be>(position, MAGIC)?;
    writer.write_at::<u32be>(position, 0x5)?; // version
    writer.write_at::<Platform>(position, options.game_platform.platform)?;
    writer.write_at::<u32be>(position, u32::try_from(base_offset)?)?;
    writer.write_at::<u32be>(position, u32::try_from(files.len())?)?;
    writer.write_at::<u32be>(position, 0x0)?; // unk1
    writer.write_at::<u32be>(position, 0x0)?; // unk2
    writer.write_at::<u32be>(position, 0x0)?; // unk3
    writer.write_at::<u32be>(position, options.unk4)?;
    writer.write_at::<UniqueGameId>(position, options.game_platform)?;
    writer.write_at::<u32be>(position, options.engine_version)?;
    writer.write_at::<u32be>(position, u32::try_from(files.len())?)?;

    // Skip the file metadata for now, as it depends on compression results
    let base_offset = u64::try_from(base_offset)?;
    *position = base_offset;

    // For keeping track of the relevant metadata that needs to be written to the header
    let mut reduced_metadata = Vec::with_capacity(files.len());

    // Write the content of all files, while filling `reduced_metadata`
    for path in files {
        // The offset from the start of the file
        // NB: the metadata stores the offset relevant to the end of the header
        let raw_offset = *position;
        // This is presumably a timestamp, but the values don't add up. So we use a static value.
        let timestamp = 132_761_939_258_059_932;

        // Open the file and get the size
        let file = vfs.open(path.as_ref())?;
        let size = u64::try_from(file.len())?;

        // File content can be stored compressed.
        // Skip compression for small files, and already compressed files.
        let compressed = if size < 2048
            || path.extension() == Some("jpg")
            || path.extension() == Some("webm")
            || path.extension() == Some("ogg")
            || path.extension() == Some("png")
        {
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
                    // Compress with flate2
                    let cursor = CursorAt::new(writer, *position);
                    let mut encoder = ZlibEncoder::new(cursor, Compression::best());
                    encoder.write_all(&file)?;
                    (writer, *position) = encoder.finish()?.into_inner();
                    // Return compressed size
                    *position - raw_offset
                }
                #[cfg(feature = "zopfli")]
                CompressionEffort::Zopfli(provided_options) => {
                    let mut cursor = CursorAt::new(writer, *position);
                    // Zopfli encoder consumes the writer
                    let mut encoder = zopfli::DeflateEncoder::new(
                        provided_options.into(),
                        zopfli::BlockType::default(),
                        &mut cursor,
                    );
                    encoder.write_all(&file)?;
                    // Writer is returned at finish
                    encoder.finish()?;
                    *position = cursor.into_inner().1;
                    // Return compressed size
                    *position - raw_offset
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
    *position = u64::try_from(STATIC_HEADER_SIZE).unwrap_or_else(|_| unreachable!());

    // Write all the metadata
    for metadata in &reduced_metadata {
        // Convert the path into a `SplitPath`
        let path =
            SplitPath::try_from(metadata.path).map_err(|e| WriteError::custom(format!("{e:?}")))?;
        // Write the file metadata
        writer.write_at::<u32be>(position, 0x1)?; // unk1
        writer.write_at::<u32be>(position, u32::try_from(metadata.size)?)?;
        writer.write_at::<u32be>(position, u32::try_from(metadata.compressed)?)?;
        writer.write_at::<u64be>(position, metadata.timestamp)?;
        writer.write_at::<u64be>(position, metadata.offset)?;
        writer.write_at::<SplitPath>(position, path)?;
        // The SplitPath padding byte is reused as a cooked indicator
        *position -= 4;
        if metadata.path.starts_with("cache/itf_cooked") {
            writer.write_at::<u32be>(position, 0x2)?;
        } else {
            writer.write_at::<u32be>(position, 0)?;
        }
    }

    if options.game_platform.platform == Platform::Nx
        && (options.game_platform.game == Game::JustDance2020
            || options.game_platform.game == Game::JustDance2021
            || options.game_platform.game == Game::JustDance2022)
    {
        writer.write_at::<u32be>(position, 0x0)?; // unknown seperator between metadata and data
    }

    test_eq!(*position, base_offset)?;

    Ok(())
}
