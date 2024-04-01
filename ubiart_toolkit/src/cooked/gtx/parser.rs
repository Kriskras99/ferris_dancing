use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::{test_any, test_eq, test_le},
};

use super::{
    types::{GfdHeader, Gtx},
    Block, Format, Gx2Surface,
};

// TODO: improve with info from https://mk8.tockdom.com/wiki/GTX%5CGSH_(File_Format)

const COMP_SEL: &[char] = &['R', 'G', 'B', 'A', '0', '1'];

impl<'de> BinaryDeserialize<'de> for Gtx<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let gfd = reader.read_at::<GfdHeader>(position)?;

        let mut blocks = Vec::new();

        let mut num_images = 0;

        loop {
            match reader.read_at::<u32be>(position) {
                Ok(magic) => {
                    *position -= 4;
                    if u32::from(magic) != Block::MAGIC {
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
            if matches!(block, Block::Surface(_)) {
                num_images += 1;
            }
            blocks.push(block);
        }

        test_eq(&num_images, &1)?;

        Ok(Gtx { gfd, blocks })
    }
}

impl BinaryDeserialize<'_> for GfdHeader {
    fn deserialize_at(
        reader: &(impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32be>(position)?.into();
        test_eq(&magic, &Self::MAGIC)?;
        let size = reader.read_at::<u32be>(position)?.into();
        let major_version = reader.read_at::<u32be>(position)?.into();
        test_eq(&major_version, &7u32)?;
        let minor_version = reader.read_at::<u32be>(position)?.into();
        test_eq(&minor_version, &1u32)?;
        let gpu_version = reader.read_at::<u32be>(position)?.into();
        test_eq(&gpu_version, &Self::GPU_VERSION)?;
        let align_mode = reader.read_at::<u32be>(position)?.into();
        let reserved1 = reader.read_at::<u32be>(position)?.into();
        test_eq(&reserved1, &0u32)?;
        let reserved2 = reader.read_at::<u32be>(position)?.into();
        test_eq(&reserved2, &0u32)?;
        test_eq(&(*position - start), &size)?;

        Ok(Self { align_mode })
    }
}

impl<'de> BinaryDeserialize<'de> for Block<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32be>(position)?.into();
        test_eq(&magic, &Self::MAGIC)?;
        let size = u32::from(reader.read_at::<u32be>(position)?);
        let major_version = reader.read_at::<u32be>(position)?.into();
        test_eq(&major_version, &1u32)?;
        let minor_version = reader.read_at::<u32be>(position)?.into();
        test_eq(&minor_version, &0u32)?;
        let type_it = reader.read_at::<u32be>(position)?.into();
        let data_size = u32::from(reader.read_at::<u32be>(position)?);
        let id = reader.read_at::<u32be>(position)?.into();
        test_eq(&id, &0u32)?;
        let type_idx = reader.read_at::<u32be>(position)?.into();
        test_eq(&type_idx, &0u32)?;
        test_eq(&(*position - start), &u64::from(size))?;

        let block = match type_it {
            // Surf
            0x0Bu32 => Block::Surface(reader.read_at(position)?),
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
    fn deserialize_at(
        reader: &(impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let dim = reader.read_at::<u32be>(position)?.into();
        let width = reader.read_at::<u32be>(position)?.into();
        let height = reader.read_at::<u32be>(position)?.into();
        let depth = reader.read_at::<u32be>(position)?.into();
        let num_mips = reader.read_at::<u32be>(position)?.into();
        test_le(&num_mips, &13)?;
        let format = reader.read_at::<Format>(position)?;
        let aa = reader.read_at::<u32be>(position)?.into();
        test_eq(&aa, &0u32)?;
        let use_it = reader.read_at::<u32be>(position)?.into();
        let image_size = reader.read_at::<u32be>(position)?.into();
        let image_ptr = reader.read_at::<u32be>(position)?.into();
        let mip_size = reader.read_at::<u32be>(position)?.into();
        let mip_ptr = reader.read_at::<u32be>(position)?.into();
        let tile_mode = reader.read_at::<u32be>(position)?.into();
        test_any(
            &tile_mode,
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        )?;
        let swizzle = reader.read_at::<u32be>(position)?.into();
        let alignment = reader.read_at::<u32be>(position)?.into();
        let pitch = reader.read_at::<u32be>(position)?.into();

        let mut mip_offsets = [0u32; 13];
        for i in &mut mip_offsets {
            *i = reader.read_at::<u32be>(position)?.into();
        }

        let slice = reader.read_slice_at(position, 16)?;

        let mut comp_sel = [0; 4];
        for i in &mut comp_sel {
            *i = reader.read_at::<u8>(position)?;
        }

        match format {
            Format::TcR5G5B5A1Unorm
            | Format::TcR4G4B4A4Unorm
            | Format::TcsR10G10B10A2Unorm
            | Format::TcsR8G8B8A8Unorm
            | Format::TcsR8G8B8A8Srgb
            | Format::TBc1Srgb
            | Format::TBc1Unorm
            | Format::TBc2Srgb
            | Format::TBc2Unorm
            | Format::TBc3Srgb
            | Format::TBc3Unorm
            | Format::TBc4Snorm
            | Format::TBc4Unorm
            | Format::TBc5Snorm
            | Format::TBc5Unorm => comp_sel = [0, 1, 2, 3],
            Format::TcR4G4Unorm | Format::TcR8G8Unorm => comp_sel = [0, 5, 5, 1],
            Format::TcR8Unorm => comp_sel = [0, 5, 5, 5],
            Format::TcsR5G6B5Unorm => comp_sel = [0, 1, 2, 5],
        }

        let slice = reader.read_slice_at(position, 20)?;

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
            comp_sel,
            bpp,
            real_size,
        })
    }
}

impl BinaryDeserialize<'_> for Format {
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u32 = reader.read_at::<u32be>(position)?.into();
        Self::try_from(value)
    }
}
