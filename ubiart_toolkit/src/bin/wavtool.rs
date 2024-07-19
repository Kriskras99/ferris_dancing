#![allow(clippy::missing_panics_doc, reason = "Tool not a library")]

use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use clap::Parser;
use dotstar_toolkit_utils::{
    bytes::{
        primitives::{i16le, u16le, u32be, u32le, u64be, u64le},
        read::{BinaryDeserialize, BinaryDeserializeExt as _, ReadAt, ReadAtExt, ReadError},
        write::{BinarySerialize, BinarySerializeExt, WriteAt, WriteError},
        CursorAt,
    },
    test_eq, test_ge, test_le,
};
use ogg::{
    PacketWriteEndInfo::{EndPage, EndStream, NormalPacket},
    PacketWriter,
};
use ubiart_toolkit::cooked::wav::{AdIn, Codec, Data, Dsp, Fmt, Wav, WavCkdEncoder, WavPlatform};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    source: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    // println!("{:?}", cli.source);

    let file = File::open(&cli.source).unwrap();
    let magic = file.read_at::<[u8; 4]>(&mut 0).unwrap();

    assert_eq!(magic, *b"HAHA");

    match &magic {
        b"RAKI" => {
            let wav = Wav::deserialize(&file).unwrap();
            println!(
                "Header size: 0x{:x}, start offset: 0x{:x}",
                wav.header_size, wav.data_start_offset
            );
            let mut path = cli.source.with_extension("wav");
            println!("{wav:#?}");
            // println!("Decoding {:?} to {path:?}", cli.source);
            if wav.codec == Codec::PCM {
                decode_pcm(&file, &wav, &path);
            } else if wav.codec == Codec::Nx {
                path.set_extension("opus");
                decode_opus(&file, &wav, &path);
            } else if wav.codec == Codec::Adpc && wav.platform == WavPlatform::WiiU {
                decode_gc_dsp(&file, &wav, &path);
            } else {
                panic!("Unsupported codec/platform combination!")
            }
        }
        b"OggS" | b"RIFF" => {
            let path = cli.source.with_extension("wav.ckd");
            let destination = File::create(path).unwrap();
            let encoder = WavCkdEncoder::new(file, destination);
            encoder.encode(&mut 0).unwrap();
        }
        _ => {
            panic!("Unknown file, expecting .wav.ckd, .opus, or .wav");
        }
    }
}

fn decode_opus(file: &File, wav: &Wav, path: &Path) {
    let mut dest = File::create(path).unwrap();
    decode_nx_opus_inner(file, wav, &mut dest);
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

    assert_eq!(
        fmt.bits_per_sample, 16,
        "Bits per sample != 16, this is not supported"
    );

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

fn decode_nx_opus_inner(src: &File, wav: &Wav, destination: &mut (impl WriteAt + ?Sized)) {
    let fmt = wav.chunks[&Fmt::MAGIC].as_fmt().unwrap();
    let data = wav.chunks[&Data::MAGIC].as_data().unwrap();
    let adin = wav.chunks[&AdIn::MAGIC].as_adin().unwrap();

    let mut position = data.position;

    let nx_header = NxOpusHeader::deserialize_at(src, &mut position).unwrap();

    assert_eq!(
        fmt.channel_count,
        u16::from(nx_header.channels),
        "Channel count does not match!"
    );
    assert_eq!(
        fmt.sample_rate, nx_header.sample_rate,
        "Sample rate does not match!"
    );

    let data_start = position;
    let data_type = src.read_at::<u32le>(&mut position).unwrap();
    test_eq!(data_type, 0x80000004).unwrap();
    let data_size = src.read_at::<u32le>(&mut position).unwrap();
    let data_end = data_start + u64::from(data_size);

    let mut writer = PacketWriter::new(CursorAt::new(destination, 0));
    let header = OpusHeader {
        channels: nx_header.channels,
        skip: i16::try_from(nx_header.pre_skip).unwrap(),
        sample_rate: nx_header.sample_rate,
        output_gain: 0,
    };
    let mut vec = Vec::new();
    OpusHeader::serialize(header, &mut vec).unwrap();
    writer.write_packet(vec, 0x0D15EA5E, EndPage, 0).unwrap();

    let comments = OpusComments {
        vendor: Cow::Borrowed("UbiArt Toolkit"),
        comments: HashMap::new(),
    };
    let mut vec = Vec::new();
    OpusComments::serialize(comments, &mut vec).unwrap();
    writer.write_packet(vec, 0x0D15EA5E, EndPage, 0).unwrap();

    let mut total_samples = 0;
    let mut n = 0;
    while total_samples < adin.num_of_samples {
        n += 1;
        let data_size = usize::try_from(src.read_at::<u32be>(&mut position).unwrap()).unwrap();
        let _unk2 = src.read_at::<u32be>(&mut position).unwrap(); // opus state??

        let data = src.read_slice_at(&mut position, data_size).unwrap();

        let toc = OpusToc::deserialize(data.as_ref()).unwrap();
        let samples = u32::from(toc.frames_per_packet)
            * ((toc.frame_size * nx_header.sample_rate) / 1_000_000);

        total_samples += samples;

        if total_samples >= adin.num_of_samples {
            writer
                .write_packet(data, 0x0D15EA5E, EndStream, u64::from(adin.num_of_samples))
                .unwrap();
        } else {
            writer
                .write_packet(data, 0x0D15EA5E, NormalPacket, u64::from(total_samples))
                .unwrap();
        };
    }

    if position != data_end {
        println!("Position is not at data end!: position: {position}, data end: {data_end}");
    }

    if total_samples != adin.num_of_samples {
        println!(
            "Total samples do not match!: expected: {} read: {total_samples}, (packets: {n})",
            adin.num_of_samples
        );
    }
}

fn encode_opus(file: File, destination: &mut (impl WriteAt + ?Sized)) {
    let mut ogg = ogg::PacketReader::new(file);

    let mut packet = ogg.read_packet_expected().unwrap();
    let serial = packet.stream_serial();
    let mut data = Vec::new();
    // all header packets have a absgp of 0
    while packet.absgp_page() == 0 {
        test_eq!(serial, packet.stream_serial(), "More than one stream in ogg file!")
            .unwrap();
        data.extend_from_slice(&packet.data);
        packet = ogg.read_packet_expected().unwrap();
    }

    let mut position = 0;
    let header = OpusHeader::deserialize_at(&data, &mut position).unwrap();
    let _comments = OpusComments::deserialize_at(&data, &mut position).unwrap();

    let mut position = 0;
    destination
        .write_at::<NxOpusHeader>(
            &mut position,
            NxOpusHeader {
                channels: header.channels,
                sample_rate: header.sample_rate,
                pre_skip: u32::try_from(header.skip).unwrap(),
            },
        )
        .unwrap();

    destination
        .write_at::<u32le>(&mut position, 0x80000004)
        .unwrap(); // data magic
    let mut position_data_size = position;
    destination.write_at::<u32le>(&mut position, 0).unwrap(); // data size

    let mut data_size = 0;
    // process all audio packets
    loop {
        test_eq!(serial, packet.stream_serial(), "More than one stream in ogg file!")
            .unwrap();
        let size = u32::try_from(packet.data.len()).unwrap();
        data_size += size;
        destination.write_at::<u32be>(&mut position, size).unwrap();
        destination
            .write_at::<u32be>(&mut position, 0x1000000)
            .unwrap(); // unk2
        destination
            .write_slice_at(&mut position, &packet.data)
            .unwrap();
        match ogg.read_packet().unwrap() {
            Some(new) => packet = new,
            None => break,
        }
    }
    destination
        .write_at::<u32le>(&mut position_data_size, data_size)
        .unwrap(); // data size
}

#[derive(Debug)]
pub struct OpusHeader {
    pub channels: u8,
    pub skip: i16,
    pub sample_rate: u32,
    pub output_gain: u16,
}

impl BinaryDeserialize<'_> for OpusHeader {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u64be>(position)?;
        test_eq!(magic, u64::from_be_bytes(*b"OpusHead"))?;
        let version = reader.read_at::<u8>(position)?;
        test_eq!(version, 1)?;
        let channels = reader.read_at::<u8>(position)?;
        let skip = reader.read_at::<i16le>(position)?;
        let sample_rate = reader.read_at::<u32le>(position)?;
        let output_gain = reader.read_at::<u16le>(position)?;
        let mapping_file = reader.read_at::<u8>(position)?;
        test_eq!(mapping_file, 0, "Mapping file is not yet supported!")?;

        Ok(Self {
            channels,
            skip,
            sample_rate,
            output_gain,
        })
    }
}

impl BinarySerialize for OpusHeader {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        test_le!(input.channels, 2, "Mapping file is required for more than 2 channels and is not supported")?;
        writer.write_at::<u64be>(position, u64::from_be_bytes(*b"OpusHead"))?;
        writer.write_at::<u8>(position, 1)?; // version

        writer.write_at::<u8>(position, input.channels)?;
        writer.write_at::<i16le>(position, input.skip)?;
        writer.write_at::<u32le>(position, input.sample_rate)?;
        writer.write_at::<u16le>(position, input.output_gain)?; // output gain
        writer.write_at::<u8>(position, 0)?; // mapping file
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpusComments<'a> {
    pub vendor: Cow<'a, str>,
    pub comments: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'de> BinaryDeserialize<'de> for OpusComments<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u64be>(position)?;
        test_eq!(magic, u64::from_be_bytes(*b"OpusTags"))?;
        let vendor = reader.read_len_string_at::<u32le>(position)?;
        let comment_list_length = reader.read_at::<u32le>(position)?;
        let mut comments = HashMap::with_capacity(usize::try_from(comment_list_length)?);
        for _ in 0..comment_list_length {
            let comment = reader.read_len_string_at::<u32le>(position)?;
            let index = comment
                .find("=")
                .ok_or_else(|| ReadError::custom(format!("Invalid comment: {comment}")))?;
            test_ge!(comment.len(), index + 2)?;
            let (key, value) = match comment {
                Cow::Borrowed(comment) => {
                    let (left, right) = comment.split_at(index);
                    let right = &right[1..];
                    (Cow::Borrowed(left), Cow::Borrowed(right))
                }
                Cow::Owned(mut comment) => {
                    let right = comment.split_at(index + 1).1;
                    let right = right.to_string();
                    comment.truncate(index);
                    comment.shrink_to_fit();
                    (Cow::Owned(comment), Cow::Owned(right))
                }
            };
            comments.insert(key, value);
        }
        Ok(Self { vendor, comments })
    }
}

impl BinarySerialize for OpusComments<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u64be>(position, u64::from_be_bytes(*b"OpusTags"))?;
        writer.write_len_string_at::<u32le>(position, &input.vendor)?;
        writer.write_at::<u32le>(position, u32::try_from(input.comments.len())?)?;
        for (key, value) in input.comments {
            test_eq!(key.contains('='), false, "Comment key cannot contain a '='")?;
            let length = key.len() + value.len() + 1;
            writer.write_at::<u32le>(position, u32::try_from(length)?)?;
            writer.write_slice_at(position, key.as_bytes())?;
            writer.write_at::<u8>(position, b'=')?;
            writer.write_slice_at(position, value.as_bytes())?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct OpusToc {
    pub mode: Mode,
    pub bandwith: Bandwith,
    pub frame_size: u32,
    pub stereo: bool,
    pub frames_per_packet: u8,
}

#[derive(Debug)]
pub enum Mode {
    Silk,
    Hybrid,
    Celt,
}

#[derive(Debug)]
pub enum Bandwith {
    Narrow,
    Medium,
    Wide,
    Superwide,
    Full,
}

impl BinaryDeserialize<'_> for OpusToc {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let byte = reader.read_at::<u8>(position)?;
        let (mode, bandwith, frame_size) = match byte >> 3 {
            0 => (Mode::Silk, Bandwith::Narrow, 10_000u32),
            1 => (Mode::Silk, Bandwith::Narrow, 20_000u32),
            2 => (Mode::Silk, Bandwith::Narrow, 40_000u32),
            3 => (Mode::Silk, Bandwith::Narrow, 60_000u32),
            4 => (Mode::Silk, Bandwith::Medium, 10_000u32),
            5 => (Mode::Silk, Bandwith::Medium, 20_000u32),
            6 => (Mode::Silk, Bandwith::Medium, 40_000u32),
            7 => (Mode::Silk, Bandwith::Medium, 60_000u32),
            8 => (Mode::Silk, Bandwith::Wide, 10_000u32),
            9 => (Mode::Silk, Bandwith::Wide, 20_000u32),
            10 => (Mode::Silk, Bandwith::Wide, 40_000u32),
            11 => (Mode::Silk, Bandwith::Wide, 60_000u32),
            12 => (Mode::Hybrid, Bandwith::Superwide, 10_000u32),
            13 => (Mode::Hybrid, Bandwith::Superwide, 20_000u32),
            14 => (Mode::Hybrid, Bandwith::Full, 10_000u32),
            15 => (Mode::Hybrid, Bandwith::Full, 20_000u32),
            16 => (Mode::Celt, Bandwith::Narrow, 2_500u32),
            17 => (Mode::Celt, Bandwith::Narrow, 5_000u32),
            18 => (Mode::Celt, Bandwith::Narrow, 10_000u32),
            19 => (Mode::Celt, Bandwith::Narrow, 20_000u32),
            20 => (Mode::Celt, Bandwith::Wide, 2_500u32),
            21 => (Mode::Celt, Bandwith::Wide, 5_000u32),
            22 => (Mode::Celt, Bandwith::Wide, 10_000u32),
            23 => (Mode::Celt, Bandwith::Wide, 20_000u32),
            24 => (Mode::Celt, Bandwith::Superwide, 2_500u32),
            25 => (Mode::Celt, Bandwith::Superwide, 5_000u32),
            26 => (Mode::Celt, Bandwith::Superwide, 10_000u32),
            27 => (Mode::Celt, Bandwith::Superwide, 20_000u32),
            28 => (Mode::Celt, Bandwith::Full, 2_500u32),
            29 => (Mode::Celt, Bandwith::Full, 5_000u32),
            30 => (Mode::Celt, Bandwith::Full, 10_000u32),
            31 => (Mode::Celt, Bandwith::Full, 20_000u32),
            _ => unreachable!(),
        };
        let stereo = (byte & 0b100) == 0b100;
        let frames_per_packet = match byte & 0b11 {
            0 => 1u8,
            1 | 2 => 2,
            3 => reader.read_at::<u8>(position)? & 0x3F,
            _ => unreachable!(),
        };

        Ok(Self {
            mode,
            bandwith,
            frame_size,
            stereo,
            frames_per_packet,
        })
    }
}

pub struct NxOpusHeader {
    pub channels: u8,
    pub sample_rate: u32,
    pub pre_skip: u32,
}

impl NxOpusHeader {
    const MAGIC: u32 = 0x8000_0001;
}

impl BinaryDeserialize<'_> for NxOpusHeader {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let the_type = reader.read_at::<u32le>(position)?;
        test_eq!(the_type, Self::MAGIC)?;
        let header_size = reader.read_at::<u32le>(position)?;
        test_eq!(header_size, 24)?; // header size excludes itself and the type
        let version = reader.read_at::<u8>(position)?;
        test_eq!(version, 0)?;
        let channels = reader.read_at::<u8>(position)?;
        let frame_size = reader.read_at::<u16le>(position)?;
        test_eq!(frame_size, 0)?;
        let sample_rate = reader.read_at::<u32le>(position)?;
        let data_offset = reader.read_at::<u32le>(position)?;
        test_eq!(data_offset, 32)?; // from the start of the data block
        let unk1 = reader.read_at::<u64le>(position)?;
        test_eq!(unk1, 0)?;
        let pre_skip = reader.read_at::<u32le>(position)?;

        Ok(Self {
            channels,
            sample_rate,
            pre_skip,
        })
    }
}

impl BinarySerialize for NxOpusHeader {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32le>(position, Self::MAGIC)?;
        writer.write_at::<u32le>(position, 0x18)?; // header size
        writer.write_at::<u8>(position, 0)?; // version
        writer.write_at::<u8>(position, input.channels)?;
        writer.write_at::<u16le>(position, 0)?; // frame size?
        writer.write_at::<u32le>(position, input.sample_rate)?;
        writer.write_at::<u32le>(position, 0x20)?; // data offset
        writer.write_at::<u64le>(position, 0)?; // unk1
        writer.write_at::<u32le>(position, input.pre_skip)?;

        Ok(())
    }
}
