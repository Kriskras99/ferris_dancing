//! Various utilities like texture encoding/decoding and dealing with paths
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::{
    bytes::read::ReadAtExt,
    vfs::{VirtualFileSystem, VirtualPath},
};
use image::{imageops, EncodableLayout, RgbaImage};
use regex::Regex;
use texpresso::Format;
use ubiart_toolkit::{
    cooked::{
        gtx,
        png::{self, Png, Texture},
        xtx::{self, Image, Index, TextureHeader},
    },
    utils::{Platform, UniqueGameId},
};

/// Cook a path so it stars with 'cache/itf_cooked/...'
///
/// # Errors
/// Will return an error if it's unknown how the path or the platform should be cooked
pub fn cook_path(path: &str, platform: Platform) -> Result<String, Error> {
    let path = path.strip_prefix('/').unwrap_or(path);

    // Just return if it is already cooked
    if path.starts_with("cache/itf_cooked/") {
        return Ok(path.to_string());
    }

    // Reserve enough memory for the entire cooked path: original path + cooked prefix + .ckd + max platform name
    let mut cooked =
        String::with_capacity(path.len() + "cache/itf_cooked/".len() + 4 + "durango".len());
    cooked.push_str("cache/itf_cooked/");

    match platform {
        Platform::Nx => cooked.push_str("nx/"),
        Platform::WiiU => cooked.push_str("wiiu/"),
        _ => Err(anyhow!("Not yet implemented for {path}"))?,
    };

    cooked.push_str(path);

    // Early exit if there's no filename
    if path.ends_with('/') {
        return Ok(cooked);
    }

    if let Some((_, extension)) = path.rsplit_once('.') {
        match extension {
            "tpl" | "tape" | "ktape" | "dtape" | "wav" | "png" | "tga" | "isg" | "isc" | "sgs"
            | "json" | "act" => cooked.push_str(".ckd"),
            _ => Err(anyhow!(
                "Cooking extension '{extension}' not yet implemented! Full path: {path}"
            ))?,
        };
    } else {
        match path {
            "sgscontainer" => cooked.push_str(".ckd"),
            _ => Err(anyhow!("Don't know how to cook: {path}!"))?,
        }
    }

    Ok(cooked)
}

/// With this macro you can create a Regex that is only compiled once.
#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

/// Decode a XTX texture into an image buffer
///
/// # Errors
/// Will return an error if the parsing fails
/// Will return an error if the decoded image doesn't fit into memory
///
/// # Panics
/// Will panic if there is more than one image in the texture
pub fn decode_texture(
    reader: &(impl ReadAtExt + ?Sized),
    ugi: UniqueGameId,
) -> Result<RgbaImage, Error> {
    let png = png::parse(reader, ugi)?;

    let png_height = u32::from(png.height);
    let png_width = u32::from(png.width);

    let mut buffer = match png.texture {
        png::Texture::Xtx(xtx) => {
            assert!(
                xtx.images.len() == 1,
                "More than one image in texture, not supported!"
            );

            let big_image = &xtx.images[0];
            let header = &big_image.header;
            let width = usize::try_from(header.width)?;
            let height = usize::try_from(header.height)?;

            let data_decompressed = match header.format {
                xtx::Format::DXT1 => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc1.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                xtx::Format::DXT3 => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc2.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                xtx::Format::DXT5 => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc3.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                xtx::Format::NvnFormatRGBA8 => big_image.data.clone(),
                _ => unimplemented!("{:?}", header.format),
            };

            RgbaImage::from_vec(header.width, header.height, data_decompressed)
                .ok_or_else(|| anyhow!("Failure decoding!"))?
        }
        png::Texture::Gtx(gtx) => {
            assert!(
                gtx.images.len() == 1,
                "More than one image in texture, not supported!"
            );

            let big_image = &gtx.images[0];
            let header = &big_image.surface;
            let width = usize::try_from(header.width)?;
            let height = usize::try_from(header.height)?;

            let data_decompressed = match header.format {
                gtx::Format::TBc1Srgb | gtx::Format::TBc1Unorm => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc1.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                gtx::Format::TBc2Srgb | gtx::Format::TBc2Unorm => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc2.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                gtx::Format::TBc3Srgb | gtx::Format::TBc3Unorm => {
                    // TODO: Replace with Vec::with_capacity
                    let mut data_decompressed = vec![0xFF; width * height * 4];
                    Format::Bc3.decompress(&big_image.data, width, height, &mut data_decompressed);
                    data_decompressed
                }
                gtx::Format::TcsR8G8B8A8Srgb | gtx::Format::TcsR8G8B8A8Unorm => {
                    big_image.data.clone()
                }
                _ => unimplemented!("{:?}", header.format),
            };

            RgbaImage::from_vec(header.width, header.height, data_decompressed)
                .ok_or_else(|| anyhow!("Failure decoding!"))?
        }
        png::Texture::None => unreachable!("Should not exist!"),
    };

    if png_width != buffer.width() || png_height != buffer.height() {
        buffer = imageops::resize(
            &buffer,
            png_width,
            png_height,
            imageops::FilterType::Lanczos3,
        );
    }

    Ok(buffer)
}

// TODO: Create mipmaps if requested
// TODO: Use better codecs
/// Encode a image at `image_path` as an XTX texture
///
/// If the file has no alpha (alpha is all 1), then the BC1 codec is used.
/// Otherwise the BC3 codec is used.
///
/// # Errors
/// Will return an error if any IO or parsing fails
#[tracing::instrument(skip(vfs, image_path))]
pub fn encode_texture(
    vfs: &impl VirtualFileSystem,
    image_path: &VirtualPath,
) -> Result<Png, Error> {
    // let mipmaps = false;
    let img_file = vfs.open(image_path)?;
    let img = image::load_from_memory(&img_file)?;
    let img = img.into_rgba8();

    let width = u16::try_from(img.width())?;
    let height = u16::try_from(img.height())?;

    if img.pixels().all(|p| p.0[3] == u8::MAX) {
        // Image has no transparency, so encode as BC1/DXT1

        let mut new_picto = Png {
            height,
            width,
            unk5: 0x1800,
            ..Default::default()
        };

        // TODO: Use Vec::with_capacity
        let mut data = vec![
            0;
            texpresso::Format::Bc1
                .compressed_size(usize::from(width), usize::from(height))
        ];

        texpresso::Format::Bc1.compress(
            img.as_bytes(),
            usize::from(width),
            usize::from(height),
            texpresso::Params {
                algorithm: texpresso::Algorithm::IterativeClusterFit,
                weights: texpresso::COLOUR_WEIGHTS_PERCEPTUAL,
                weigh_colour_by_alpha: false,
            },
            &mut data,
        );

        tracing::trace!(
            "width: {width}, height: {height}, format: DXT1, data_size: {}",
            data.len()
        );
        let indexes = vec![Index {
            width: usize::from(width),
            height: usize::from(height),
            offset: 0,
            size: data.len(),
        }];
        let image = Image {
            header: TextureHeader {
                // TODO! Check these values!
                image_size: u64::try_from(data.len())?,
                alignment: 0x200,
                width: u32::from(width),
                height: u32::from(height),
                depth: 1,
                target: 1,
                format: xtx::Format::DXT1,
                mipmaps: 1,
                slice_size: u32::try_from(data.len())?,
                mipmap_offsets: [0; 17],
                block_height_log2: 0,
            },
            data,
            indexes,
        };
        // if mipmaps {
        //     image.data.reserve_exact(0xf);
        //     for i in 1..=0x10 {
        //         if width >> i == 0 || height >> i == 0 {
        //             break;
        //         }
        //         let mipmap = imageops::resize(&img, width >> i, height >> i, imageops::FilterType::Triangle);

        //     }
        // }

        new_picto.texture = Texture::Xtx(xtx::Xtx {
            images: vec![image],
            ..Default::default()
        });

        Ok(new_picto)
    } else {
        // Image has transparency, so encode as BC3/DXT5

        let mut new_picto = Png {
            height,
            width,
            unk5: 0x2000,
            ..Default::default()
        };

        // TODO: Use Vec::with_capacity
        let mut data = vec![
            0;
            texpresso::Format::Bc3
                .compressed_size(usize::from(width), usize::from(height))
        ];

        texpresso::Format::Bc3.compress(
            img.as_bytes(),
            usize::from(width),
            usize::from(height),
            texpresso::Params {
                algorithm: texpresso::Algorithm::IterativeClusterFit,
                weights: texpresso::COLOUR_WEIGHTS_PERCEPTUAL,
                weigh_colour_by_alpha: true,
            },
            &mut data,
        );

        let indexes = vec![Index {
            width: usize::from(width),
            height: usize::from(height),
            offset: 0,
            size: data.len(),
        }];
        let image = Image {
            header: TextureHeader {
                // TODO! Check these values!
                image_size: u64::try_from(data.len())?,
                alignment: 0x200,
                width: u32::from(width),
                height: u32::from(height),
                depth: 1,
                target: 1,
                format: xtx::Format::DXT5,
                mipmaps: 1,
                slice_size: u32::try_from(data.len())?,
                mipmap_offsets: [0; 17],
                block_height_log2: 0,
            },
            data,
            indexes,
        };

        new_picto.texture = Texture::Xtx(xtx::Xtx {
            images: vec![image],
            ..Default::default()
        });

        Ok(new_picto)
    }
}

/// Efficient implementation of `(_, [needle]) = regex.captures(haystack).extract()` for `Cow<str>`
///
/// # Errors
/// Returns an error if the needle is not in the haystack
pub fn cow_regex_single_capture<'a>(
    regex: &Regex,
    haystack: Cow<'a, str>,
) -> Result<Cow<'a, str>, Error> {
    match haystack {
        Cow::Borrowed(haystack) => {
            let (_, [needle]) = regex
                .captures(haystack)
                .ok_or_else(|| anyhow!("No needle found! Haystack: {haystack}, regex: {regex:?}"))?
                .extract();
            Ok(Cow::Borrowed(needle))
        }
        Cow::Owned(haystack) => {
            let (_, [needle]) = regex
                .captures(&haystack)
                .ok_or_else(|| anyhow!("No needle found! Haystack: {haystack}, regex: {regex:?}"))?
                .extract();
            Ok(Cow::Owned(String::from(needle)))
        }
    }
}
