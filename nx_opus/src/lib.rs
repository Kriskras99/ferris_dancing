//! This crate provides two functions, [`mux_to_opus`] and [`mux_from_opus`],
//! for muxing between the Ogg/Opus file format and the Nintendo Switch Opus file format.
//! This is achieved without re-encoding the audio, and therefore there is no quality loss.
//!
//! # Nintendo Switch Opus file format
//! The file format is described below. The offsets are relative to the start of the file.
//! However, the file is often embedded in engine specific file formats. In these cases the offsets
//! are relative to the start of the data block where the file is embedded.
//!
//! ## Header
//! | Offset  | Type   | Name         | Explanation |
//! | ------- | ------ | ------------ | ----------- |
//! | 0x00    | u32le  | Magic        | 0x8000_0001 |
//! | 0x04    | u32le  | Header size  | 24, excludes itself and the magic value |
//! | 0x08    | u8     | Version      | 0  |
//! | 0x09    | u8     | Channels     | |
//! | 0x0A    | u16le  | Frame size   | 0 |
//! | 0x0C    | u32le  | Sample rate  | in Hz |
//! | 0x10    | u32le  | Data offset  | 32, from the start of the file |
//! | 0x14    | u64le  | Filler       | 0 |
//! | 0x1C    | u32le  | Pre-skip     | Skip playing the first `n` samples |
//! | 0x20    | u32le  | Magic        | 0x8000_0004 |
//! | 0x24    | u32le  | Data size    | |
//! | 0x28    | Packet | Packets      | Until 0x24 + Data size |
//!
//! ## Packet
//! | Offset  | Type   | Name         | Explanation |
//! | ------- | ------ | ------------ | ----------- |
//! | p + 0x0 | u32be  | Packet size  | p is the end of the last packet |
//! | p + 0x4 | u32be  | Final range  | State of the encoder/decoder after this packet |
//! | p + 0x8 | u8     | TOC          | [Opus table-of-contents](https://datatracker.ietf.org/doc/html/rfc6716#autoid-25) |
//! | p + 0x9 | \[u8\] | Samples      | The encoded samples |
//!
//! If the last packet brings the total samples to the expected samples,
//! one more (empty) packet is written.
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
use tracing::{instrument, warn};

/// Mux a Nintendo Switch Opus file to a Ogg/Opus file
///
/// `num_of_samples` is the total amount of samples in this file.
/// This value is stored out-of-band.
#[instrument(skip(source, position, destination))]
pub fn mux_to_opus(
    source: &(impl ReadAtExt + ?Sized),
    position: &mut u64,
    destination: &mut (impl WriteAt + ?Sized),
    num_of_samples: u32,
) -> Result<(), Error> {
    let header = source.read_at::<NxOpusHeader>(position)?;
    let data_type = source.read_at::<u32le>(position)?;
    test_eq!(data_type, 0x8000_0004)?;
    let data_size = source.read_at::<u32le>(position)?;
    let data_end = *position + u64::from(data_size);

    let mut writer = PacketWriter::new(CursorAt::new(destination, 0));

    // Write the opus header to the first page
    let opus_header = OpusHeader {
        channels: header.channels,
        skip: i16::try_from(header.pre_skip)?,
        sample_rate: header.sample_rate,
        output_gain: 0,
    };
    let mut vec = Vec::new();
    OpusHeader::serialize(opus_header, &mut vec)?;
    writer.write_packet(vec, 0x0D15EA5E, EndPage, 0)?;

    // Write the comments to the second page
    let comments = OpusComments {
        vendor: Cow::Borrowed("UbiArt Toolkit"),
        comments: HashMap::new(),
    };
    let mut vec = Vec::new();
    OpusComments::serialize(comments, &mut vec)?;
    writer.write_packet(vec, 0x0D15EA5E, EndPage, 0)?;

    // Write all the opus packets to the third page
    let mut samples_read = 0;
    let mut samples_in_last_packet = 0;
    // Read one packet past the expected amount of samples
    while samples_read <= num_of_samples {
        // the first 8 bytes are not part of the opus packet, but nx specific
        let data_size = usize::try_from(source.read_at::<u32be>(position)?)?;
        let _final_range = source.read_at::<u32be>(position)?;

        // the actual opus packet
        let data = source.read_slice_at(position, data_size)?;

        // decode the toc byte so we can count the total samples
        let toc = OpusToc::deserialize(data.as_ref())?;
        if toc.mode != Mode::Celt
            || toc.bandwith != Bandwith::Full
            || toc.frame_duration != 20_000
            || toc.frames_per_packet != 1
        {
            warn!("Abnormal TOC-byte: {toc:#?}");
        }
        let samples = toc.samples(header.sample_rate);

        samples_in_last_packet = samples;
        samples_read += samples;

        // if we've read more samples than are valid, it means this should be the final packet
        if samples_read > num_of_samples {
            writer.write_packet(data, 0x0D15EA5E, EndStream, u64::from(num_of_samples))?;
        } else {
            writer.write_packet(data, 0x0D15EA5E, NormalPacket, u64::from(samples_read))?;
        };
    }

    // Verify that we've read all data
    if *position != data_end {
        warn!("Position is not at the end of data: position {position}, end of data: {data_end}");
    }

    // Verify that we haven't read too many samples
    // The margin on this is the amount of samples in the last packet (normally 960)
    if samples_read > (num_of_samples + samples_in_last_packet) {
        warn!(
            "Read more samples than expected: expected: {} read: {samples_read}",
            num_of_samples + samples_in_last_packet
        );
    }

    Ok(())
}

/// Mux a Ogg/Opus file to a Nintendo Switch Opus file
///
/// # Returns
/// This function returns a tuple of the Opus header and the number of samples written.
pub fn mux_from_opus(
    reader: &(impl ReadAtExt + ?Sized),
    destination: &mut (impl WriteAt + ?Sized),
    position: &mut u64,
) -> Result<(OpusHeader, u32), Error> {
    let mut ogg = ogg::PacketReader::new(CursorAt::new(reader, 0));

    // read all header packets into data
    // header packets can be recognized by having a granule position of 0
    let mut packet = ogg.read_packet_expected()?;
    let serial = packet.stream_serial();
    let mut data = Vec::new();
    while packet.absgp_page() == 0 {
        test_eq!(
            serial,
            packet.stream_serial(),
            "More than one stream in ogg file!"
        )?;
        data.extend_from_slice(&packet.data);
        packet = ogg.read_packet_expected()?;
    }

    // decode the header and the comments (unused)
    let mut read_position = 0;
    let header = OpusHeader::deserialize_at(&data, &mut read_position)?;
    let _comments = OpusComments::deserialize_at(&data, &mut read_position)?;

    test_le!(header.channels, 2, "Only mono and stereo are supported").or(test_eq!(
        header.channels,
        0,
        "Only mono and stereo are supported"
    ))?;

    // write the nx header
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

    let channels = match header.channels {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => unreachable!(),
    };
    let mut decoder = opus::Decoder::new(header.sample_rate, channels)?;
    let mut decode_buffer = vec![0i16; 960 * usize::from(header.channels)];

    let mut data_size = 0;
    // write all opus packets
    loop {
        test_eq!(
            serial,
            packet.stream_serial(),
            "More than one stream in ogg file!"
        )?;
        let size = u32::try_from(packet.data.len())?;

        // Make sure the decode buffer is large enough
        let toc = OpusToc::deserialize(&packet.data)?;
        if toc.mode != Mode::Celt
            || toc.bandwith != Bandwith::Full
            || toc.frame_duration != 20_000
            || toc.frames_per_packet != 1
        {
            warn!("Abnormal TOC-byte, output might be broken: {toc:#?}");
        }
        let samples = usize::try_from(toc.samples(header.sample_rate))?;
        decode_buffer.resize(
            decode_buffer
                .len()
                .max(samples * usize::from(header.channels)),
            0,
        );

        // Decode the packet to calculate the final range
        decoder.decode(&packet.data, &mut decode_buffer, false)?;
        let final_range = decoder.get_final_range()?;

        // Size of this packet is the packet size plus the eight NX metadata bytes
        data_size += size + 8;
        destination.write_at::<u32be>(position, size)?;
        destination.write_at::<u32be>(position, final_range)?;

        destination.write_slice_at(position, &packet.data)?;
        match ogg.read_packet()? {
            Some(new) => packet = new,
            None => break,
        }
    }

    // write the data size, now it's known
    destination.write_at::<u32le>(&mut position_data_size, data_size)?; // data size

    Ok((header, u32::try_from(packet.absgp_page())?))
}

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
    #[error("Failed to read ogg file: {0}")]
    OggReadError(#[from] OggReadError),
    #[error("Failed to decode the opus stream: {0}")]
    OpusDecodeError(#[from] opus::Error),
}

#[derive(Debug)]
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
        let magic = reader.read_at::<u32le>(position)?;
        test_eq!(magic, Self::MAGIC)?;
        let header_size = reader.read_at::<u32le>(position)?;
        test_eq!(header_size, 24)?; // header size excludes itself and the magic value
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

#[derive(Debug)]
pub struct OpusHeader {
    pub channels: u8,
    /// Amount of samples to skip when playing back
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
        test_eq!(mapping_file, 0, "Mapping file is not supported!")?;

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
    /// String identifying the application that created this file
    pub vendor: Cow<'a, str>,
    /// Metadata comments
    ///
    /// The key cannot contain `=`.
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

/// Table of Contents of an Opus packet
#[derive(Debug)]
pub struct OpusToc {
    /// Encoder/decoder mode
    pub mode: Mode,
    /// Frequency bandwith
    pub bandwith: Bandwith,
    /// Frame duration in microseconds
    pub frame_duration: u32,
    pub stereo: bool,
    pub frames_per_packet: u8,
}

impl OpusToc {
    /// Calculate the amount of samples in this packet
    ///
    /// `sample_rate` is in Hz
    #[must_use]
    pub fn samples(&self, sample_rate: u32) -> u32 {
        u32::from(self.frames_per_packet) * ((self.frame_duration * sample_rate) / 1_000_000)
    }
}

/// Encoder mode used for this packet
#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Silk,
    Hybrid,
    Celt,
}

/// Bandwith used for this packet
#[derive(Debug, PartialEq, Eq)]
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
        let (mode, bandwith, frame_duration) = match byte >> 3 {
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
            frame_duration,
            stereo,
            frames_per_packet,
        })
    }
}
