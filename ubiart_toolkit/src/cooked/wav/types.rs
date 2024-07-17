use std::{borrow::Cow, collections::HashMap};

use dotstar_toolkit_utils::bytes::{
    primitives::u32be,
    read::{BinaryDeserialize, ReadAtExt, ReadError},
};

#[derive(Debug)]
pub struct Wav<'a> {
    pub unk1: u32,
    pub unk2: u32,
    pub platform: WavPlatform,
    pub codec: Codec,
    pub header_size: u32,
    pub data_start_offset: u32,
    pub chunks: HashMap<u32, Chunk<'a>>,
}

impl Wav<'_> {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"RAKI");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum WavPlatform {
    DS3 = u32::from_be_bytes(*b"CTR\0"),
    PS3 = u32::from_be_bytes(*b"PS3 "),
    PS4 = u32::from_be_bytes(*b"Orbi"),
    Switch = u32::from_be_bytes(*b"Nx  "),
    Vita = u32::from_be_bytes(*b"VITA"),
    Wii = u32::from_be_bytes(*b"Wii "),
    WiiU = u32::from_be_bytes(*b"Cafe"),
    Windows = u32::from_be_bytes(*b"Win "),
    X360 = u32::from_be_bytes(*b"X360"),
}

impl TryFrom<u32> for WavPlatform {
    type Error = ReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match &value.to_be_bytes() {
            b"CTR\0" => Ok(Self::DS3),
            b"PS3 " => Ok(Self::PS3),
            b"Orbi" => Ok(Self::PS4),
            b"Nx  " => Ok(Self::Switch),
            b"VITA" => Ok(Self::Vita),
            b"Wii " => Ok(Self::Wii),
            b"Cafe" => Ok(Self::WiiU),
            b"Win " => Ok(Self::Windows),
            b"X360" => Ok(Self::X360),
            _ => Err(ReadError::custom(format!("Unknown platform! {value:x}"))),
        }
    }
}

impl From<WavPlatform> for u32 {
    #[allow(clippy::as_conversions, reason = "It's repr(u32)")]
    fn from(value: WavPlatform) -> Self {
        value as Self
    }
}

impl BinaryDeserialize<'_> for WavPlatform {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
    }
}

/// The codec that is used
///
/// Note: for `Adpc` this is meaningless without also knowing the [`WavPlatform`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Codec {
    Adpc = u32::from_be_bytes(*b"adpc"),
    At9 = u32::from_be_bytes(*b"at9 "),
    Mp3 = u32::from_be_bytes(*b"mp3 "),
    Nx = u32::from_be_bytes(*b"Nx  "),
    PCM = u32::from_be_bytes(*b"pcm "),
    Xma2 = u32::from_be_bytes(*b"xma2"),
}

impl TryFrom<u32> for Codec {
    type Error = ReadError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match &value.to_be_bytes() {
            b"adpc" => Ok(Self::Adpc),
            b"at9 " => Ok(Self::At9),
            b"mp3 " => Ok(Self::Mp3),
            b"Nx  " => Ok(Self::Nx),
            b"pcm " => Ok(Self::PCM),
            b"xma2" => Ok(Self::Xma2),
            _ => Err(ReadError::custom(format!("Unknown platform! {value:x}"))),
        }
    }
}

impl From<Codec> for u32 {
    #[allow(clippy::as_conversions, reason = "It's repr(u32)")]
    fn from(value: Codec) -> Self {
        value as Self
    }
}

impl BinaryDeserialize<'_> for Codec {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
    }
}

/// A chunk in the audio file
#[derive(Debug)]
pub enum Chunk<'a> {
    /// Basic codec information
    Fmt(Fmt),
    /// Aditional information needed by a codec
    AdIn(AdIn),
    /// The samples
    Data(Data),
    /// Seek table
    Mark(Mark),
    /// Description
    Strg(Strg<'a>),
    /// The samples (stereo)
    DatS(Data),
    /// The samples (left channel or mono)
    DatL(Data),
    /// The samples (right channel)
    DatR(Data),
    /// Dsp coefficients (left channel or mono)
    DspL(Dsp),
    /// Dsp coefficients (right channel)
    DspR(Dsp),
}

impl Chunk<'_> {
    /// Get the magic of this chunk
    #[must_use]
    pub const fn magic(&self) -> u32 {
        match self {
            Chunk::Fmt(_) => Fmt::MAGIC,
            Chunk::AdIn(_) => AdIn::MAGIC,
            Chunk::Data(_) => Data::MAGIC,
            Chunk::DatS(_) => Data::MAGIC_STEREO,
            Chunk::DatL(_) => Data::MAGIC_LEFT,
            Chunk::DatR(_) => Data::MAGIC_RIGHT,
            Chunk::Mark(_) => Mark::MAGIC,
            Chunk::Strg(_) => Strg::MAGIC,
            Chunk::DspL(_) => Dsp::MAGIC_LEFT,
            Chunk::DspR(_) => Dsp::MAGIC_RIGHT,
        }
    }

    /// Extract this chunk as a `Fmt` chunk
    pub fn as_fmt(&self) -> Result<&Fmt, ReadError> {
        if let Chunk::Fmt(fmt) = self {
            Ok(fmt)
        } else {
            Err(ReadError::custom("Not a fmt chunk!".into()))
        }
    }

    /// Extract this chunk as a `Data` chunk
    ///
    /// Note: this work for `Chunk::Data`, `Chunk::DatS`, `Chunk::DatL`, and `Chunk::DatR`
    pub fn as_data(&self) -> Result<&Data, ReadError> {
        match self {
            Chunk::Data(data) | Chunk::DatS(data) | Chunk::DatL(data) | Chunk::DatR(data) => {
                Ok(data)
            }
            _ => Err(ReadError::custom("Not a data chunk!".into())),
        }
    }

    /// Extract this chunk as a `AdIn` chunk
    pub fn as_adin(&self) -> Result<&AdIn, ReadError> {
        if let Chunk::AdIn(data) = self {
            Ok(data)
        } else {
            Err(ReadError::custom("Not a AdIn chunk!".into()))
        }
    }

    /// Extract this chunk as a `Dsp` chunk
    ///
    /// Note: this work for `Chunk::DspL` and `Chunk::DspR`
    pub fn as_dsp(&self) -> Result<&Dsp, ReadError> {
        match self {
            Chunk::DspL(data) | Chunk::DspR(data) => Ok(data),
            _ => Err(ReadError::custom("Not a dsp chunk!".into())),
        }
    }
}

#[derive(Debug)]
pub struct Fmt {
    pub unk1: u16,
    pub channel_count: u16,
    /// Sample rate in Hz
    pub sample_rate: u32,
    pub unk2: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

impl Fmt {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"fmt ");
    pub const SIZE: u32 = 16;
}

#[derive(Debug)]
pub struct AdIn {
    pub num_of_samples: u32,
}

impl AdIn {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"AdIn");
    pub const SIZE: u32 = 4;
}

#[derive(Debug)]
pub struct Data {
    pub position: u64,
    pub size: u32,
}

impl Data {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"data");
    pub const MAGIC_STEREO: u32 = u32::from_be_bytes(*b"datS");
    pub const MAGIC_LEFT: u32 = u32::from_be_bytes(*b"datL");
    pub const MAGIC_RIGHT: u32 = u32::from_be_bytes(*b"datR");
}

#[derive(Debug)]
pub struct Mark {
    pub position: u64,
    pub size: u32,
}

impl Mark {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"MARK");
}

#[derive(Debug)]
pub struct Strg<'a> {
    pub unk1: u32,
    pub unk2: u32,
    pub data: StrOrRaw<'a>,
}

#[derive(Debug)]
pub enum StrOrRaw<'a> {
    String(Cow<'a, str>),
    Raw(Cow<'a, [u8]>),
}

impl Strg<'_> {
    pub const MAGIC: u32 = u32::from_be_bytes(*b"STRG");
}

#[derive(Debug)]
pub struct Dsp {
    pub coefficients: [i16; 0x10],
    pub sample_count: u32,
    pub nibble_count: u32,
    pub sample_rate: u32,
    pub loop_flag: bool,
    pub loop_start_offset: u32,
    pub loop_end_offset: u32,
    pub current_address: u32,
    pub gain: u16,
    pub initial_predictor_scale: u16,
    pub initial_sample_history_1: i16,
    pub initial_sample_history_2: i16,
    pub loop_context_predictor_scale: u16,
    pub loop_context_sample_history_1: i16,
    pub loop_context_sample_history_2: i16,
}

impl Dsp {
    pub const MAGIC_LEFT: u32 = u32::from_be_bytes(*b"dspL");
    pub const MAGIC_RIGHT: u32 = u32::from_be_bytes(*b"dspR");
}
