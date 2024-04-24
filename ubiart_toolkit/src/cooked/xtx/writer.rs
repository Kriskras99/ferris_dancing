use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};
use dotstar_toolkit_utils::bytes::write::WriteError;
use tegra_swizzle::{
    block_height_mip0, div_round_up,
    surface::{swizzle_surface, BlockDim},
    BlockHeight,
};

use super::Xtx;

/// Writes the XTX texture to the file
#[tracing::instrument(skip(src, xtx))]
pub fn create<W: Write>(mut src: W, xtx: &Xtx) -> Result<(), WriteError> {
    src.write_u32::<LittleEndian>(0x4E76_4644)?;
    src.write_u32::<LittleEndian>(0x10)?;
    src.write_u32::<LittleEndian>(xtx.major_version)?;
    src.write_u32::<LittleEndian>(xtx.minor_version)?;
    let mut id = 0;
    for image in &xtx.images {
        let bpp = usize::try_from(image.header.format.get_bpp())?;
        let is_bcn = image.header.format.is_bcn();
        let width = usize::try_from(image.header.width)?;
        let height = usize::try_from(image.header.height)?;
        let depth = usize::try_from(image.header.depth)?;
        let mipmap_count = usize::try_from(image.header.mipmaps)?;
        let block_dim = if is_bcn {
            BlockDim::block_4x4()
        } else {
            BlockDim::uncompressed()
        };

        let block_height = if image.header.format.is_bcn() {
            block_height_mip0(div_round_up(height, 4))
        } else {
            block_height_mip0(height)
        };

        let block_height_log2: u32 = match block_height {
            BlockHeight::One => 0,
            BlockHeight::Two => 1,
            BlockHeight::Four => 2,
            BlockHeight::Eight => 3,
            BlockHeight::Sixteen => 4,
            BlockHeight::ThirtyTwo => 5,
        };
        let swizzled = swizzle_surface(
            width,
            height,
            depth,
            &image.data,
            block_dim,
            Some(block_height),
            bpp,
            mipmap_count,
            1,
        )
        .map_err(|e| WriteError::custom(format!("{e:?}")))?;
        tracing::trace!("width: {width}, height: {height}, depth: {depth}, data: {}, block_dim: {block_dim:?}, block_height: {block_height:?}, bpp: {bpp}, mipmap_count: {mipmap_count}, swizzled: {}", image.data.len(), swizzled.len());

        // Write texture header
        src.write_u32::<LittleEndian>(0x4E76_4248)?;
        src.write_u32::<LittleEndian>(0x24)?;
        src.write_u64::<LittleEndian>(0x78)?;
        src.write_u64::<LittleEndian>(0x24)?;
        src.write_u32::<LittleEndian>(0x2)?;
        src.write_u32::<LittleEndian>(id)?;
        src.write_u32::<LittleEndian>(0x0)?;
        src.write_u64::<LittleEndian>(u64::try_from(swizzled.len())?)?;
        src.write_u32::<LittleEndian>(image.header.alignment)?;
        src.write_u32::<LittleEndian>(image.header.width)?;
        src.write_u32::<LittleEndian>(image.header.height)?;
        src.write_u32::<LittleEndian>(image.header.depth)?;
        src.write_u32::<LittleEndian>(image.header.target)?;
        src.write_u32::<LittleEndian>(image.header.format.into())?;
        src.write_u32::<LittleEndian>(image.header.mipmaps)?;
        src.write_u32::<LittleEndian>(u32::try_from(swizzled.len())?)?;
        for mipmap in image.header.mipmap_offsets {
            src.write_u32::<LittleEndian>(mipmap)?;
        }

        src.write_u32::<LittleEndian>(block_height_log2)?;
        src.write_u32::<LittleEndian>(0x7)?;
        src.write_u32::<LittleEndian>(0x0)?;

        id += 1;

        // Write texture data
        src.write_u32::<LittleEndian>(0x4E76_4248)?;
        src.write_u32::<LittleEndian>(0x24)?;
        src.write_u64::<LittleEndian>(u64::try_from(swizzled.len())?)?;
        src.write_u64::<LittleEndian>(0x154)?;
        src.write_u32::<LittleEndian>(0x3)?;
        src.write_u32::<LittleEndian>(id)?;
        src.write_u32::<LittleEndian>(0x0)?;
        for _ in 0..0x130 {
            src.write_u8(0x0)?;
        }
        src.write_all(&swizzled)?;
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
