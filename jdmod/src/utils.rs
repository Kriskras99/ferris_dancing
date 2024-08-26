//! Various utilities like texture encoding/decoding and dealing with paths
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::{
    bytes::{
        primitives::i16le,
        read::{BinaryDeserialize, BinaryDeserializeExt as _, ReadAtExt},
        write::WriteAt,
        CursorAt,
    },
    test_eq,
    vfs::{VirtualFileSystem, VirtualPath},
};
use hound::SampleFormat;
use image::{imageops, RgbaImage};
use nx_opus::NxOpus;
use regex::Regex;
use ubiart_toolkit::{
    cooked::{
        png::Png,
        wav::{self, AdIn, Codec, Data, Dsp, Fmt, Wav, WavPlatform},
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
        static RE: ::std::sync::OnceLock<regex::Regex> = ::std::sync::OnceLock::new();
        RE.get_or_init(|| ::regex::Regex::new($re).unwrap_or_else(|_| ::std::unreachable!()))
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

/// Decode JD audio file
///
/// Returns true if the audio is opus encoded
pub fn decode_audio(
    reader: &(impl ReadAtExt + ?Sized),
    writer: &mut (impl WriteAt + ?Sized),
) -> Result<bool, Error> {
    let wav = Wav::deserialize(reader)?;

    let fmt = wav
        .chunks
        .get(&Fmt::MAGIC)
        .ok_or_else(|| anyhow!("No `fmt ` chunk!"))?
        .as_fmt()?;

    match (wav.platform, wav.codec) {
        (_, Codec::PCM) => {
            assert_eq!(
                fmt.bits_per_sample, 16,
                "Bits per sample != 16, this is not supported"
            );
            let data = wav.chunks[&Data::MAGIC].as_data()?;

            let spec = hound::WavSpec {
                channels: fmt.channel_count,
                sample_rate: fmt.sample_rate,
                bits_per_sample: fmt.bits_per_sample,
                sample_format: hound::SampleFormat::Int,
            };
            let buffer = CursorAt::new(writer, 0);
            let mut writer = hound::WavWriter::new(buffer, spec)?;
            let mut sample_writer = writer.get_i16_writer(u32::try_from(data.data.len() / 2)?);

            let mut position = 0;
            for _ in 0..(data.data.len() / 2) {
                let sample = data.data.read_at::<i16le>(&mut position)?;
                sample_writer.write_sample(sample);
            }
            sample_writer.flush()?;
            writer.finalize()?;
            Ok(false)
        }
        (WavPlatform::Switch, Codec::Nx) => {
            let data = wav.chunks[&Data::MAGIC].as_data()?;
            let mut position = 0;
            let adin = wav
                .chunks
                .get(&AdIn::MAGIC)
                .ok_or_else(|| anyhow!("No 'AdIn' chunk!"))?
                .as_adin()?;
            NxOpus::mux_to_opus(
                data.data.as_ref(),
                &mut position,
                writer,
                adin.num_of_samples,
            )?;
            Ok(true)
        }
        (WavPlatform::WiiU, Codec::Adpc) => {
            let spec = hound::WavSpec {
                channels: fmt.channel_count,
                sample_rate: fmt.sample_rate,
                bits_per_sample: fmt.bits_per_sample,
                sample_format: hound::SampleFormat::Int,
            };

            if let Some(data) = wav.chunks.get(&Data::MAGIC_STEREO) {
                // interleaved per frame
                let data = data.as_data()?;
                let dsp_left = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp()?;
                let dsp_right = wav.chunks[&Dsp::MAGIC_RIGHT].as_dsp()?;

                let left_state = gc_adpcm::DspState {
                    hist1: dsp_left.initial_sample_history_1,
                    hist2: dsp_left.initial_sample_history_2,
                    coefficients: dsp_left.coefficients,
                };
                let right_state = gc_adpcm::DspState {
                    hist1: dsp_left.initial_sample_history_1,
                    hist2: dsp_left.initial_sample_history_2,
                    coefficients: dsp_left.coefficients,
                };
                let total_frames = dsp_left.sample_count.div_ceil(gc_adpcm::SAMPLES_PER_FRAME) * 2;
                assert_eq!(
                    dsp_left.sample_count, dsp_right.sample_count,
                    "One channel has more samples than the other"
                );

                let decoder = gc_adpcm::Decoder::interleaved_stereo(
                    data.data.as_ref(),
                    0,
                    left_state,
                    right_state,
                    total_frames,
                );

                let mut buffer = CursorAt::new(writer, 0);
                let mut writer = hound::WavWriter::new(&mut buffer, spec)?;
                let mut sample_writer = writer.get_i16_writer(dsp_left.sample_count * 2);

                for sample in decoder {
                    let sample = sample?;
                    sample_writer.write_sample(sample);
                }
                sample_writer.flush()?;
                writer.finalize()?;
                Ok(false)
            } else if let Some(data_right) = wav.chunks.get(&Data::MAGIC_RIGHT) {
                // non-interleaved stereo
                let data_right = data_right.as_data()?;
                let data_left = wav.chunks[&Data::MAGIC_LEFT].as_data()?;
                let dsp_right = wav.chunks[&Dsp::MAGIC_RIGHT].as_dsp()?;
                let dsp_left = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp()?;

                let left_state = gc_adpcm::DspState {
                    hist1: dsp_left.initial_sample_history_1,
                    hist2: dsp_left.initial_sample_history_2,
                    coefficients: dsp_left.coefficients,
                };
                let right_state = gc_adpcm::DspState {
                    hist1: dsp_right.initial_sample_history_1,
                    hist2: dsp_right.initial_sample_history_2,
                    coefficients: dsp_right.coefficients,
                };
                assert_eq!(
                    dsp_left.sample_count, dsp_right.sample_count,
                    "One channel has more samples than the other"
                );
                let total_frames = dsp_left.sample_count.div_ceil(gc_adpcm::SAMPLES_PER_FRAME);

                let decoder = gc_adpcm::Decoder::stereo(
                    data_left.data.as_ref(),
                    0,
                    left_state,
                    data_right.data.as_ref(),
                    0,
                    right_state,
                    total_frames,
                );

                let mut buffer = CursorAt::new(writer, 0);
                let mut writer = hound::WavWriter::new(&mut buffer, spec)?;
                let mut sample_writer = writer.get_i16_writer(dsp_left.sample_count * 2);

                for sample in decoder {
                    let sample = sample?;
                    sample_writer.write_sample(sample);
                }
                sample_writer.flush()?;
                writer.finalize()?;
                Ok(false)
            } else if let Some(data) = wav.chunks.get(&Data::MAGIC_LEFT) {
                // mono
                let data = data.as_data()?;
                let dsp = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp()?;

                let state = gc_adpcm::DspState {
                    hist1: dsp.initial_sample_history_1,
                    hist2: dsp.initial_sample_history_2,
                    coefficients: dsp.coefficients,
                };
                let total_frames = dsp.sample_count.div_ceil(gc_adpcm::SAMPLES_PER_FRAME);
                let decoder = gc_adpcm::Decoder::mono(data.data.as_ref(), 0, state, total_frames);

                let mut buffer = CursorAt::new(writer, 0);
                let mut writer = hound::WavWriter::new(&mut buffer, spec)?;
                let mut sample_writer = writer.get_i16_writer(dsp.sample_count);

                for sample in decoder {
                    let sample = sample?;
                    sample_writer.write_sample(sample);
                }
                sample_writer.flush()?;
                writer.finalize()?;
                Ok(false)
            } else {
                Err(anyhow!("Unexpected WiiU/ADPC configuration: {wav:?}"))
            }
        }
        _ => Err(anyhow!(
            "Unsupported platform/codec combination: {:?} {:?}",
            wav.platform,
            wav.codec
        )),
    }
}

/// Encode a JD audio file
pub fn encode_audio(
    vfs: &impl VirtualFileSystem,
    image_path: &VirtualPath,
) -> Result<Vec<u8>, Error> {
    if image_path.extension() != Some("wav") && image_path.extension() != Some("opus") {
        return Err(anyhow!(
            "This application only supports .wav and .opus files"
        ));
    }
    let file = vfs.open(image_path)?;
    let magic = file.read_at::<[u8; 4]>(&mut 0)?;
    match &magic {
        b"OggS" => {
            let mut nx_opus = Vec::new();
            let (header, num_of_samples) = NxOpus::mux_from_opus(&file, &mut nx_opus, &mut 0)?;

            let fmt = Fmt {
                unk1: 1, // TODO: check this
                channel_count: u16::from(header.channels),
                sample_rate: header.sample_rate,
                unk2: 192_000,       // TODO: check this
                block_align: 4,      // TODO: check this
                bits_per_sample: 16, // TODO: check this
                unk3: None,
            };

            let adin = AdIn { num_of_samples };

            let mut vec = Vec::new();
            wav::Writer::create_opus(&mut vec, &mut 0, fmt, adin, &nx_opus)?;

            Ok(vec)
        }
        b"RIFF" => {
            let mut vec = Vec::new();
            let decoder = hound::WavReader::new(CursorAt::new(file, 0))?;
            let spec = decoder.spec();
            test_eq!(spec.sample_format, SampleFormat::Int)
                .and(test_eq!(spec.bits_per_sample, 16))?;

            let fmt = Fmt {
                unk1: 1,
                channel_count: spec.channels,
                sample_rate: spec.sample_rate,
                unk2: 192_000,
                block_align: 4,
                bits_per_sample: 16,
                unk3: None,
            };

            let samples = decoder.into_samples().collect::<Result<Vec<_>, _>>()?;

            wav::Writer::create_pcm(&mut vec, &mut 0, fmt, &samples)?;

            Ok(vec)
        }
        _ => Err(anyhow!(
            "This application only supports .wav and .opus files"
        )),
    }
}
