use dotstar_toolkit_utils::bytes::{primitives::u32be, read::{BinaryDeserialize, ReadAtExt, ReadError}};

pub struct Wav {}

impl Wav {
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
            _ => Err(ReadError::custom(format!("Unknown platform! {value:x}")))
        }
    }
}

impl From<WavPlatform> for u32 {
    fn from(value: WavPlatform) -> Self {
        value as u32
    }
}

impl BinaryDeserialize<'_> for WavPlatform {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Ok(Self::try_from(reader.read_at::<u32be>(position)?)?)
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
            _ => Err(ReadError::custom(format!("Unknown platform! {value:x}")))
        }
    }
}

impl From<Codec> for u32 {
    fn from(value: Codec) -> Self {
        value as u32
    }
}

impl BinaryDeserialize<'_> for Codec {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Ok(Self::try_from(reader.read_at::<u32be>(position)?)?)
    }
}

