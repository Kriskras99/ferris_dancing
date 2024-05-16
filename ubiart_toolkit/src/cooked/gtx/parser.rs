use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    testing::{test_eq, test_ge, test_le},
};
use wiiu_swizzle::{deswizzle_surface, AddrTileMode};

use super::{
    types::{GfdHeader, Gtx},
    Block, Format, Gx2Surface,
};
use crate::cooked::gtx::Image;

// TODO: improve with info from https://mk8.tockdom.com/wiki/GTX%5CGSH_(File_Format)

const COMP_SEL: &[char] = &['R', 'G', 'B', 'A', '0', '1'];

impl BinaryDeserialize<'_> for Gtx {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let gfd = reader.read_at::<GfdHeader>(position)?;

        let mut blocks = Vec::new();

        loop {
            match reader.read_at::<u32be>(position) {
                Ok(magic) => {
                    *position -= 4;
                    if magic != Block::MAGIC {
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
            match block {
                Block::Surface(hdr) => {
                    let data = loop {
                        index += 1;
                        match blocks.get(index) {
                            Some(Block::Data(data)) => break data,
                            Some(_) => continue,
                            None => {
                                return Err(ReadError::custom(
                                    "Found header without data".to_string(),
                                ))
                            }
                        }
                    };

                    images.push(parse_data_block_to_image(hdr, data));

                    index += 2;

                    Ok(())
                }
                Block::Data(_) => Err(ReadError::custom("Found data without a header".to_string())),
                Block::Mip(_) => {
                    println!("Ignoring MIP!");
                    index += 1;
                    Ok(())
                }
                Block::Unknown(id, data) => {
                    println!("Unknown block!: id: {id}, data: &[u8; {}]", data.len());
                    index += 1;
                    Ok(())
                }
            }?;
        }

        Ok(Self { gfd, images })
    }
}

impl BinaryDeserialize<'_> for GfdHeader {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32be>(position)?;
        test_eq(&magic, &Self::MAGIC)?;
        let size = reader.read_at::<u32be>(position)?;
        test_eq(&size, &0x20)?;
        let major_version = reader.read_at::<u32be>(position)?;
        test_eq(&major_version, &7)?;
        let minor_version = reader.read_at::<u32be>(position)?;
        test_eq(&minor_version, &1)?;
        let gpu_version = reader.read_at::<u32be>(position)?;
        test_eq(&gpu_version, &Self::GPU_VERSION)?;
        let align_mode = reader.read_at::<u32be>(position)?;
        let reserved1 = reader.read_at::<u32be>(position)?;
        test_eq(&reserved1, &0)?;
        let reserved2 = reader.read_at::<u32be>(position)?;
        test_eq(&reserved2, &0)?;

        Ok(Self { align_mode })
    }
}

impl<'de> BinaryDeserialize<'de> for Block<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32be>(position)?;
        test_eq(&magic, &Self::MAGIC)?;
        let size = reader.read_at::<u32be>(position)?;
        let major_version = reader.read_at::<u32be>(position)?;
        test_eq(&major_version, &1)?;
        let minor_version = reader.read_at::<u32be>(position)?;
        test_eq(&minor_version, &0)?;
        let type_it = reader.read_at::<u32be>(position)?;
        let data_size = reader.read_at::<u32be>(position)?;
        let id = reader.read_at::<u32be>(position)?;
        test_eq(&id, &0)?;
        let type_idx = reader.read_at::<u32be>(position)?;
        test_eq(&type_idx, &0)?;
        test_eq(&(*position - start), &u64::from(size))?;

        let block = match type_it {
            // Surf
            0x0Bu32 => Block::Surface(reader.read_at::<Gx2Surface>(position)?),
            // Data
            0x0C => Block::Data(reader.read_slice_at(position, usize::try_from(data_size)?)?),
            // Mip
            0x0D => Block::Mip(reader.read_slice_at(position, usize::try_from(data_size)?)?),
            _ => Block::Unknown(
                type_it,
                reader.read_slice_at(position, usize::try_from(data_size)?)?,
            ),
        };

        Ok(block)
    }
}

impl BinaryDeserialize<'_> for Gx2Surface {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let dim = reader.read_at::<u32be>(position)?;
        let width = reader.read_at::<u32be>(position)?;
        let height = reader.read_at::<u32be>(position)?;
        let depth = reader.read_at::<u32be>(position)?;
        let num_mips = reader.read_at::<u32be>(position)?;
        test_le(&num_mips, &13)?;
        let format = reader.read_at::<Format>(position)?;
        let aa = reader.read_at::<u32be>(position)?;
        test_eq(&aa, &0)?;
        let use_it = reader.read_at::<u32be>(position)?;
        let image_size = reader.read_at::<u32be>(position)?;
        let image_ptr = reader.read_at::<u32be>(position)?;
        let mip_size = reader.read_at::<u32be>(position)?;
        let mip_ptr = reader.read_at::<u32be>(position)?;
        let tile_mode = reader.read_at::<u32be>(position)?;
        test_ge(&tile_mode, &0).and(test_le(&tile_mode, &19))?;
        let tile_mode = addr_tile_mode(tile_mode);
        let swizzle = reader.read_at::<u32be>(position)?;
        let alignment = reader.read_at::<u32be>(position)?;
        let pitch = reader.read_at::<u32be>(position)?;
        let mip_offsets = reader.read_at::<[u32be; 13]>(position)?;
        let _slice = reader.read_slice_at(position, 16)?;
        let _comp_sel: [u8; 4] = reader.read_at::<[u8; 4]>(position)?;
        let _slice = reader.read_slice_at(position, 20)?;

        let bpp = format.get_bpp();

        let real_size = if format.is_bcn() {
            ((width + 3) / 4) * ((height + 3) / 4) * (bpp / 8)
        } else {
            width * height * (bpp / 8)
        };

        Ok(Self {
            dim,
            width,
            height,
            depth,
            num_mips,
            format,
            use_it,
            image_size,
            image_ptr,
            mip_size,
            mip_ptr,
            tile_mode,
            swizzle,
            alignment,
            pitch,
            mip_offsets,
            bpp,
            real_size,
        })
    }
}

impl BinaryDeserialize<'_> for Format {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
    }
}

/// Convert a u32 to a tile mode
///
/// # Panics
/// Will panic if the tile mode is invalid
fn addr_tile_mode(tile_mode: u32) -> AddrTileMode {
    match tile_mode {
        0 => AddrTileMode::ADDR_TM_LINEAR_GENERAL,
        1 => AddrTileMode::ADDR_TM_LINEAR_ALIGNED,
        2 => AddrTileMode::ADDR_TM_1D_TILED_THIN1,
        3 => AddrTileMode::ADDR_TM_1D_TILED_THICK,
        4 => AddrTileMode::ADDR_TM_2D_TILED_THIN1,
        5 => AddrTileMode::ADDR_TM_2D_TILED_THIN2,
        6 => AddrTileMode::ADDR_TM_2D_TILED_THIN4,
        7 => AddrTileMode::ADDR_TM_2D_TILED_THICK,
        8 => AddrTileMode::ADDR_TM_2B_TILED_THIN1,
        9 => AddrTileMode::ADDR_TM_2B_TILED_THIN2,
        10 => AddrTileMode::ADDR_TM_2B_TILED_THIN4,
        11 => AddrTileMode::ADDR_TM_2B_TILED_THICK,
        12 => AddrTileMode::ADDR_TM_3D_TILED_THIN1,
        13 => AddrTileMode::ADDR_TM_3D_TILED_THICK,
        14 => AddrTileMode::ADDR_TM_3B_TILED_THIN1,
        15 => AddrTileMode::ADDR_TM_3B_TILED_THICK,
        16 => AddrTileMode::ADDR_TM_2D_TILED_XTHICK,
        17 => AddrTileMode::ADDR_TM_3D_TILED_XTHICK,
        18 => AddrTileMode::ADDR_TM_POWER_SAVE,
        19 => AddrTileMode::ADDR_TM_COUNT,
        _ => panic!("Unknown address tile mode!"),
    }
}

/// Retrieve the data the [`TextureHeader`] points at and create a [`Image`]
#[tracing::instrument(skip(hdr, data))]
fn parse_data_block_to_image(hdr: &Gx2Surface, data: &[u8]) -> Image {
    let bpp = hdr.format.get_bpp() / 8;
    let is_bcn = hdr.format.is_bcn();
    let (width, height) = if is_bcn {
        (hdr.width / 4, hdr.height / 4)
    } else {
        (hdr.width, hdr.height)
    };
    let depth = hdr.depth;
    let swizzle = hdr.swizzle;
    let pitch = hdr.pitch;
    let tile_mode = hdr.tile_mode;

    tracing::trace!("format: {:?}, width: {width}, height: {height}, depth: {depth}, data: {}, swizzle: {swizzle}, pitch: {pitch}, bpp: {bpp}, tile_mode: {tile_mode:?}", hdr.format, data.len());

    let deswizzled = deswizzle_surface(width, height, depth, data, swizzle, pitch, tile_mode, bpp);

    tracing::trace!("deswizzled: {}", deswizzled.len());

    Image {
        surface: *hdr,
        data: deswizzled,
    }
}
