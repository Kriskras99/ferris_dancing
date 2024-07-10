#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{
    fs::File,
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use clap::Parser;
use dotstar_toolkit_utils::bytes::{
    primitives::{i16le, u16le, u32be, u32le, u64le},
    read::{BinaryDeserializeExt as _, ReadAt, ReadAtExt},
    write::WriteAt,
};
use ogg::{
    PacketWriteEndInfo::{EndPage, NormalPacket},
    PacketWriter,
};
use ubiart_toolkit::cooked::wav::{AdIn, Codec, Data, Dsp, Fmt, Wav, WavPlatform};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // println!("{:?}", cli.source);

    let file = File::open(&cli.source).unwrap();
    let wav = Wav::deserialize(&file).unwrap();

    let mut path = cli.source.with_extension("wav");
    println!("{wav:#?}");

    if wav.codec == Codec::PCM {
        decode_pcm(&file, &wav, &path);
    } else if wav.codec == Codec::Nx {
        path.set_extension("opus");
        decode_opus(&file, &wav, &path);
    } else if wav.codec == Codec::Adpc && wav.platform == WavPlatform::WiiU {
        decode_gc_dsp(&file, &wav, &path);
    }
}

fn decode_opus(file: &File, wav: &Wav, path: &Path) {
    let mut dest = File::create(path).unwrap();
    let ogg = convert_opus_formats(file, wav);
    dest.write_all(&ogg).unwrap();
}

fn decode_pcm(file: &File, wav: &Wav, path: &Path) {
    let fmt = wav.chunks[&Fmt::MAGIC].as_fmt().unwrap();
    let data = wav.chunks[&Data::MAGIC].as_data().unwrap();

    let spec = hound::WavSpec {
        channels: fmt.channel_count,
        sample_rate: fmt.sample_rate,
        bits_per_sample: fmt.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();

    if fmt.bits_per_sample != 16 {
        todo!();
    }

    let mut position = data.position;
    for _ in 0..(data.size / 2) {
        let sample = file.read_at::<i16le>(&mut position).unwrap();
        writer.write_sample(sample).unwrap();
    }
}

const SAMPLES_PER_FRAME: u32 = 14;
fn decode_gc_dsp(file: &File, wav: &Wav, path: &Path) {
    let fmt = wav.chunks[&Fmt::MAGIC].as_fmt().unwrap();

    let spec = hound::WavSpec {
        channels: fmt.channel_count,
        sample_rate: fmt.sample_rate,
        bits_per_sample: fmt.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();

    if let Some(data) = wav.chunks.get(&Data::MAGIC_STEREO) {
        let data = data.as_data().unwrap();
        let dsp_left = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp().unwrap();
        let dsp_right = wav.chunks[&Dsp::MAGIC_RIGHT].as_dsp().unwrap();
        // interleaved per frame

        let mut position = data.position;
        let mut decoder_left = gc_adpcm::Decoder {
            hist1: dsp_left.initial_sample_history_1,
            hist2: dsp_left.initial_sample_history_2,
            coefficients: dsp_left.coefficients,
        };
        let mut decoder_right = gc_adpcm::Decoder {
            hist1: dsp_left.initial_sample_history_1,
            hist2: dsp_left.initial_sample_history_2,
            coefficients: dsp_left.coefficients,
        };
        let frame_count = dsp_left.sample_count.div_ceil(SAMPLES_PER_FRAME);
        assert_eq!(
            dsp_left.sample_count, dsp_right.sample_count,
            "One channel has more samples than the other"
        );

        for _ in 0..frame_count {
            let samples_left =
                decoder_left.decode_frame(file.read_at::<[u8; 8]>(&mut position).unwrap());
            let samples_right =
                decoder_right.decode_frame(file.read_at::<[u8; 8]>(&mut position).unwrap());

            for i in 0..14 {
                writer.write_sample(samples_left[i]).unwrap();
                writer.write_sample(samples_right[i]).unwrap();
            }
        }
    } else if let Some(data_right) = wav.chunks.get(&Data::MAGIC_RIGHT) {
        let data_right = data_right.as_data().unwrap();
        let data_left = wav.chunks[&Data::MAGIC_LEFT].as_data().unwrap();
        let dsp_right = wav.chunks[&Dsp::MAGIC_RIGHT].as_dsp().unwrap();
        let dsp_left = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp().unwrap();

        let mut position_left = data_left.position;
        let mut decoder_left = gc_adpcm::Decoder {
            hist1: dsp_left.initial_sample_history_1,
            hist2: dsp_left.initial_sample_history_2,
            coefficients: dsp_left.coefficients,
        };
        let mut position_right = data_right.position;
        let mut decoder_right = gc_adpcm::Decoder {
            hist1: dsp_right.initial_sample_history_1,
            hist2: dsp_right.initial_sample_history_2,
            coefficients: dsp_right.coefficients,
        };
        assert_eq!(
            dsp_left.sample_count, dsp_right.sample_count,
            "One channel has more samples than the other"
        );
        let frame_count = dsp_left.sample_count.div_ceil(SAMPLES_PER_FRAME);

        for _ in 0..frame_count {
            let samples_left =
                decoder_left.decode_frame(file.read_at::<[u8; 8]>(&mut position_left).unwrap());
            let samples_right =
                decoder_right.decode_frame(file.read_at::<[u8; 8]>(&mut position_right).unwrap());

            for i in 0..14 {
                writer.write_sample(samples_left[i]).unwrap();
                writer.write_sample(samples_right[i]).unwrap();
            }
        }

        // non interleaved stereo
    } else if let Some(data) = wav.chunks.get(&Data::MAGIC_LEFT) {
        let data = data.as_data().unwrap();
        let dsp = wav.chunks[&Dsp::MAGIC_LEFT].as_dsp().unwrap();

        let mut position = data.position;

        let mut decoder = gc_adpcm::Decoder {
            hist1: dsp.initial_sample_history_1,
            hist2: dsp.initial_sample_history_2,
            coefficients: dsp.coefficients,
        };
        let frame_count = dsp.sample_count.div_ceil(SAMPLES_PER_FRAME);

        for _ in 0..frame_count {
            let samples = decoder.decode_frame(file.read_at::<[u8; 8]>(&mut position).unwrap());

            for sample in samples {
                writer.write_sample(sample).unwrap();
            }
        }
    } else {
        panic!("No DATA!");
    }
}

fn convert_opus_formats(file: &File, wav: &Wav) -> Vec<u8> {
    let fmt = wav.chunks[&Fmt::MAGIC].as_fmt().unwrap();
    let data = wav.chunks[&Data::MAGIC].as_data().unwrap();
    let adin = wav.chunks[&AdIn::MAGIC].as_adin().unwrap();

    let num_of_samples = adin.num_of_samples;

    // Apparently `data` contains a normal switch opus header plus the data
    let mut position = data.position;
    println!("0x{position:x}");
    let the_type = file.read_at::<u32le>(&mut position).unwrap();
    let header_size = file.read_at::<u32le>(&mut position).unwrap();
    let version = file.read_at::<u8>(&mut position).unwrap();
    let channel_count = file.read_at::<u8>(&mut position).unwrap();
    let frame_size = file.read_at::<u16le>(&mut position).unwrap();
    let sample_rate = file.read_at::<u32le>(&mut position).unwrap();
    let data_offset = file.read_at::<u32le>(&mut position).unwrap();
    let unk1 = file.read_at::<u64le>(&mut position).unwrap();
    let pre_skip = file.read_at::<u32le>(&mut position).unwrap();

    assert_eq!(
        fmt.channel_count,
        u16::from(channel_count),
        "Channel count does not match!"
    );
    assert_eq!(fmt.sample_rate, sample_rate, "Sample rate does not match!");

    position = data.position + u64::from(data_offset);
    let data_type = file.read_at::<u32le>(&mut position).unwrap();
    let data_size = file.read_at::<u32le>(&mut position).unwrap();

    println!("Type: 0x{the_type:x}");
    println!("Header size: {header_size}");
    println!("Version: {version}");
    println!("Channel count: {channel_count}");
    println!("Frame size: {frame_size}");
    println!("Sample rate: {sample_rate}");
    println!("Data offset: {data_offset}");
    println!("Unk1: {unk1}");
    println!("Pre skip: {pre_skip}");
    println!("Data type: 0x{data_type:x}");
    println!("Data size: {data_size}");

    let mut writer = PacketWriter::new(Cursor::new(Vec::new()));
    let header = make_opus_header(
        channel_count,
        1,
        i16::try_from(pre_skip).unwrap(),
        sample_rate,
    );
    writer.write_packet(header, 0x1234, EndPage, 0).unwrap();
    let comment = make_opus_comment();
    writer.write_packet(comment, 0x1234, EndPage, 0).unwrap();

    let mut total_samples = 0;
    while position < data.position + u64::from(data.size) {
        let data_size = usize::try_from(file.read_at::<u32be>(&mut position).unwrap()).unwrap();
        let _unk2 = file.read_at::<u32be>(&mut position).unwrap();

        let data = file.read_slice_at(&mut position, data_size).unwrap();
        let samples = opus_get_packet_samples([data[0], data[1]], sample_rate);
        total_samples += samples;
        writer
            .write_packet(data, 0x1234, NormalPacket, u64::from(total_samples))
            .unwrap();
    }

    if total_samples != num_of_samples {
        println!("Total samples do not match!: {num_of_samples} {total_samples}");
    }

    writer.into_inner().into_inner()
}

fn opus_get_packet_samples(data: [u8; 2], fs: u32) -> u32 {
    opus_packet_get_nb_frames(data) * opus_packet_get_samples_per_frame(data[0], fs)
}

fn opus_packet_get_samples_per_frame(data: u8, fs: u32) -> u32 {
    if data & 0x80 == 0x80 {
        let audiosize = u32::from((data >> 3) & 0x3);
        (fs << audiosize) / 400
    } else if data & 0x60 == 0x60 {
        if (data & 0x8) == 0x8 {
            fs / 50
        } else {
            fs / 100
        }
    } else {
        let audiosize = u32::from((data >> 3) & 0x3);
        if audiosize == 3 {
            (fs * 60) / 1000
        } else {
            (fs << audiosize) / 100
        }
    }
}

fn opus_packet_get_nb_frames(packet: [u8; 2]) -> u32 {
    let count = packet[0] & 0x3;
    if count == 0 {
        1
    } else if count != 3 {
        2
    } else {
        u32::from(packet[1] & 0x3F)
    }
}

fn make_opus_header(channels: u8, stream_count: usize, skip: i16, sample_rate: u32) -> Vec<u8> {
    assert!(
        !(channels > 2 || stream_count > 1),
        "More than 2 channels or multiple streams is not supported!"
    );

    let mut vec = Vec::new();
    let mut position = 0;

    vec.write_at::<u32be>(&mut position, u32::from_be_bytes(*b"Opus"))
        .unwrap();
    vec.write_at::<u32be>(&mut position, u32::from_be_bytes(*b"Head"))
        .unwrap();
    vec.write_at::<u8>(&mut position, 1).unwrap(); // version
    vec.write_at::<u8>(&mut position, channels).unwrap();
    vec.write_at::<i16le>(&mut position, skip).unwrap();
    vec.write_at::<u32le>(&mut position, sample_rate).unwrap();
    vec.write_at::<u16le>(&mut position, 0).unwrap(); // output gain
    vec.write_at::<u8>(&mut position, 0).unwrap(); // mapping file

    vec
}

fn make_opus_comment() -> Vec<u8> {
    let mut vec = Vec::new();
    let mut position = 0;

    vec.write_at::<u32be>(&mut position, u32::from_be_bytes(*b"Opus"))
        .unwrap();
    vec.write_at::<u32be>(&mut position, u32::from_be_bytes(*b"Tags"))
        .unwrap();
    vec.write_len_string_at::<u32le>(&mut position, "UbiArt Toolkit")
        .unwrap();
    vec.write_at::<u32le>(&mut position, 1).unwrap(); // user comment list length
    vec.write_len_string_at::<u32le>(&mut position, "UbiArt Toolkit Opus converter")
        .unwrap();

    vec
}
