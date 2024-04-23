//! Contains the parser implementation

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32le, u64le},
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::{test_eq, test_le},
};

use super::{
    count_zeros, get_addr, is_pow_2, pow_2_roundup, round_size, Block, BlockData, Format, Image,
    TextureHeader, Xtx,
};

const TEX_HEAD_BLK_TYPE: u32 = 0x2;
const DATA_BLK_TYPE: u32 = 0x3;
const UNKNOWN_BLK_TYPE_THREE: u32 = 0x5;

impl BinaryDeserialize<'_> for Xtx {
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, dotstar_toolkit_utils::bytes::read::ReadError> {
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
    let width = reader.read_at::<u32le>(position)?.into();
    let height = reader.read_at::<u32le>(position)?.into();
    let depth = reader.read_at::<u32le>(position)?.into();
    let target = reader.read_at::<u32le>(position)?.into();
    let format = reader.read_at::<Format>(position)?;
    let mip_count = reader.read_at::<u32le>(position)?.into();
    test_le(&mip_count, &17)?;
    let slice_size = reader.read_at::<u32le>(position)?.into();

    let mut mipmap_offsets = [0; 17];
    for i in &mut mipmap_offsets {
        *i = reader.read_at::<u32le>(position)?.into();
    }

    let texture_layout_1 = reader.read_at::<u32le>(position)?.into();
    let texture_layout_2 = reader.read_at::<u32le>(position)?.into();
    let boolean = reader.read_at::<u32le>(position)?.into();

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
        texture_layout_1,
        texture_layout_2,
        boolean,
    }))
}

/// Retrieve the data the [`TextureHeader`] points at and create a [`Image`]
#[tracing::instrument(skip(hdr, data))]
fn parse_data_block_to_image(hdr: &TextureHeader, data: &[u8]) -> Result<Image, ReadError> {
    test_eq(&hdr.depth, &1)?;
    let bpp = hdr.format.get_bpp();
    let is_bcn = hdr.format.is_bcn();

    let mut deswizzled_data = Vec::with_capacity(usize::try_from(hdr.mipmaps)?);
    tracing::trace!("Data size: {}", data.len());
    tracing::trace!("Data mipmaps: {}", hdr.mipmaps);
    tracing::trace!("Data offsets: {:?}", hdr.mipmap_offsets);
    for level in 0..hdr.mipmaps {
        let size = if is_bcn {
            usize::try_from(
                ((1.max(hdr.width >> level) + 3) >> 2)
                    * ((1.max(hdr.height >> level) + 3) >> 2)
                    * bpp,
            )?
        } else {
            usize::try_from(1.max(hdr.width >> level) * 1.max(hdr.height >> level) * bpp)?
        };

        let mipmap_offset = usize::try_from(hdr.mipmap_offsets[usize::try_from(level)?])?;

        let width = 1.max(hdr.width >> level);
        let height = 1.max(hdr.height >> level);
        tracing::trace!("Level: {level}, width: {width}, height: {height}, format: {:?}, offset: {mipmap_offset}, size: {size}", hdr.format);

        let data = deswizzle(
            1.max(hdr.width >> level),
            1.max(hdr.height >> level),
            hdr.format,
            &data[mipmap_offset..mipmap_offset + size],
        )?;

        deswizzled_data.push(data);
    }

    // TODO: After level 4 this goes wrong and the image size does not add up
    // test(
    //     &usize::try_from(hdr.image_size).ok(),
    //     &deswizzled_data
    //         .iter()
    //         .map(Vec::len)
    //         .reduce(|acc, e| acc + e),
    // )?;

    Ok(Image {
        header: *hdr,
        data: deswizzled_data,
    })
}

/// Deswizzle the image in `data`
fn deswizzle(width: u32, height: u32, format: Format, data: &[u8]) -> Result<Vec<u8>, ReadError> {
    let (origin_width, origin_height) = if format.is_bcn() {
        ((width + 3) / 4, (height + 3) / 4)
    } else {
        (width, height)
    };

    let xb = count_zeros(pow_2_roundup(origin_width));
    let mut yb = count_zeros(pow_2_roundup(origin_height));

    let hh = pow_2_roundup(origin_height) >> 1;

    if !is_pow_2(origin_height) && origin_height <= hh + (hh / 3) && yb > 3 {
        yb -= 1;
    }

    let pad = match format.get_bpp() {
        1 => Ok(64),
        2 => Ok(32),
        4 => Ok(16),
        8 => Ok(8),
        16 => Ok(4),
        _ => Err(ReadError::custom(format!(
            "BPP is not 1, 2, 4, 8, or 16: {}",
            format.get_bpp()
        ))),
    }?;

    let rounded_width = round_size(origin_width, pad);

    let mut result = data.to_vec();

    let x_base = match format.get_bpp() {
        1 => Ok(4),
        2 => Ok(3),
        4 => Ok(2),
        8 => Ok(1),
        16 => Ok(0),
        _ => Err(ReadError::custom(format!(
            "BPP is not 1, 2, 4, 8, or 16: {}",
            format.get_bpp()
        ))),
    }?;

    let mut pos_ = 0;
    let bpp = usize::try_from(format.get_bpp())?;

    for y in 0..origin_height {
        for x in 0..origin_width {
            let pos = get_addr(x, y, xb, yb, rounded_width, x_base)? * bpp;

            if pos + bpp < data.len() && pos_ + bpp < data.len() {
                result[pos_..pos_ + bpp].copy_from_slice(&data[pos..pos + bpp]);
            }

            pos_ += bpp;
        }
    }

    Ok(result)
}
