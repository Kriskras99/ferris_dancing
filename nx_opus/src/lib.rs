use std::{borrow::Cow, collections::HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::{i16le, u16le, u32be, u32le, u64be, u64le},
        read::{BinaryDeserialize, BinaryDeserializeExt as _, ReadAtExt, ReadError},
        write::{BinarySerialize, BinarySerializeExt as _, WriteAt, WriteError},
        CursorAt,
    },
    test_eq, test_ge, test_le,
    testing::TestError,
};
use ogg::{
    OggReadError,
    PacketWriteEndInfo::{EndPage, EndStream, NormalPacket},
    PacketWriter,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
    #[error("Write error: {0}")]
    WriteError(#[from] WriteError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Integer conversion failed: {0}")]
    IntegerConversionFailed(#[from] std::num::TryFromIntError),
    #[error("Sanity check failed: {0}")]
    TestError(#[from] TestError),
    #[error("Failed to read opus file: {0}")]
    OpusReadError(#[from] OggReadError),
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

pub struct NxOpus {
    pub header: NxOpusHeader,
    pub num_of_samples: u32,
    pub data_offset: u64,
}

impl BinaryDeserialize<'_> for NxOpus {
    type Ctx = u32;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        num_of_samples: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let header = reader.read_at::<NxOpusHeader>(position)?;
        Ok(Self {
            header,
            num_of_samples,
            data_offset: *position,
        })
    }
}

impl NxOpus {
    pub fn mux_to_opus(
        source: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        destination: &mut (impl WriteAt + ?Sized),
        num_of_samples: u32,
    ) -> Result<(), Error> {
        let header = source.read_at::<NxOpusHeader>(position)?;
        let data_offset = *position;
        let data_type = source.read_at::<u32le>(position)?;
        test_eq!(data_type, 0x8000_0004)?;
        let data_size = source.read_at::<u32le>(position)?;
        let data_end = data_offset + u64::from(data_size);

        let mut writer = PacketWriter::new(CursorAt::new(destination, 0));
        let opus_header = OpusHeader {
            channels: header.channels,
            skip: i16::try_from(header.pre_skip)?,
            sample_rate: header.sample_rate,
            output_gain: 0,
        };
        let mut vec = Vec::new();
        OpusHeader::serialize(opus_header, &mut vec)?;
        writer.write_packet(vec, 0x0D15EA5E, EndPage, 0)?;

        let comments = OpusComments {
            vendor: Cow::Borrowed("UbiArt Toolkit"),
            comments: HashMap::new(),
        };
        let mut vec = Vec::new();
        OpusComments::serialize(comments, &mut vec)?;
        writer.write_packet(vec, 0x0D15EA5E, EndPage, 0)?;

        let mut total_samples = 0;
        let mut n = 0;
        while total_samples < num_of_samples {
            n += 1;
            let data_size = usize::try_from(source.read_at::<u32be>(position)?)?;
            let _unk2 = source.read_at::<u32be>(position)?; // opus state??

            let data = source.read_slice_at(position, data_size)?;

            let toc = OpusToc::deserialize(data.as_ref())?;
            let samples = u32::from(toc.frames_per_packet)
                * ((toc.frame_size * header.sample_rate) / 1_000_000);

            total_samples += samples;

            if total_samples >= num_of_samples {
                writer.write_packet(data, 0x0D15EA5E, EndStream, u64::from(num_of_samples))?;
            } else {
                writer.write_packet(data, 0x0D15EA5E, NormalPacket, u64::from(total_samples))?;
            };
        }

        if *position != data_end {
            println!("Position is not at data end!: position: {position}, data end: {data_end}");
        }

        if total_samples != num_of_samples {
            println!("Total samples do not match!: expected: {num_of_samples} read: {total_samples}, (packets: {n})");
        }

        Ok(())
    }

    /// Mux a normal opus to a NX opus
    ///
    /// Returns the total samples muxed
    pub fn mux_from_opus(
        reader: &(impl ReadAtExt + ?Sized),
        destination: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
    ) -> Result<(OpusHeader, u32), Error> {
        let mut ogg = ogg::PacketReader::new(CursorAt::new(reader, 0));
        let mut packet = ogg.read_packet_expected()?;
        let serial = packet.stream_serial();
        let mut data = Vec::new();
        // all header packets have a absgp of 0
        while packet.absgp_page() == 0 {
            test_eq!(
                serial,
                packet.stream_serial(),
                "More than one stream in ogg file!"
            )?;
            data.extend_from_slice(&packet.data);
            packet = ogg.read_packet_expected()?;
        }

        let mut read_position = 0;
        let header = OpusHeader::deserialize_at(&data, &mut read_position)?;
        let _comments = OpusComments::deserialize_at(&data, &mut read_position)?;

        destination.write_at::<NxOpusHeader>(
            position,
            NxOpusHeader {
                channels: header.channels,
                sample_rate: header.sample_rate,
                pre_skip: u32::try_from(header.skip)?,
            },
        )?;

        destination.write_at::<u32le>(position, 0x8000_0004)?; // data magic
        let mut position_data_size = *position;
        destination.write_at::<u32le>(position, 0)?; // data size

        let mut data_size = 0;
        let mut total_samples = 0;
        // process all audio packets
        loop {
            test_eq!(
                serial,
                packet.stream_serial(),
                "More than one stream in ogg file!"
            )?;
            let size = u32::try_from(packet.data.len())?;
            data_size += size;
            destination.write_at::<u32be>(position, size)?;
            destination.write_at::<u32be>(position, 0x1_000_000)?; // unk2
            let toc = packet.data.read_at::<OpusToc>(&mut 0)?;
            let samples = u32::from(toc.frames_per_packet)
                * ((toc.frame_size * header.sample_rate) / 1_000_000);
            total_samples += samples;

            destination.write_slice_at(position, &packet.data)?;
            match ogg.read_packet()? {
                Some(new) => packet = new,
                None => break,
            }
        }
        destination.write_at::<u32le>(&mut position_data_size, data_size)?; // data size

        Ok((header, total_samples))
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
        test_le!(
            input.channels,
            2,
            "Mapping file is required for more than 2 channels and is not supported"
        )?;
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
