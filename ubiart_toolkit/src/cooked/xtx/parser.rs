//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32le, u64le},
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::{test_eq, test_le},
};
use tegra_swizzle::{
    surface::{deswizzle_surface, BlockDim},
    BlockHeight,
};

use super::{Block, BlockData, Format, Image, TextureHeader, Xtx};
use crate::cooked::xtx::Index;

const TEX_HEAD_BLK_TYPE: u32 = 0x2;
const DATA_BLK_TYPE: u32 = 0x3;
const UNKNOWN_BLK_TYPE_THREE: u32 = 0x5;

impl BinaryDeserialize<'_> for Xtx {
    #[tracing::instrument(skip(reader))]
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, dotstar_toolkit_utils::bytes::read::ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32le>(position)?.into();
        test_eq(&magic, &0x4E76_4644u32)?;

        let size = reader.read_at::<u32le>(position)?.into();
        test_eq(&size, &0x10u32)?;

        let major_version = reader.read_at::<u32le>(position)?.into();
        test_eq(&major_version, &0x1)?;

        let minor_version = reader.read_at::<u32le>(position)?.into();

        let mut blocks = Vec::new();

        loop {
            match reader.read_at::<u32le>(position) {
                Ok(magic) => {
                    *position -= 4;
                    if u32::from(magic) != 0x4E76_4248 {
                        break;
                    }
                }
                Err(ReadError::IoError {
                    error: _,
                    backtrace: _,
                }) => break,
                Err(error) => return Err(error),
            }
            tracing::trace!("Block start: {}", *position - start);
            let block = reader.read_at::<Block>(position)?;
            blocks.push(block);
        }

        let mut images = Vec::new();

        let mut index = 0;
        while index < blocks.len() {
            let block = blocks.get(index).unwrap_or_else(|| unreachable!());
            match &block.data {
                BlockData::TextureHeader(hdr) => {
                    let second_block = blocks.get(index + 1);
                    let data = match second_block {
                        Some(block) => match &block.data {
                            BlockData::Data(data) => Ok(data),
                            _ => Err(ReadError::custom("Found header without data".to_string())),
                        },
                        None => Err(ReadError::custom("Found header without data".to_string())),
                    }?;

                    images.push(parse_data_block_to_image(hdr, data)?);

                    index += 2;

                    Ok(())
                }
                BlockData::Data(_) => {
                    Err(ReadError::custom("Found data without a header".to_string()))
                }
                BlockData::Three(_) => {
                    index += 1;
                    Ok(())
                }
            }?;
        }

        Ok(Self {
            major_version,
            minor_version,
            images,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Block<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32le>(position)?.into();
        test_eq(&magic, &0x4E76_4248u32)?;
        let header_size = reader.read_at::<u32le>(position)?.into();
        test_eq(&header_size, &0x24)?;
        let data_size = usize::try_from(reader.read_at::<u64le>(position)?)?;
        let data_offset = reader.read_at::<u64le>(position)?.into();
        let block_type = reader.read_at::<u32le>(position)?.into();
        let id = reader.read_at::<u32le>(position)?.into();
        let type_idx = reader.read_at::<u32le>(position)?.into();
        test_eq(&type_idx, &0x0u32)?;

        let pos = *position;
        let block_data = match block_type {
            TEX_HEAD_BLK_TYPE => {
                test_eq(&data_size, &0x78)?;
                test_eq(&data_offset, &0x24)?;
                parse_tex_header_block(reader, position)
            }
            DATA_BLK_TYPE => {
                *position = pos + data_offset - header_size;
                Ok(BlockData::Data(reader.read_slice_at(position, data_size)?))
            }
            UNKNOWN_BLK_TYPE_THREE => {
                *position = pos + data_offset - header_size;
                let data = reader.read_slice_at(position, data_size)?;
                test_eq(
                    &data.as_ref(),
                    &[
                        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    ]
                    .as_slice(),
                )?;
                Ok(BlockData::Three(data))
            }
            _ => Err(ReadError::custom(format!(
                "Unknown block type found: {block_type:x}"
            ))),
        }?;

        let data_size = u64::try_from(data_size)?;

        let new_pos = *position;
        test_eq(&(new_pos - pos), &(data_size + data_offset - header_size))?;

        *position = start + data_offset + data_size;
        Ok(Block {
            id,
            data: block_data,
        })
    }
}

impl BinaryDeserialize<'_> for Format {
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value = u32::from(reader.read_at::<u32le>(position)?);
        Self::try_from(value).map_err(|_| ReadError::custom(format!("Unknown format: {value:x}")))
    }
}

/// Parse some data at `position` as a [`BlockData::TextureHeader`]
fn parse_tex_header_block<'de>(
    reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
    position: &mut u64,
) -> Result<BlockData<'de>, ReadError> {
    let image_size = reader.read_at::<u64le>(position)?.into();
    let alignment = reader.read_at::<u32le>(position)?.into();
    test_eq(&alignment, &0x200)?;
    let width = reader.read_at::<u32le>(position)?.into();
    let height = reader.read_at::<u32le>(position)?.into();
    let depth = reader.read_at::<u32le>(position)?.into();
    test_eq(&depth, &1)?;
    let target = reader.read_at::<u32le>(position)?.into();
    test_eq(&target, &1)?;
    let format = reader.read_at::<Format>(position)?;
    let mip_count = reader.read_at::<u32le>(position)?.into();
    test_le(&mip_count, &17)?;
    let slice_size = reader.read_at::<u32le>(position)?.into();

    test_eq(&image_size, &u64::from(slice_size))?;

    let mut mipmap_offsets = [0; 17];
    for i in &mut mipmap_offsets {
        *i = reader.read_at::<u32le>(position)?.into();
    }

    let texture_layout_1: u32 = reader.read_at::<u32le>(position)?.into();
    test_eq(&(texture_layout_1 & !0b111), &0)?;
    let texture_layout_2 = reader.read_at::<u32le>(position)?.into();
    test_eq(&texture_layout_2, &7u32)?;
    let boolean = reader.read_at::<u32le>(position)?.into();
    test_eq(&boolean, &0u32)?;

    let block_height_log2 =
        u8::try_from(texture_layout_1 & 0b111).unwrap_or_else(|_| unreachable!());

    Ok(BlockData::TextureHeader(TextureHeader {
        image_size,
        alignment,
        width,
        height,
        depth,
        target,
        format,
        mipmaps: mip_count,
        slice_size,
        mipmap_offsets,
        block_height_log2,
    }))
}

/// Retrieve the data the [`TextureHeader`] points at and create a [`Image`]
#[tracing::instrument(skip(hdr, data))]
fn parse_data_block_to_image(hdr: &TextureHeader, data: &[u8]) -> Result<Image, ReadError> {
    let bpp = usize::try_from(hdr.format.get_bpp())?;
    let is_bcn = hdr.format.is_bcn();
    let width = usize::try_from(hdr.width)?;
    let height = usize::try_from(hdr.height)?;
    let depth = usize::try_from(hdr.depth)?;
    let mipmap_count = usize::try_from(hdr.mipmaps)?;

    let block_dim = if is_bcn {
        BlockDim::block_4x4()
    } else {
        BlockDim::uncompressed()
    };

    let block_height_log2 = u32::from(hdr.block_height_log2);
    let block_height = BlockHeight::new(2usize.pow(block_height_log2))
        .ok_or_else(|| ReadError::custom(format!("Invalid block height: 2^{block_height_log2}")))?;

    tracing::trace!("format: {:?}, width: {width}, height: {height}, depth: {depth}, data: {}, block_dim: {block_dim:?}, block_height: {block_height:?}, bpp: {bpp}, mipmap_count: {mipmap_count}", hdr.format, data.len());

    let deswizzled = deswizzle_surface(
        width,
        height,
        depth,
        data,
        block_dim,
        Some(block_height),
        bpp,
        mipmap_count,
        1,
    )
    .map_err(|e| ReadError::custom(format!("{e:?}")))?;

    tracing::trace!("deswizzled: {}", deswizzled.len());

    let mut indexes = Vec::with_capacity(mipmap_count);
    tracing::trace!("Mipmap offsets: {:?}", hdr.mipmap_offsets);
    for level in 0..mipmap_count {
        let width = 1.max(width >> level);
        let height = 1.max(height >> level);
        let size = if is_bcn {
            // BCn formats have 4x4 pixels per texel
            (round_up(width, 4) / 4) * (round_up(height, 4) / 4) * bpp
        } else {
            width * height * bpp
        };

        let mipmap_offset = usize::try_from(hdr.mipmap_offsets[level])?;

        indexes.push(Index {
            width,
            height,
            offset: mipmap_offset,
            size,
        });

        tracing::trace!("Level: {level}, width: {width}, height: {height}, offset: {mipmap_offset}, size: {size}");
    }

    let alignment = usize::try_from(hdr.alignment)?;
    let image_size = usize::try_from(hdr.image_size)?;
    let actual_size = indexes
        .iter()
        .map(|i| i.size)
        .map(|i| round_up(i, alignment))
        .reduce(|acc, e| acc + e)
        .ok_or_else(|| ReadError::custom("No image in data??".into()))?;

    if image_size != actual_size {
        println!("Image size does not match: 0x{image_size:x} 0x{actual_size:x}");
    }

    Ok(Image {
        header: *hdr,
        data: deswizzled,
        indexes,
    })
}

#[tracing::instrument]
fn round_up(n: usize, m: usize) -> usize {
    assert_ne!(m, 0, "Can't round up to zero!");
    (n + m - 1) & !(m - 1)
}
