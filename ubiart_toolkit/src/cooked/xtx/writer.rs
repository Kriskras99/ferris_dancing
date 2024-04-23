use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};
use dotstar_toolkit_utils::bytes::write::WriteError;

use super::{count_zeros, get_addr, is_pow_2, pow_2_roundup, round_size, Format, Xtx};

/// Writes the XTX texture to the file
#[tracing::instrument(skip(src, xtx))]
pub fn create<W: Write>(mut src: W, xtx: &Xtx) -> Result<(), WriteError> {
    src.write_u32::<LittleEndian>(0x4E76_4644)?;
    src.write_u32::<LittleEndian>(0x10)?;
    src.write_u32::<LittleEndian>(xtx.major_version)?;
    src.write_u32::<LittleEndian>(xtx.minor_version)?;
    let mut id = 0;
    for image in &xtx.images {
        // Write texture header
        src.write_u32::<LittleEndian>(0x4E76_4248)?;
        src.write_u32::<LittleEndian>(0x24)?;
        src.write_u64::<LittleEndian>(0x78)?;
        src.write_u64::<LittleEndian>(0x24)?;
        src.write_u32::<LittleEndian>(0x2)?;
        src.write_u32::<LittleEndian>(id)?;
        src.write_u32::<LittleEndian>(0x0)?;
        src.write_u64::<LittleEndian>(image.header.image_size)?;
        src.write_u32::<LittleEndian>(image.header.alignment)?;
        src.write_u32::<LittleEndian>(image.header.width)?;
        src.write_u32::<LittleEndian>(image.header.height)?;
        src.write_u32::<LittleEndian>(image.header.depth)?;
        src.write_u32::<LittleEndian>(image.header.target)?;
        src.write_u32::<LittleEndian>(image.header.format.into())?;
        src.write_u32::<LittleEndian>(image.header.mipmaps)?;
        src.write_u32::<LittleEndian>(image.header.slice_size)?;
        for mipmap in image.header.mipmap_offsets {
            src.write_u32::<LittleEndian>(mipmap)?;
        }
        src.write_u64::<LittleEndian>(image.header.unk1)?;
        src.write_u64::<LittleEndian>(0x7)?;

        id += 1;

        // Write texture data
        src.write_u32::<LittleEndian>(0x4E76_4248)?;
        src.write_u32::<LittleEndian>(0x24)?;
        let mut data_size = 0;
        for data in &image.data {
            data_size += u64::try_from(data.len())?;
        }
        src.write_u64::<LittleEndian>(data_size)?;
        src.write_u64::<LittleEndian>(0x154)?;
        src.write_u32::<LittleEndian>(0x3)?;
        src.write_u32::<LittleEndian>(id)?;
        src.write_u32::<LittleEndian>(0x0)?;
        for _ in 0..0x130 {
            src.write_u8(0x0)?;
        }

        for level in 0..image.header.mipmaps {
            let data = &image.data[usize::try_from(level)?];
            let swizzled = swizzle(
                1.max(image.header.width >> level),
                1.max(image.header.height >> level),
                image.header.format,
                data,
            )?;
            src.write_all(&swizzled)?;
            tracing::trace!(
                "Data size: {}, swizzled size: {}",
                data.len(),
                swizzled.len()
            );
        }
        id += 1;
    }
    // Write unknown third header
    src.write_u32::<LittleEndian>(0x4E76_4248)?;
    src.write_u32::<LittleEndian>(0x24)?;
    src.write_u64::<LittleEndian>(24)?;
    src.write_u64::<LittleEndian>(0x24)?;
    src.write_u32::<LittleEndian>(0x5)?;
    src.write_u32::<LittleEndian>(id)?;
    src.write_u32::<LittleEndian>(0x0)?;
    src.write_all(&[
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ])?;
    Ok(())
}

/// Swizzle the image in `data`
fn swizzle(width: u32, height: u32, format: Format, data: &[u8]) -> Result<Vec<u8>, WriteError> {
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
        _ => Err(WriteError::custom(format!(
            "BPP is not 1, 2, 4, 8, or 16! {}",
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
        _ => Err(WriteError::custom(format!(
            "BPP is not 1, 2, 4, 8, or 16! {}",
            format.get_bpp()
        ))),
    }?;

    let mut pos_ = 0;
    let bpp = usize::try_from(format.get_bpp())?;

    for y in 0..origin_height {
        for x in 0..origin_width {
            let pos = get_addr(x, y, xb, yb, rounded_width, x_base)? * bpp;

            if pos + bpp < data.len() && pos_ + bpp < data.len() {
                result[pos..pos + bpp].copy_from_slice(&data[pos_..pos_ + bpp]);
            }

            pos_ += bpp;
        }
    }

    Ok(result)
}
