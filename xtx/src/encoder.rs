use std::{borrow::Cow, collections::VecDeque, num::TryFromIntError};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{u32le, u64le},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    testing::TestError,
};
use image::{
    error::{EncodingError, ImageFormatHint},
    ExtendedColorType, ImageEncoder, ImageError, ImageResult,
};
use tegra_swizzle::{
    block_height_mip0,
    surface::{swizzle_surface, BlockDim},
    BlockHeight, SwizzleError,
};
use thiserror::Error;

use crate::{
    Block, BlockData, Format, TextureHeader, XtxRaw, DATA_BLK_TYPE, FIVE_EXPECTED_DATA,
    TEX_HEAD_BLK_TYPE, UNKNOWN_BLK_TYPE_FIVE,
};

pub struct XtxEncoder<W: WriteAt> {
    writer: W,
    position: u64,
    format: Option<Format>,
    mipmaps: u8,
    params: texpresso::Params,
    minor_version: u32,
}

impl<W: WriteAt> XtxEncoder<W> {
    /// Create a new encoder that start writing in `writer` at `position`
    pub fn new(writer: W, position: u64) -> Self {
        Self {
            writer,
            position,
            format: None,
            mipmaps: 0,
            params: texpresso::Params {
                algorithm: texpresso::Algorithm::IterativeClusterFit,
                ..Default::default()
            },
            minor_version: 0x1,
        }
    }

    /// Set the texture encoding that will be used
    ///
    /// If not set, `BC3` will be used for RGBA images
    /// and `BC1` will be used for RGB images.
    pub fn set_format(&mut self, format: Format) {
        self.format = Some(format);
    }

    /// Set the amount of mip maps
    ///
    /// Maximum amount possible is 16.
    ///
    /// # Panics
    /// Will panic if mipmaps is larger than 16.
    pub fn set_mipmaps(&mut self, mipmaps: u8) {
        assert!(mipmaps <= 16, "Only up to 16 mipmaps supported");
        self.mipmaps = mipmaps;
    }

    /// Set the encoder parameters
    ///
    /// The default is `texpresso::Params::default()` except for the
    /// algorithm, which is set to `IterativeClusterFit`.
    pub fn set_encoder_params(&mut self, params: texpresso::Params) {
        self.params = params;
    }

    /// Set the minor version of the XTX format
    ///
    /// Defaults to 1.
    pub fn set_minor_version(&mut self, minor_version: u32) {
        self.minor_version = minor_version;
    }
}

/// Errors returned when the encoder fails
#[derive(Error, Debug)]
pub enum EncoderError {
    /// The input color type was wrong
    #[error("The input color format was not RGBA8")]
    InvalidInputColor,
    /// Swizzling went wrong
    #[error("Texture deswizzling failed")]
    SwizzleError(#[from] SwizzleError),
    /// Read failure
    #[error("Read error")]
    Read(#[from] WriteError),
    /// Test failure
    #[error("Value test failed")]
    Test(#[from] TestError),
    /// Integer conversion failed
    #[error("Integer conversion failed")]
    TryFromInt(#[from] TryFromIntError),
}

impl From<EncoderError> for ImageError {
    fn from(err: EncoderError) -> Self {
        Self::Encoding(EncodingError::new(ImageFormatHint::Name("XTX".into()), err))
    }
}

impl<W: WriteAt> ImageEncoder for XtxEncoder<W> {
    fn write_image(
        mut self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: ExtendedColorType,
    ) -> ImageResult<()> {
        if color_type != ExtendedColorType::Rgba8 {
            return Err(EncoderError::InvalidInputColor.into());
        }
        if self.mipmaps != 0 {
            unimplemented!("Encoding mipmaps is not yet implemented!");
        }

        let format = match self.format {
            Some(format) => format,
            None => {
                if buf.iter().skip(3).step_by(4).all(|b| *b == u8::MAX) {
                    Format::BC1
                } else {
                    Format::BC3
                }
            }
        };

        let width_usize = usize::try_from(width).map_err(EncoderError::from)?;
        let height_usize = usize::try_from(height).map_err(EncoderError::from)?;

        let buffer_size = match format {
            Format::NvnFormatRGBA8 | Format::NvnFormatRGBA8SRGB => buf.len(),
            Format::BC1 => texpresso::Format::Bc1.compressed_size(width_usize, height_usize),
            Format::BC2 => texpresso::Format::Bc2.compressed_size(width_usize, height_usize),
            Format::BC3 => texpresso::Format::Bc3.compressed_size(width_usize, height_usize),
            _ => unimplemented!("Encoding {format:?} is not yet implemented!"),
        };

        let (block_dim, block_height) = if format.is_bcn() {
            (
                BlockDim::block_4x4(),
                block_height_mip0(height_usize.div_ceil(4)),
            )
        } else {
            (BlockDim::uncompressed(), block_height_mip0(height_usize))
        };

        let block_height_log2: u8 = match block_height {
            BlockHeight::One => 0,
            BlockHeight::Two => 1,
            BlockHeight::Four => 2,
            BlockHeight::Eight => 3,
            BlockHeight::Sixteen => 4,
            BlockHeight::ThirtyTwo => 5,
        };

        let header = TextureHeader {
            image_size: u64::try_from(buffer_size).map_err(EncoderError::from)?,
            alignment: 0x200,
            width,
            height,
            depth: 1,
            target: 1,
            format,
            mipmaps: u32::from(self.mipmaps + 1),
            slice_size: u32::try_from(buffer_size).map_err(EncoderError::from)?,
            mipmap_offsets: [0; 17],
            block_height_log2,
        };

        let data = match format {
            Format::NvnFormatRGBA8 | Format::NvnFormatRGBA8SRGB => Cow::Borrowed(buf),
            Format::BC1 => {
                let mut output = vec![0; buffer_size];
                texpresso::Format::Bc1.compress(
                    buf,
                    width_usize,
                    height_usize,
                    self.params,
                    &mut output,
                );
                Cow::Owned(output)
            }
            Format::BC2 => {
                let mut output = vec![0; buffer_size];
                texpresso::Format::Bc2.compress(
                    buf,
                    width_usize,
                    height_usize,
                    self.params,
                    &mut output,
                );
                Cow::Owned(output)
            }
            Format::BC3 => {
                let mut output = vec![0; buffer_size];
                texpresso::Format::Bc3.compress(
                    buf,
                    width_usize,
                    height_usize,
                    self.params,
                    &mut output,
                );
                Cow::Owned(output)
            }
            _ => unreachable!(),
        };

        let swizzled = swizzle_surface(
            width_usize,
            height_usize,
            1,
            &data,
            block_dim,
            Some(block_height),
            usize::try_from(format.get_bpp()).map_err(EncoderError::from)?,
            usize::try_from(header.mipmaps).map_err(EncoderError::from)?,
            1,
        )
        .map_err(EncoderError::from)?;
        drop(data); // Don't keep the original image in ram

        let mut blocks = VecDeque::with_capacity(2);
        blocks.push_back(Block {
            id: 0,
            data: BlockData::TextureHeader(header),
        });
        blocks.push_back(Block {
            id: 1,
            data: BlockData::Data(swizzled),
        });
        blocks.push_back(Block {
            id: 2,
            data: BlockData::Five(Cow::Borrowed(FIVE_EXPECTED_DATA)),
        });

        let xtx = XtxRaw {
            minor_version: self.minor_version,
            blocks,
        };

        self.writer
            .write_at::<XtxRaw>(&mut self.position, xtx)
            .map_err(EncoderError::from)?;

        Ok(())
    }
}

impl BinarySerialize for XtxRaw<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        xtx: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, 0x4E76_4644)?; // magic
        writer.write_at::<u32le>(position, 0x10)?; // size
        writer.write_at::<u32le>(position, 0x1)?; // major version
        writer.write_at::<u32le>(position, xtx.minor_version)?;

        for block in xtx.blocks {
            writer.write_at_with_ctx::<BlockData>(position, block.data, block.id)?;
        }

        Ok(())
    }
}

impl BinarySerialize for BlockData<'_> {
    type Ctx = u32;
    type Input = Self;

    fn serialize_at_with_ctx(
        block: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        id: Self::Ctx,
    ) -> Result<(), WriteError> {
        match block {
            BlockData::TextureHeader(header) => {
                writer.write_at_with_ctx::<TextureHeader>(position, header, id)
            }
            BlockData::Data(data) => {
                let data_size = u64::try_from(data.len())?;
                // Align the data to 0x200
                let data_offset = (*position + 0x24).next_multiple_of(0x200) - *position;
                // Block header
                writer.write_at::<u32le>(position, 0x4E76_4248)?; // magic
                writer.write_at::<u32le>(position, 0x24)?; // header size
                writer.write_at::<u64le>(position, data_size)?;
                writer.write_at::<u64le>(position, data_offset)?;
                writer.write_at::<u32le>(position, DATA_BLK_TYPE)?; // block type
                writer.write_at::<u32le>(position, id)?;
                writer.write_at::<u32le>(position, 0)?; // type idx

                for _ in 0..(data_offset - 0x24) {
                    writer.write_at::<u8>(position, 0)?; // filler
                }

                writer.write_slice_at(position, &data)?;

                Ok(())
            }
            BlockData::Five(data) => {
                let data_size = u64::try_from(data.len())?;
                // Align the data to 0x200
                let data_offset = (*position + 0x24).next_multiple_of(0x200) - *position;
                // Block header
                writer.write_at::<u32le>(position, 0x4E76_4248)?; // magic
                writer.write_at::<u32le>(position, 0x24)?; // header size
                writer.write_at::<u64le>(position, data_size)?;
                writer.write_at::<u64le>(position, data_offset)?;
                writer.write_at::<u32le>(position, UNKNOWN_BLK_TYPE_FIVE)?; // block type
                writer.write_at::<u32le>(position, id)?;
                writer.write_at::<u32le>(position, 0)?; // type idx

                for _ in 0..(data_offset - 0x24) {
                    writer.write_at::<u8>(position, 0)?; // filler
                }

                writer.write_slice_at(position, &data)?;

                Ok(())
            }
            BlockData::DataLazy(_) => unreachable!(),
        }
    }
}

impl BinarySerialize for TextureHeader {
    type Ctx = u32;
    type Input = Self;

    fn serialize_at_with_ctx(
        header: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        id: Self::Ctx,
    ) -> Result<(), WriteError> {
        // Block header
        writer.write_at::<u32le>(position, 0x4E76_4248)?; // magic
        writer.write_at::<u32le>(position, 0x24)?; // header size
        writer.write_at::<u64le>(position, 0x78)?; // data size
        writer.write_at::<u64le>(position, 0x24)?; // data offset
        writer.write_at::<u32le>(position, TEX_HEAD_BLK_TYPE)?; // block type
        writer.write_at::<u32le>(position, id)?;
        writer.write_at::<u32le>(position, 0)?; // type idx

        // Texture header
        writer.write_at::<u64le>(position, header.image_size)?;
        writer.write_at::<u32le>(position, header.alignment)?;
        writer.write_at::<u32le>(position, header.width)?;
        writer.write_at::<u32le>(position, header.height)?;
        writer.write_at::<u32le>(position, header.depth)?;
        writer.write_at::<u32le>(position, header.target)?;
        writer.write_at::<Format>(position, header.format)?;
        writer.write_at::<u32le>(position, header.mipmaps)?;
        writer.write_at::<u32le>(position, header.slice_size)?;
        writer.write_at::<[u32le; 17]>(position, header.mipmap_offsets)?;
        let texture_layout_1 = u32::from(header.block_height_log2);
        writer.write_at::<u32le>(position, texture_layout_1)?;
        writer.write_at::<u32le>(position, 0x7)?; // Texture layout 2
        writer.write_at::<u32le>(position, 0x0)?; // boolean

        Ok(())
    }
}

impl BinarySerialize for Format {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        format: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, u32::from(format))
    }
}
