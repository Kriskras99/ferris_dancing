use std::{collections::VecDeque, num::TryFromIntError};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    testing::{test_eq, test_ge, test_le, TestError},
};
use image::{
    error::{DecodingError, ImageFormatHint},
    ColorType, ImageDecoder, ImageError, ImageResult,
};
use thiserror::Error;
use wiiu_swizzle::{deswizzle_mipmap, SwizzleError, TileMode};

use crate::types::{Block, Data, Format, GfdHeader, GtxRaw, Gx2Surface};

/// Errors returned when the decoder fails
#[derive(Error, Debug)]
pub enum DecoderError {
    /// Encountered unknown texture format
    #[error("Unknown texture format found: 0x{0:x}")]
    UnknownTextureFormat(u32),
    /// Encountered a header block without a data block
    #[error("Found a header block without a data block")]
    HeaderBlockWithoutDataBlock,
    /// Encountered a data block without a header block
    #[error("Found a data block without a header block")]
    DataBlockWithoutHeaderBlock,
    /// More than one image in the texture file
    #[error("More than one image in the texture")]
    MoreThanOneImage,
    /// No image in the texture file
    #[error("No image in the texture")]
    NoImages,
    /// Invalid block height
    #[error("Invalid block height: 2^{0}")]
    InvalidBlockHeight(u32),
    /// Swizzling went wrong
    #[error("Texture deswizzling failed")]
    SwizzleError(#[from] SwizzleError),
    /// Read failure
    #[error("Read error")]
    Read(#[from] ReadError),
    /// Test failure
    #[error("Value test failed")]
    Test(#[from] TestError),
    /// Integer conversion failed
    #[error("Integer conversion failed")]
    TryFromInt(#[from] TryFromIntError),
}

impl From<DecoderError> for ImageError {
    fn from(err: DecoderError) -> Self {
        Self::Decoding(DecodingError::new(ImageFormatHint::Name("XTX".into()), err))
    }
}

pub struct GtxDecoder<R: ReadAtExt> {
    reader: R,
    header: Gx2Surface,
    data: Data,
    pub align_mode: u32,
}

impl<R: ReadAtExt> GtxDecoder<R> {
    pub fn new(reader: R, position: &mut u64) -> Result<Self, DecoderError> {
        let gtx: GtxRaw = reader.read_at::<GtxRaw>(position)?;

        let mut blocks = gtx.blocks;
        let mut image = None;

        while !blocks.is_empty() {
            let block = blocks.pop_front().unwrap_or_else(|| unreachable!());
            match block {
                Block::Surface(hdr) => {
                    let second_block = blocks.pop_front();
                    let data = match second_block {
                        Some(Block::DataLazy(data)) => Ok(data),
                        _ => Err(DecoderError::HeaderBlockWithoutDataBlock),
                    }?;

                    if image.is_some() {
                        Err(DecoderError::MoreThanOneImage)
                    } else {
                        image = Some((hdr, data));
                        Ok(())
                    }
                }
                Block::DataLazy(_) => Err(DecoderError::DataBlockWithoutHeaderBlock),
                Block::Mip(_) => Ok(()),
            }?;
        }

        let (header, data) = image.ok_or(DecoderError::NoImages)?;

        let align_mode = gtx.header.align_mode;
        Ok(Self {
            reader,
            header,
            data,
            align_mode,
        })
    }
}

impl<R: ReadAtExt> ImageDecoder for GtxDecoder<R> {
    fn dimensions(&self) -> (u32, u32) {
        (self.header.width, self.header.height)
    }

    fn color_type(&self) -> ColorType {
        // everything is decoded to RGBA8, even if there is no alpha channel
        ColorType::Rgba8
    }

    fn read_image(self, buf: &mut [u8]) -> ImageResult<()>
    where
        Self: Sized,
    {
        assert_eq!(
            u64::try_from(buf.len()).expect("Image is too big for usize"),
            self.total_bytes(),
            "Buffer is too small or too big for the image"
        );

        let hdr = self.header;
        let bpp = hdr.format.get_bpp();
        let is_bcn = hdr.format.is_bcn();
        let (width, height) = if is_bcn {
            (hdr.width / 4, hdr.height / 4)
        } else {
            (hdr.width, hdr.height)
        };
        let width_usize = usize::try_from(width).map_err(DecoderError::from)?;
        let height_usize = usize::try_from(height).map_err(DecoderError::from)?;
        let depth = hdr.depth;
        let swizzle = hdr.swizzle;
        let pitch = hdr.pitch;
        let tile_mode = hdr.tile_mode;

        let mut position = self.data.position;

        let data = self
            .reader
            .read_slice_at(&mut position, self.data.size)
            .map_err(DecoderError::from)?;
        let deswizzled = deswizzle_mipmap(
            width,
            height,
            depth,
            &data,
            swizzle,
            pitch,
            tile_mode,
            bpp,
            wiiu_swizzle::AaMode::X1,
        )
        .map_err(DecoderError::from)?;
        drop(data); // drop original data early
        match hdr.format {
            Format::TBc1Srgb | Format::TBc1Unorm => {
                texpresso::Format::Bc1.decompress(&deswizzled, width_usize, height_usize, buf);
            }
            Format::TBc2Srgb | Format::TBc2Unorm => {
                texpresso::Format::Bc2.decompress(&deswizzled, width_usize, height_usize, buf);
            }
            Format::TBc3Srgb | Format::TBc3Unorm => {
                texpresso::Format::Bc3.decompress(&deswizzled, width_usize, height_usize, buf);
            }
            Format::TcsR8G8B8A8Srgb | Format::TcsR8G8B8A8Unorm => buf.copy_from_slice(&deswizzled),
            _ => unimplemented!("Decoding of {:?} is not yet implemented", hdr.format),
        }

        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        (*self).read_image(buf)
    }
}

impl<'de> BinaryDeserialize<'de> for GtxRaw<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let header = reader.read_at::<GfdHeader>(position)?;

        let mut blocks = VecDeque::new();

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
            blocks.push_back(block);
        }

        Ok(GtxRaw { header, blocks })
    }
}

impl BinaryDeserialize<'_> for GfdHeader {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
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

    fn deserialize_at_with(
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
            0x0Bu32 => Ok(Block::Surface(reader.read_at::<Gx2Surface>(position)?)),
            // Data
            0x0C => Ok(Block::DataLazy(Data {
                position: *position,
                size: usize::try_from(data_size)?,
            })),
            // Mip
            0x0D => Ok(Block::Mip(
                reader.read_slice_at(position, usize::try_from(data_size)?)?,
            )),
            _ => Err(ReadError::custom(format!(
                "Unknown block type: 0x{type_it:x}"
            ))),
        }?;

        Ok(block)
    }
}

impl BinaryDeserialize<'_> for Gx2Surface {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
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
        let tile_mode = TileMode::from_repr(tile_mode)
            .ok_or_else(|| ReadError::custom("Tile mode is invalid!".into()))?;
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

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
    }
}
