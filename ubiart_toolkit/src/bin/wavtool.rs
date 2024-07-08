#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use dotstar_toolkit_utils::bytes::{
    primitives::{i16le, u16le, u32le, u64le},
    read::{BinaryDeserializeExt as _, ReadAt, ReadAtExt},
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

    let path = cli.source.with_extension("wav");
    println!("{wav:#?}");

    if wav.codec == Codec::PCM {
        decode_pcm(&file, &wav, &path);
    } else if wav.codec == Codec::Nx {
        decode_opus(&file, &wav);
    } else if wav.codec == Codec::Adpc && wav.platform == WavPlatform::WiiU {
        decode_gc_dsp(&file, &wav, &path);
    }
}

fn decode_opus(file: &File, wav: &Wav) {
    let fmt = wav.chunks[&Fmt::MAGIC].as_fmt().unwrap();
    let data = wav.chunks[&Data::MAGIC].as_data().unwrap();
    let adin = wav.chunks[&AdIn::MAGIC].as_adin().unwrap();

    let num_of_samples = adin.num_of_samples;

    let channels = match fmt.channel_count {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => panic!("Too many channels"),
    };
    let mut decoder = opus::Decoder::new(fmt.sample_rate, channels).unwrap();

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

    let mut slice_position = data.position + u64::from(data_offset + 0x8);
    println!("0x{slice_position:x}");
    let data = file
        .read_slice_at(&mut slice_position, usize::try_from(data_size).unwrap())
        .unwrap();
    let mut new_data = vec![0; usize::try_from(num_of_samples).unwrap()];
    decoder.decode(&data, &mut new_data, false).unwrap();
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

    // let data_left = wav.chunks.get(&Data::MAGIC_LEFT).unwrap().as_data().unwrap();
    // let dsp_left = wav.chunks.get(&Dsp::MAGIC_LEFT).unwrap().as_dsp().unwrap();

    // let spec = hound::WavSpec {
    //     channels: 1,
    //     sample_rate: fmt.sample_rate,
    //     bits_per_sample: fmt.bits_per_sample,
    //     sample_format: hound::SampleFormat::Int,
    // };
    // let mut writer = hound::WavWriter::create(path, spec).unwrap();

    // let mut position = data_left.position;

    // let mut decoder = gc_adpcm::Decoder {
    //     hist1: dsp_left.initial_sample_history_1,
    //     hist2: dsp_left.initial_sample_history_2,
    //     coefficients: dsp_left.coefficients,
    // };

    // // let mut hist1 = dsp_left.initial_sample_history_1 as i32;
    // // let mut hist2 = dsp_left.initial_sample_history_2 as i32;
    // let frame_count = dsp_left.sample_count.div_ceil(SAMPLES_PER_FRAME);

    // for _ in 0..frame_count {
    //     let samples = decoder.decode_frame(file.read_at::<[u8; 8]>(&mut position).unwrap());

    //     for sample in samples {
    //         writer.write_sample(sample).unwrap();
    //     }

    //     // let header = file.read_at::<u8>(&mut position).unwrap();

    //     // let scale = (1u16 << (header & 0xF)) as i32;
    //     // let coef_index = usize::from((header >> 4) & 0xf);
    //     // let coef1 = dsp_left.coefficients[coef_index*2] as i32;
    //     // let coef2 = dsp_left.coefficients[coef_index*2 + 1] as i32;

    //     // // 7 data bytes per frame
    //     // for _ in 0..7 {
    //     //     let byte = file.read_at::<u8>(&mut position).unwrap();

    //     //     // 2 samples per byte
    //     //     for s in 0..2 {
    //     //         let mut sample = if s == 0 { get_high_nibble(byte) } else { get_low_nibble(byte) };
    //     //         sample = if sample >= 8 { sample - 16 } else { sample };
    //     //         sample = (((scale * sample) << 11) + 1024 + (coef1 * hist1 + coef2 * hist2)) >> 11;
    //     //         sample = clamp(sample);
    //     //         writer.write_sample(sample).unwrap();

    //     //         hist2 = hist1;
    //     //         hist1 = sample;
    //     //     }
    //     // }

    // }
}

// const NIBBLE_TO_S8: [i32; 0x10] = [0,1,2,3,4,5,6,7,-8,-7,-6,-5,-4,-3,-2,-1];
// fn get_low_nibble(byte: u8) -> i32 {
//     NIBBLE_TO_S8[usize::from(byte & 0xF)]
// }
// fn get_high_nibble(byte: u8) -> i32 {
//     NIBBLE_TO_S8[usize::from((byte >> 4) & 0xF)]
// }
// fn clamp(val: i32) -> i32 {
//     val.clamp(-32768, 32767) as i32
// }
