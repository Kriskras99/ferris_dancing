#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Error};
use clap::Parser;
use dotstar_toolkit_utils::{
    bytes::{
        primitives::i16le,
        read::{BinaryDeserializeExt as _, ReadAtExt},
    },
    test_eq,
};
use hound::SampleFormat;
use nx_opus::{mux_from_opus, mux_to_opus};
use tracing::{level_filters::LevelFilter, trace};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
use ubiart_toolkit::cooked::wav::{
    self, AdIn, Codec, Data, Dsp, Fmt, Mark, Strg, Wav, WavPlatform,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
    output_dir: Option<PathBuf>,
}

pub fn main() {
    let args = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer()
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        .without_time();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();

    let output_dir = if let Some(dir) = args.output_dir {
        assert!(dir.is_dir(), "output_dir needs to be a directory");
        dir
    } else {
        std::env::current_dir().unwrap()
    };

    let source_file = File::open(&args.source).unwrap();
    let magic = source_file.read_at::<[u8; 4]>(&mut 0).unwrap();

    match &magic {
        b"RAKI" => {
            let filename = args.source.file_name().unwrap();
            let output_file_path = output_dir.join(filename).with_extension("");
            let mut output_file = File::create(&output_file_path).unwrap();
            let is_opus = decode_audio(&source_file, &mut output_file).unwrap();
            if is_opus {
                std::fs::rename(&output_file_path, output_file_path.with_extension("opus"))
                    .unwrap();
            }
        }
        _ => {
            let content = encode_audio(source_file).unwrap();
            let filename = args.source.file_name().unwrap();
            let output_file_path = output_dir.join(filename).with_extension("wav.ckd");
            let mut output_file = File::create(&output_file_path).unwrap();
            output_file.write_all(&content).unwrap();
        }
    }
}

/// Decode JD audio file
///
/// Returns true if the audio is opus encoded
pub fn decode_audio(reader: &File, writer: &mut File) -> Result<bool, Error> {
    let wav = Wav::deserialize(reader)?;

    let fmt = wav
        .chunks
        .get(&Fmt::MAGIC)
        .ok_or_else(|| anyhow!("No `fmt ` chunk!"))?
        .as_fmt()?;

    if !wav.chunks.contains_key(&Strg::MAGIC) && !wav.chunks.contains_key(&Mark::MAGIC) {
        trace!("No special chunks");
    }

    match (wav.platform, wav.codec) {
        (_, Codec::PCM) => {
            let writer = BufWriter::new(writer);
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
            let mut writer = hound::WavWriter::new(writer, spec)?;
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
            mux_to_opus(
                data.data.as_ref(),
                &mut position,
                writer,
                adin.num_of_samples,
            )?;
            Ok(true)
        }
        (WavPlatform::WiiU, Codec::Adpc) => {
            let writer = BufWriter::new(writer);
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

                let mut writer = hound::WavWriter::new(writer, spec)?;
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

                let mut writer = hound::WavWriter::new(writer, spec)?;
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

                let mut writer = hound::WavWriter::new(writer, spec)?;
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
pub fn encode_audio(file: File) -> Result<Vec<u8>, Error> {
    let magic = file.read_at::<[u8; 4]>(&mut 0)?;
    match &magic {
        b"OggS" => {
            let mut nx_opus = Vec::new();
            let (header, num_of_samples) = mux_from_opus(&file, &mut nx_opus, &mut 0)?;

            let channel_count = u16::from(header.channels);
            let fmt = Fmt {
                unk1: 99,
                channel_count,
                sample_rate: header.sample_rate,
                unk2: 96_000 * u32::from(channel_count),
                block_align: 2 * channel_count,
                bits_per_sample: 16,
                unk3: None,
            };

            let adin = AdIn { num_of_samples };

            let mut vec = Vec::new();
            wav::Writer::create_opus(&mut vec, &mut 0, fmt, adin, &nx_opus)?;

            Ok(vec)
        }
        b"RIFF" => {
            let mut vec = Vec::new();
            let decoder = hound::WavReader::new(file)?;
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
