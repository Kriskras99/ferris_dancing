use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering::Relaxed},
};

use dotstar_toolkit_utils::bytes::{
    primitives::{u32le, u64le},
    write::{BinarySerialize, WriteAt, WriteError},
};
use tegra_swizzle::{
    block_height_mip0, div_round_up,
    surface::{swizzle_surface, BlockDim},
    BlockHeight,
};

use super::{Image, Xtx};

impl BinarySerialize for Xtx {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        xtx: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, 0x4E76_4644)?;
        writer.write_at::<u32le>(position, 0x10)?;
        writer.write_at::<u32le>(position, xtx.major_version)?;
        writer.write_at::<u32le>(position, xtx.minor_version)?;

        // Hacky work around as associated types cannot have a free lifetime
        let id = Rc::new(AtomicU32::new(0));
        for image in xtx.images {
            writer.write_at_with_ctx::<Image>(position, image, id.clone())?;
        }
        // Write unknown third header
        writer.write_at::<u32le>(position, 0x4E76_4248)?;
        writer.write_at::<u32le>(position, 0x24)?;
        writer.write_at::<u64le>(position, 24)?;
        writer.write_at::<u64le>(position, 0x24)?;
        writer.write_at::<u32le>(position, 0x5)?;
        writer.write_at::<u32le>(position, id.load(Relaxed))?;
        writer.write_at::<u32le>(position, 0x0)?;
        writer.write_slice_at(
            position,
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        )?;
        Ok(())
    }
}

impl BinarySerialize for Image {
    // Hacky work around as associated types cannot have a free lifetime
    type Ctx = Rc<AtomicU32>;
    type Input = Self;

    fn serialize_at_with_ctx(
        image: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        id: Self::Ctx,
    ) -> Result<(), WriteError> {
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

        // Write texture header
        writer.write_at::<u32le>(position, 0x4E76_4248)?;
        writer.write_at::<u32le>(position, 0x24)?;
        writer.write_at::<u64le>(position, 0x78)?;
        writer.write_at::<u64le>(position, 0x24)?;
        writer.write_at::<u32le>(position, 0x2)?;
        writer.write_at::<u32le>(position, id.load(Relaxed))?;
        writer.write_at::<u32le>(position, 0x0)?;
        writer.write_at::<u64le>(position, u64::try_from(swizzled.len())?)?;
        writer.write_at::<u32le>(position, image.header.alignment)?;
        writer.write_at::<u32le>(position, image.header.width)?;
        writer.write_at::<u32le>(position, image.header.height)?;
        writer.write_at::<u32le>(position, image.header.depth)?;
        writer.write_at::<u32le>(position, image.header.target)?;
        writer.write_at::<u32le>(position, image.header.format.into())?;
        writer.write_at::<u32le>(position, image.header.mipmaps)?;
        writer.write_at::<u32le>(position, u32::try_from(swizzled.len())?)?;
        for mipmap in image.header.mipmap_offsets {
            writer.write_at::<u32le>(position, mipmap)?;
        }

        writer.write_at::<u32le>(position, block_height_log2)?;
        writer.write_at::<u32le>(position, 0x7)?;
        writer.write_at::<u32le>(position, 0x0)?;

        id.fetch_add(1, Relaxed);

        // Write texture data
        writer.write_at::<u32le>(position, 0x4E76_4248)?;
        writer.write_at::<u32le>(position, 0x24)?;
        writer.write_at::<u64le>(position, u64::try_from(swizzled.len())?)?;
        writer.write_at::<u64le>(position, 0x154)?;
        writer.write_at::<u32le>(position, 0x3)?;
        writer.write_at::<u32le>(position, id.load(Relaxed))?;
        writer.write_at::<u32le>(position, 0x0)?;
        for _ in 0..0x130 {
            writer.write_at::<u8>(position, 0)?;
        }
        writer.write_slice_at(position, &swizzled)?;
        id.fetch_add(1, Relaxed);
        Ok(())
    }
}
