//! Various utilities like texture encoding/decoding and dealing with paths
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::{
    bytes::read::{BinaryDeserialize, ReadAtExt},
    vfs::{VirtualFileSystem, VirtualPath},
};
use image::{imageops, RgbaImage};
use regex::Regex;
use ubiart_toolkit::{
    cooked::png::Png,
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
    let png = Png::deserialize_with(reader, ugi)?;

    let png_height = u32::from(png.height);
    let png_width = u32::from(png.width);

    let mut buffer = png.texture;

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

/// Encode a image at `image_path` as an XTX texture
///
/// # Errors
/// Will return an error if any IO or parsing fails
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

    Ok(Png {
        width,
        height,
        unk5: 0x2000,
        texture: img,
        ..Default::default()
    })
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
