use std::{borrow::Cow, collections::HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{i16le, u16le, u32be, u32le, u64be, u64le},
        read::{BinaryDeserialize, BinaryDeserializeExt, ReadAtExt, ReadError},
        write::{BinarySerialize, WriteAt, WriteError},
        CursorAt,
    },
    test_eq, test_ge, test_le,
};
use hound::SampleFormat;
use ogg::PacketReader;

use super::{Codec, Data, Fmt, WavPlatform};

pub struct WavCkdEncoder<S: ReadAtExt, D: WriteAt> {
    source: S,
    writer: D,
}

impl<S: ReadAtExt, D: WriteAt> WavCkdEncoder<S, D> {
    pub fn new(source: S, destination: D) -> Self {
        Self {
            source,
            writer: destination,
        }
    }

    pub fn encode(self, position: &mut u64) -> Result<(), WriteError> {
        let magic = self.source.read_at::<[u8; 4]>(&mut 0).unwrap();

        if magic == *b"OggS" {
            self.encode_opus(position)
        } else if magic == *b"RIFF" {
            self.encode_wav(position)
        } else {
            Err(WriteError::custom(format!(
                "Unknown source magic: {magic:?}"
            )))
        }
    }

    fn encode_wav(mut self, position: &mut u64) -> Result<(), WriteError> {
        let decoder = hound::WavReader::new(CursorAt::new(self.source, 0)).unwrap();
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
        };

        let header_size = 32; // header size without the chunks
        let chunk_header_size = 12 * 2; // size of the chunks without their data
        let total_header_size = header_size + chunk_header_size + Fmt::SIZE;
        let data_start = total_header_size.next_multiple_of(8);
        let data_size = decoder.duration() * u32::from(spec.channels) * 2; // every sample is two bytes

        // write the header
        self.writer.write_at::<&[u8; 4]>(position, b"RAKI")?;
        self.writer.write_at::<u32le>(position, 0xB)?;
        self.writer
            .write_at::<WavPlatform>(position, WavPlatform::Switch)?;
        self.writer.write_at::<Codec>(position, Codec::PCM)?;
        self.writer.write_at::<u32le>(position, total_header_size)?; // header size
        self.writer.write_at::<u32le>(position, data_start)?; // data start offset
        self.writer.write_at::<u32le>(position, 2)?; // number of chunks
        self.writer.write_at::<u32le>(position, 0)?; // unk2

        // write the chunks
        self.writer.write_at::<u32be>(position, Fmt::MAGIC)?;
        self.writer
            .write_at::<u32le>(position, header_size + chunk_header_size)?; // offset
        self.writer.write_at::<u32le>(position, Fmt::SIZE)?;
        self.writer.write_at::<u32be>(position, Data::MAGIC)?;
        self.writer.write_at::<u32le>(position, data_start)?;
        self.writer.write_at::<u32le>(position, data_size)?;
        self.writer.write_at::<Fmt>(position, fmt)?;

        // write filler between header and data
        for _ in 0..data_start - total_header_size {
            self.writer.write_at::<u8>(position, 0)?;
        }

        for sample in decoder.into_samples::<i16>() {
            self.writer.write_at::<i16le>(position, sample.unwrap())?;
        }

        Ok(())
    }

    fn encode_opus(self, position: &mut u64) -> Result<(), WriteError> {
        let mut ogg = PacketReader::new(CursorAt::new(self.source, 0));

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

        let mut src_position = 0;
        let header = OpusHeader::deserialize_at(&data, &mut src_position).unwrap();
        let _comments = OpusComments::deserialize_at(&data, &mut src_position).unwrap();

        todo!()
        // let fmt = Fmt {
        //     unk1: 1,
        //     channel_count: spec.channels,
        //     sample_rate: spec.sample_rate,
        //     unk2: 192_000,
        //     block_align: 4,
        //     bits_per_sample: 16,
        // };

        // let header_size = 32; // header size without the chunks
        // let chunk_header_size = 12 * 3; // size of the chunks without their data
        // let total_header_size = header_size + chunk_header_size + Fmt::SIZE + AdIn::SIZE;
        // let data_start = total_header_size.next_multiple_of(8);
        // let data_size = decoder.duration() * u32::from(spec.channels) * 2; // every sample is two bytes

        // // write the header
        // self.writer.write_at::<&[u8; 4]>(position, b"RAKI")?;
        // self.writer.write_at::<u32le>(position, 0xB)?;
        // self.writer.write_at::<WavPlatform>(position, WavPlatform::Switch)?;
        // self.writer.write_at::<Codec>(position, Codec::PCM)?;
        // self.writer.write_at::<u32le>(position, total_header_size)?; // header size
        // self.writer.write_at::<u32le>(position, data_start)?; // data start offset
        // self.writer.write_at::<u32le>(position, 2)?; // number of chunks
        // self.writer.write_at::<u32le>(position, 0)?; // unk2

        // // write the chunks
        // self.writer.write_at::<u32be>(position, Fmt::MAGIC)?;
        // self.writer.write_at::<u32le>(position, header_size + chunk_header_size)?; // offset
        // self.writer.write_at::<u32le>(position, Fmt::SIZE)?;
        // self.writer.write_at::<u32be>(position, Data::MAGIC)?;
        // self.writer.write_at::<u32le>(position, data_start)?;
        // self.writer.write_at::<u32le>(position, data_size)?;
        // self.writer.write_at::<Fmt>(position, fmt)?;

        // // write filler between header and data
        // for _ in 0..data_start - total_header_size {
        //     self.writer.write_at::<u8>(position, 0)?;
        // }
        // Ok(())
    }
}

impl BinarySerialize for WavPlatform {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))
    }
}

impl BinarySerialize for Codec {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))
    }
}

impl BinarySerialize for Fmt {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        fmt: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u16le>(position, fmt.unk1)?;
        writer.write_at::<u16le>(position, fmt.channel_count)?;
        writer.write_at::<u32le>(position, fmt.sample_rate)?;
        writer.write_at::<u32le>(position, fmt.unk2)?;
        writer.write_at::<u16le>(position, fmt.block_align)?;
        writer.write_at::<u16le>(position, fmt.bits_per_sample)?;
        Ok(())
    }
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
                .find('=')
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
