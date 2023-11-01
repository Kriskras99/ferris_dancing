//! Contains the parser implementation

use std::{fs, path::Path};

use anyhow::{anyhow, Error};
use byteorder::LittleEndian;
use memmap2::Mmap;

use super::{
    count_zeros, get_addr, is_pow_2, pow_2_roundup, round_size, Block, BlockData, Format, Image,
    TextureHeader, Xtx,
};
use crate::utils::{
    bytes::{read_u32_at, read_u64_at},
    testing::{test, test_any, test_le},
};

const TEX_HEAD_BLK_TYPE: u32 = 0x2;
const DATA_BLK_TYPE: u32 = 0x3;
const UNKNOWN_BLK_TYPE_THREE: u32 = 0x5;

/// Open the file at the given path and parse it as a Nvidia Tegra Texture file
///
/// # Errors
/// In addition to the errors specified by [`parse`]:
/// - Can't open the file
/// - Can't memory map the file
pub fn open<P: AsRef<Path>>(path: P) -> Result<Xtx, Error> {
    let file = fs::File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    parse(&mmap)
}

/// Parse an Nvidia Tegra Texture file
///
/// # Errors
/// - The file is not a Tegra Texture file
/// - The parser encounters an unexpected value
/// - The `src` is not large enough
pub fn parse(src: &[u8]) -> Result<Xtx, Error> {
    let mut position = 0;

    let magic = read_u32_at::<LittleEndian>(src, &mut position)?;
    test(&magic, &0x4E76_4644)?;

    let size = read_u32_at::<LittleEndian>(src, &mut position)?;
    test(&size, &0x10)?;

    let major_version = read_u32_at::<LittleEndian>(src, &mut position)?;
    test(&major_version, &0x1)?;

    let minor_version = read_u32_at::<LittleEndian>(src, &mut position)?;

    let end = src.len();

    let mut blocks = Vec::new();

    while position < end {
        blocks.push(parse_block(src, &mut position)?);
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
                        _ => Err(anyhow!("Found header without data!")),
                    },
                    None => Err(anyhow!("Found header without data!")),
                }?;

                images.push(parse_data_block_to_image(hdr, data)?);

                index += 2;

                Ok(())
            }
            BlockData::Data(_) => Err(anyhow!("Found data without a header!")),
            BlockData::Three(_) => {
                index += 1;
                Ok(())
            }
        }?;
    }

    Ok(Xtx {
        major_version,
        minor_version,
        images,
    })
}

fn parse_block<'a>(src: &'a [u8], position: &mut usize) -> Result<Block<'a>, Error> {
    let start = *position;
    let magic = read_u32_at::<LittleEndian>(src, position)?;
    test(&magic, &0x4E76_4248)?;
    let size = usize::try_from(read_u32_at::<LittleEndian>(src, position)?)?;
    test(&size, &0x24)?;
    let data_size = usize::try_from(read_u64_at::<LittleEndian>(src, position)?)?;
    let data_offset = usize::try_from(read_u64_at::<LittleEndian>(src, position)?)?;
    let typed = read_u32_at::<LittleEndian>(src, position)?;
    let id = read_u32_at::<LittleEndian>(src, position)?;
    let type_idx = read_u32_at::<LittleEndian>(src, position)?;
    test(&type_idx, &0x0)?;

    let pos = *position;
    let block_data = match typed {
        TEX_HEAD_BLK_TYPE => {
            test(&data_size, &0x78)?;
            test(&data_offset, &0x24)?;
            parse_tex_header_block(src, position)
        }
        DATA_BLK_TYPE => {
            *position = pos + data_offset - size;
            let begin = *position;
            let end = *position + data_size;
            *position = end;
            Ok(BlockData::Data(&src[begin..end]))
        }
        UNKNOWN_BLK_TYPE_THREE => {
            *position = pos + data_offset - size;
            let begin = *position;
            let end = *position + data_size;
            *position = end;
            let data = &src[begin..end];
            test(
                &data,
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]
                .as_slice(),
            )?;
            Ok(BlockData::Three(data))
        }
        _ => Err(anyhow!("Unknown block type found!")),
    }?;

    let new_pos = *position;
    test(&(new_pos - pos), &(data_size + data_offset - size))?;

    *position = start + data_offset + data_size;
    Ok(Block {
        id,
        data: block_data,
    })
}

fn parse_tex_header_block<'a>(src: &'a [u8], position: &mut usize) -> Result<BlockData<'a>, Error> {
    let image_size = read_u64_at::<LittleEndian>(src, position)?;
    let alignment = read_u32_at::<LittleEndian>(src, position)?;
    let width = read_u32_at::<LittleEndian>(src, position)?;
    let height = read_u32_at::<LittleEndian>(src, position)?;
    let depth = read_u32_at::<LittleEndian>(src, position)?;
    let target = read_u32_at::<LittleEndian>(src, position)?;
    let format = Format::try_from(read_u32_at::<LittleEndian>(src, position)?)?;
    let mipmaps = read_u32_at::<LittleEndian>(src, position)?;
    test_le(&mipmaps, &17)?;
    let slice_size = read_u32_at::<LittleEndian>(src, position)?;

    let mut mipmap_offsets = [0; 0x10];
    for i in &mut mipmap_offsets {
        *i = read_u32_at::<LittleEndian>(src, position)?;
    }

    let unk1 = read_u64_at::<LittleEndian>(src, position)?;
    test_any(
        &unk1,
        &[
            0x4_0000_0000,
            0x3_0000_0000,
            0x2_0000_0000,
            0x1_0000_0000,
            0x0,
        ],
    )?;

    let unk2 = read_u64_at::<LittleEndian>(src, position)?;
    test(&unk2, &0x7)?;

    Ok(BlockData::TextureHeader(TextureHeader {
        image_size,
        alignment,
        width,
        height,
        depth,
        target,
        format,
        mipmaps,
        slice_size,
        mipmap_offsets,
        unk1,
    }))
}

fn parse_data_block_to_image(hdr: &TextureHeader, data: &[u8]) -> Result<Image, Error> {
    test(&hdr.depth, &1)?;
    let bpp = hdr.format.get_bpp();
    let is_bcn = hdr.format.is_bcn();

    let mut deswizzled_data = Vec::with_capacity(usize::try_from(hdr.mipmaps)?);
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

        let data = deswizzle(
            1.max(hdr.width >> level),
            1.max(hdr.height >> level),
            hdr.format,
            &data[mipmap_offset..mipmap_offset + size],
        )?;

        deswizzled_data.push(data);
    }

    Ok(Image {
        header: *hdr,
        data: deswizzled_data,
    })
}

fn deswizzle(width: u32, height: u32, format: Format, data: &[u8]) -> Result<Vec<u8>, Error> {
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
        _ => Err(anyhow!(
            "BPP is not 1, 2, 4, 8, or 16! {}",
            format.get_bpp()
        )),
    }?;

    let rounded_width = round_size(origin_width, pad);

    let mut result = data.to_vec();

    let x_base = match format.get_bpp() {
        1 => Ok(4),
        2 => Ok(3),
        4 => Ok(2),
        8 => Ok(1),
        16 => Ok(0),
        _ => Err(anyhow!(
            "BPP is not 1, 2, 4, 8, or 16! {}",
            format.get_bpp()
        )),
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
