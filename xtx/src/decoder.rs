//! Contains the parser implementation

use std::{collections::VecDeque, num::TryFromIntError};

use dotstar_toolkit_utils::bytes::{
    primitives::{u32le, u64le},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};
use image::{
    error::{DecodingError, ImageFormatHint},
    ColorType, ImageDecoder, ImageError, ImageResult,
};
use tegra_swizzle::{
    surface::{deswizzle_surface, BlockDim},
    BlockHeight, SwizzleError,
};
use test_eq::{test_eq, test_le, TestFailure};
use thiserror::Error;

use crate::types::{
    Block, BlockData, Data, Format, TextureHeader, XtxRaw, DATA_BLK_TYPE, FIVE_EXPECTED_DATA,
    TEX_HEAD_BLK_TYPE, UNKNOWN_BLK_TYPE_FIVE,
};

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
    Test(#[from] TestFailure),
    /// Integer conversion failed
    #[error("Integer conversion failed")]
    TryFromInt(#[from] TryFromIntError),
}

impl From<DecoderError> for ImageError {
    fn from(err: DecoderError) -> Self {
        Self::Decoding(DecodingError::new(ImageFormatHint::Name("XTX".into()), err))
    }
}

/// Decoder for Nvidia Tegra X1 texture files
///
/// Most common extension is `.xtx`.
/// Magic is `NvFD`.
///
/// The decoder is lazy and will only read the metadata in the file. The
/// image data is read on demand after calling [`read_image`].
pub struct XtxDecoder<R: ReadAtExt> {
    reader: R,
    header: TextureHeader,
    data: Data,
    pub minor_version: u32,
}

impl<R: ReadAtExt> XtxDecoder<R> {
    /// Creates a new decoder that reads from `reader` starting at `position`
    ///
    /// `position` will be updated on the return of this function to point to
    /// the end of the texture file unless an error is returned. In that case
    /// `position` will be in between the start and end of the texture file.
    pub fn new(reader: R, position: &mut u64) -> Result<Self, DecoderError> {
        let xtx = reader.read_at::<XtxRaw>(position)?;

        let minor_version = xtx.minor_version;
        let mut blocks = xtx.blocks;

        let mut image = None;

        while !blocks.is_empty() {
            let block = blocks.pop_front().unwrap_or_else(|| unreachable!());
            match block.data {
                BlockData::TextureHeader(hdr) => {
                    let second_block = blocks.pop_front();
                    let data = match second_block {
                        Some(block) => match block.data {
                            BlockData::DataLazy(data) => Ok(data),
                            _ => Err(DecoderError::HeaderBlockWithoutDataBlock),
                        },
                        None => Err(DecoderError::HeaderBlockWithoutDataBlock),
                    }?;

                    if image.is_some() {
                        Err(DecoderError::MoreThanOneImage)
                    } else {
                        image = Some((hdr, data));
                        Ok(())
                    }
                }
                BlockData::DataLazy(_) => Err(DecoderError::DataBlockWithoutHeaderBlock),
                BlockData::Five(_) => Ok(()),
                BlockData::Data(_) => unreachable!(),
            }?;
        }

        let (header, data) = image.ok_or(DecoderError::NoImages)?;

        Ok(Self {
            reader,
            header,
            data,
            minor_version,
        })
    }
}

impl<R: ReadAtExt> ImageDecoder for XtxDecoder<R> {
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
        let expected_size =
            usize::try_from(self.total_bytes()).expect("Image is too big for memory");

        assert_eq!(
            buf.len(),
            expected_size,
            "Buffer is too small or too big for the image"
        );

        let block_dim = if self.header.format.is_bcn() {
            BlockDim::block_4x4()
        } else {
            BlockDim::uncompressed()
        };

        let block_height_log2 = u32::from(self.header.block_height_log2);
        let block_height_mip0 = BlockHeight::new(2u32.pow(block_height_log2))
            .ok_or(DecoderError::InvalidBlockHeight(block_height_log2))?;

        let mut position = self.data.position;

        let data = self
            .reader
            .read_slice_at(&mut position, self.data.size)
            .map_err(DecoderError::from)?;
        let deswizzled = deswizzle_surface(
            self.header.width,
            self.header.height,
            self.header.depth,
            &data,
            block_dim,
            Some(block_height_mip0),
            self.header.format.bytes_per_pixel(),
            self.header.mipmaps,
            1,
        )
        .map_err(DecoderError::from)?;
        drop(data); // drop original data early

        let width = usize::try_from(self.header.width).map_err(DecoderError::from)?;
        let height = usize::try_from(self.header.height).map_err(DecoderError::from)?;
        match self.header.format {
            Format::BC1 => {
                texpresso::Format::Bc1.decompress(&deswizzled, width, height, buf);
            }
            Format::BC2 => {
                texpresso::Format::Bc2.decompress(&deswizzled, width, height, buf);
            }
            Format::BC3 => {
                texpresso::Format::Bc3.decompress(&deswizzled, width, height, buf);
            }
            Format::NvnFormatRGBA8 => buf.copy_from_slice(&deswizzled[..expected_size]),
            _ => unimplemented!(
                "Decoding of {:?} is not yet implemented",
                self.header.format
            ),
        }

        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        (*self).read_image(buf)
    }
}

impl<'de> BinaryDeserialize<'de> for XtxRaw<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let magic = reader.read_at::<u32le>(position)?;
        test_eq!(magic, 0x4E76_4644)?;

        let size = reader.read_at::<u32le>(position)?;
        test_eq!(size, 0x10)?;

        let major_version = reader.read_at::<u32le>(position)?;
        test_eq!(major_version, 0x1)?;

        let minor_version = reader.read_at::<u32le>(position)?;

        let mut blocks = VecDeque::new();

        loop {
            match reader.read_at::<u32le>(position) {
                Ok(magic) => {
                    *position -= 4;
                    if magic != 0x4E76_4248 {
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

        Ok(XtxRaw {
            minor_version,
            blocks,
        })
    }
}

impl<'de> BinaryDeserialize<'de> for Block<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let start = *position;
        let magic = reader.read_at::<u32le>(position)?;
        test_eq!(magic, 0x4E76_4248)?;
        let header_size = u64::from(reader.read_at::<u32le>(position)?);
        test_eq!(header_size, 0x24)?;
        let data_size = usize::try_from(reader.read_at::<u64le>(position)?)?;
        let data_offset = reader.read_at::<u64le>(position)?;
        let block_type = reader.read_at::<u32le>(position)?;
        let id = reader.read_at::<u32le>(position)?;
        let type_idx = reader.read_at::<u32le>(position)?;
        test_eq!(type_idx, 0x0)?;

        let pos = *position;
        let block_data = match block_type {
            TEX_HEAD_BLK_TYPE => {
                test_eq!(data_size, 0x78)?;
                test_eq!(data_offset, 0x24)?;
                Ok(BlockData::TextureHeader(
                    reader.read_at::<TextureHeader>(position)?,
                ))
            }
            DATA_BLK_TYPE => {
                *position = pos + data_offset - header_size;
                let data_position = *position;
                *position += u64::try_from(data_size)?;
                Ok(BlockData::DataLazy(Data {
                    position: data_position,
                    size: data_size,
                }))
            }
            UNKNOWN_BLK_TYPE_FIVE => {
                *position = pos + data_offset - header_size;
                let data = reader.read_slice_at(position, data_size)?;
                test_eq!(&data, &FIVE_EXPECTED_DATA)?;
                Ok(BlockData::Five(data))
            }
            _ => Err(ReadError::custom(format!(
                "Unknown block type found: {block_type:x}"
            ))),
        }?;

        let data_size = u64::try_from(data_size)?;

        let new_pos = *position;
        test_eq!((new_pos - pos), (data_size + data_offset - header_size))?;

        *position = start + data_offset + data_size;
        Ok(Block {
            id,
            data: block_data,
        })
    }
}

impl BinaryDeserialize<'_> for Format {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let value = reader.read_at::<u32le>(position)?;
        Self::try_from(value).map_err(|_| ReadError::custom(format!("Unknown format: {value:x}")))
    }
}

impl BinaryDeserialize<'_> for TextureHeader {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let image_size = reader.read_at::<u64le>(position)?;
        let alignment = reader.read_at::<u32le>(position)?;
        test_eq!(alignment, 0x200)?;
        let width = reader.read_at::<u32le>(position)?;
        let height = reader.read_at::<u32le>(position)?;
        let depth = reader.read_at::<u32le>(position)?;
        test_eq!(depth, 1)?;
        let target = reader.read_at::<u32le>(position)?;
        test_eq!(target, 1)?;
        let format = reader.read_at::<Format>(position)?;
        let mip_count = reader.read_at::<u32le>(position)?;
        test_le!(mip_count, 17)?;
        let slice_size = reader.read_at::<u32le>(position)?;

        test_eq!(image_size, u64::from(slice_size))?;

        let mut mipmap_offsets = [0; 17];
        for i in &mut mipmap_offsets {
            *i = reader.read_at::<u32le>(position)?;
        }

        let texture_layout_1: u32 = reader.read_at::<u32le>(position)?;
        test_eq!((texture_layout_1 & !0b111), 0)?;
        let texture_layout_2 = reader.read_at::<u32le>(position)?;
        test_eq!(texture_layout_2, 7u32)?;
        let boolean = reader.read_at::<u32le>(position)?;
        test_eq!(boolean, 0u32)?;

        let block_height_log2 =
            u8::try_from(texture_layout_1 & 0b111).unwrap_or_else(|_| unreachable!());

        Ok(Self {
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
        })
    }
}
