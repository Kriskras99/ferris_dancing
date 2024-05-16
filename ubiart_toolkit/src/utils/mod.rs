pub mod errors;

use std::{borrow::Cow, ffi::OsStr, fmt::Display, ops::Deref};

pub mod bytes;
pub mod plumbing;

use clap::ValueEnum;
use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    testing::{test, test_eq, TestError},
    vfs::{VirtualPath, VirtualPathBuf},
};
use nohash_hasher::IsEnabled;
use serde::{Deserialize, Serialize};

use self::errors::ParserError;
use crate::ipk;

/// A RGBA color encoded in f32 (0.0 is black, 1.0 is white)
pub type Color = (f32, f32, f32, f32);

/// Represents the id of a localised string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct LocaleId(u32);

impl BinaryDeserialize<'_> for LocaleId {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(Self(reader.read_at::<u32be>(position)?))
    }
}

impl Default for LocaleId {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

impl Display for LocaleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x}", self.0))
    }
}

impl LocaleId {
    /// The empty/missing LocaleId
    pub const EMPTY: Self = Self(u32::MAX);
    /// The minimum value of a LocaleId
    pub const MIN: Self = Self(0);

    /// Increments the locale id and returns a new higher locale id
    ///
    /// # Panics
    /// Will panic if the increment would cause an overflow
    #[must_use]
    pub fn increment(&self) -> Self {
        Self(self.0.checked_add(1).unwrap())
    }
}

impl From<u32> for LocaleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<LocaleId> for u32 {
    fn from(value: LocaleId) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SplitPath<'a> {
    path: Cow<'a, str>,
    filename: Cow<'a, str>,
}

impl<'a> SplitPath<'a> {
    const EMPTY_PATH_ID: PathId = PathId(0xFFFF_FFFF);
    const PADDING: u32 = 0x0;

    pub fn new(mut path: Cow<'a, str>, filename: Cow<'a, str>) -> Result<Self, TestError> {
        if !path.is_empty() && !path.ends_with('/') {
            let mut string = path.into_owned();
            string.push('/');
            path = string.into();
        }
        test(path.ends_with('/')).or(test(path.is_empty()))?;
        test(!path.contains('.'))?;
        test(!filename.ends_with('/'))?;
        test(!filename.starts_with('/'))?;
        Ok(Self { path, filename })
    }

    /// The total length of the path and filename combined
    #[must_use]
    pub fn len(&self) -> usize {
        self.path.len() + self.filename.len()
    }

    /// Returns `true` if the path and filename are empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty() && self.filename.is_empty()
    }

    #[must_use]
    pub fn id(&self) -> PathId {
        if self.is_empty() {
            Self::EMPTY_PATH_ID
        } else {
            PathId::from(self)
        }
    }

    #[must_use]
    pub fn starts_with(&self, pattern: &str) -> bool {
        let (path_pattern, filename_pattern) = pattern.split_at(self.path.len().min(pattern.len()));
        self.path.starts_with(path_pattern) && self.filename.starts_with(filename_pattern)
    }

    #[must_use]
    /// Will only check if pattern is completely in path or filename, not partly in both
    // TODO: Make this work when the pattern is partly in both
    pub fn contains(&self, pattern: &str) -> bool {
        self.path.contains(pattern) || self.filename.contains(pattern)
    }

    #[must_use]
    pub fn parent(&self) -> &str {
        &self.path
    }

    #[must_use]
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

impl<'de> BinaryDeserialize<'de> for SplitPath<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let filename = reader.read_len_string_at::<u32be>(position)?;
            let path = reader.read_len_string_at::<u32be>(position)?;
            let path_id = reader.read_at::<PathId>(position)?;
            let split_path = SplitPath::new(path, filename)?;
            test_eq(&path_id, &split_path.id())?;
            let padding = reader.read_at::<u32be>(position)?.into();
            test_eq(&padding, &Self::PADDING)?;
            split_path
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl BinarySerialize for SplitPath<'_> {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_len_string_at::<u32be>(position, &input.filename)?;
        writer.write_len_string_at::<u32be>(position, &input.path)?;
        writer.write_at::<PathId>(position, input.id())?;
        writer.write_at::<u32be>(position, Self::PADDING)?;

        Ok(())
    }
}

impl Display for SplitPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.path)?;
        f.write_str(&self.filename)
    }
}

impl From<&SplitPath<'_>> for PathId {
    fn from(value: &SplitPath<'_>) -> Self {
        Self(string_id_2(&value.path, &value.filename))
    }
}

impl<'a> TryFrom<&'a str> for SplitPath<'a> {
    type Error = ParserError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (path, filename) = match value.rfind('/') {
            Some(pos) => value.split_at(pos + 1),
            None => ("", value),
        };
        let path = path.strip_prefix('/').unwrap_or(path);
        Ok(SplitPath::new(
            Cow::Borrowed(path),
            Cow::Borrowed(filename),
        )?)
    }
}

impl<'a> TryFrom<&'a VirtualPath> for SplitPath<'a> {
    type Error = ParserError;

    fn try_from(value: &'a VirtualPath) -> Result<Self, Self::Error> {
        let value = value.as_str();
        Self::try_from(value)
    }
}

impl From<&SplitPath<'_>> for VirtualPathBuf {
    fn from(value: &SplitPath<'_>) -> Self {
        let mut pb = Self::with_capacity(value.len());
        pb.push(value.path.as_ref());
        pb.push(value.filename.as_ref());
        pb
    }
}

/// The UbiArt CRC of a path converted to all caps
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathId(u32);

impl PathId {
    pub const EMPTY: Self = Self(u32::MAX);
}

impl From<&str> for PathId {
    fn from(mut value: &str) -> Self {
        if value.starts_with('/') {
            value = &value[1..];
        }
        Self(string_id(value))
    }
}

impl From<&VirtualPath> for PathId {
    fn from(value: &VirtualPath) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&VirtualPathBuf> for PathId {
    fn from(value: &VirtualPathBuf) -> Self {
        Self::from(value.as_str())
    }
}

impl From<VirtualPathBuf> for PathId {
    fn from(value: VirtualPathBuf) -> Self {
        Self::from(value.as_str())
    }
}

impl From<u32> for PathId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl From<PathId> for u32 {
    fn from(value: PathId) -> Self {
        value.0
    }
}
impl IsEnabled for PathId {}
impl Deref for PathId {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BinaryDeserialize<'_> for PathId {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(Self(reader.read_at::<u32be>(position)?.into()))
    }
}

impl BinarySerialize for PathId {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, input.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGameId {
    pub game: Game,
    pub platform: Platform,
    pub id: u32,
}

impl UniqueGameId {
    pub const WIIU2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::WiiU,
        id: 0x04A2_5379,
    };
    pub const NX2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::Nx,
        id: 0x32F3_512A,
    };
    pub const NX2018: Self = Self {
        game: Game::JustDance2018,
        platform: Platform::Nx,
        id: 0x032E_71C5,
    };
    pub const NX2019: Self = Self {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0x57A7_053C,
    };
    pub const NX2020: Self = Self {
        game: Game::JustDance2020,
        platform: Platform::Nx,
        id: 0x217A_94CE,
    };
    pub const NX_CHINA: Self = Self {
        game: Game::JustDanceChina,
        platform: Platform::Nx,
        id: 0xA155_8F87,
    };
    pub const NX2021: Self = Self {
        game: Game::JustDance2021,
        platform: Platform::Nx,
        id: 0xA4F0_18EE,
    };
    pub const NX2022: Self = Self {
        game: Game::JustDance2022,
        platform: Platform::Nx,
        id: 0x1DDB_2268,
    };
}

impl Display for UniqueGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{} for {}", self.game, self.platform)
    }
}

impl From<UniqueGameId> for u32 {
    fn from(value: UniqueGameId) -> Self {
        value.id
    }
}

impl TryFrom<u32> for UniqueGameId {
    type Error = ParserError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1C24_B91A => Ok(Self {
                game: Game::JustDance2014,
                platform: Platform::WiiU,
                id: value,
            }),
            0xC563_9F58 => Ok(Self {
                game: Game::JustDance2015,
                platform: Platform::WiiU,
                id: value,
            }),
            // Base       Update 1      Update 2
            0xDA14_5C61 | 0x8C9D_65E4 | 0xF9D_9B22B => Ok(Self {
                game: Game::JustDance2016,
                platform: Platform::WiiU,
                id: value,
            }),
            0x04A2_5379 => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::WiiU,
                id: value,
            }),
            0x415E_6D8C | 0x32F3_512A => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::Nx,
                id: value,
            }),
            0x032E_71C5 => Ok(Self {
                game: Game::JustDance2018,
                platform: Platform::Nx,
                id: value,
            }),
            0x1F5E_E42F | 0xC781_A65B | 0x57A7_053C => Ok(Self {
                game: Game::JustDance2019,
                platform: Platform::Nx,
                id: value,
            }),
            0x72B4_2FF4 | 0xB292_FD08 | 0x217A_94CE => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Nx,
                id: value,
            }),
            0xA155_8F87 => Ok(Self {
                game: Game::JustDanceChina,
                platform: Platform::Nx,
                id: value,
            }),
            0x4C8E_C5C5 => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Wii,
                id: value,
            }),
            0xEB5D_504C | 0xA4F0_18EE => Ok(Self {
                game: Game::JustDance2021,
                platform: Platform::Nx,
                id: value,
            }),
            0x1DDB_2268 => Ok(Self {
                game: Game::JustDance2022,
                platform: Platform::Nx,
                id: value,
            }),
            _ => Err(ParserError::custom(format!(
                "Unknown game platform: {value:x}"
            ))),
        }
    }
}

impl BinaryDeserialize<'_> for UniqueGameId {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let value = reader.read_at::<u32be>(position)?;
        Self::try_from(value)
            .map_err(|_| ReadError::custom(format!("Unknown game platform: {value:x}")))
    }
}

impl BinarySerialize for UniqueGameId {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, input.id)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Game {
    JustDance2014 = 20140,
    JustDance2015 = 20150,
    JustDance2016 = 20160,
    JustDance2017 = 20170,
    JustDance2018 = 20180,
    JustDance2019 = 20190,
    JustDance2020 = 20200,
    JustDanceChina = 20201,
    JustDance2021 = 20210,
    JustDance2022 = 20220,
    Unknown,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if *self == Self::Unknown || *other == Self::Unknown {
            None
        } else {
            #[allow(
                clippy::as_conversions,
                reason = "the enum values are in the range of 20140-20220 so is always safe"
            )]
            (*self as u32).partial_cmp(&(*other as u32))
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JustDance2014 => std::write!(f, "Just Dance 2014"),
            Self::JustDance2015 => std::write!(f, "Just Dance 2015"),
            Self::JustDance2016 => std::write!(f, "Just Dance 2016"),
            Self::JustDance2017 => std::write!(f, "Just Dance 2017"),
            Self::JustDance2018 => std::write!(f, "Just Dance 2018"),
            Self::JustDance2019 => std::write!(f, "Just Dance 2019"),
            Self::JustDance2020 => std::write!(f, "Just Dance 2020"),
            Self::JustDanceChina => std::write!(f, "Just Dance China"),
            Self::JustDance2021 => std::write!(f, "Just Dance 2021"),
            Self::JustDance2022 => std::write!(f, "Just Dance 2022"),
            Self::Unknown => std::write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Platform {
    X360,
    Ps3,
    Ps4,
    Wii,
    WiiU,
    Nx,
}

impl From<Platform> for ipk::IpkPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::X360 => Self::X360,
            Platform::Ps4 => Self::Ps4,
            Platform::Wii => Self::Wii,
            Platform::WiiU => Self::WiiU,
            Platform::Nx => Self::Nx,
            Platform::Ps3 => todo!("No IPK platform number known for {value}"),
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X360 => std::write!(f, "Xbox 360"),
            Self::Ps3 => std::write!(f, "PlayStation 3"),
            Self::Ps4 => std::write!(f, "PlayStation 4"),
            Self::Wii => std::write!(f, "Wii"),
            Self::WiiU => std::write!(f, "Wii U"),
            Self::Nx => std::write!(f, "Switch"),
        }
    }
}

// Calculates the Ubisoft string id for a given string.
//
// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id(string: &str) -> u32 {
    let bytes = string.as_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for a given os string.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn os_string_id(string: &OsStr) -> u32 {
    let bytes = string.as_encoded_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for two strings.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id_2(one: &str, two: &str) -> u32 {
    let bytes_one = one.as_bytes();
    let bytes_two = two.as_bytes();
    let mut upper = Vec::with_capacity(bytes_one.len() + bytes_two.len());
    // Convert lowercase chars to uppercase
    for byte in bytes_one {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    for byte in bytes_two {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

#[inline]
fn read_u32le(data: &[u8], pos: &mut usize) -> u32 {
    let value = u32::from_le_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
    *pos += 4;
    value
}

#[must_use]
/// Implementation of the UbiArt CRC function
#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "Truncating is wanted"
)]
pub fn ubi_crc(data: &[u8]) -> u32 {
    let length = data.len();
    let mut a: u32 = 0x9E37_79B9;
    let mut b: u32 = 0x9E37_79B9;
    let mut c: u32 = 0;

    let mut pos = 0;
    while pos + 12 <= length {
        a = a.wrapping_add(read_u32le(data, &mut pos));
        b = b.wrapping_add(read_u32le(data, &mut pos));
        c = c.wrapping_add(read_u32le(data, &mut pos));
        (a, b, c) = shifter(a, b, c);
    }

    c = c.wrapping_add(length as u32);
    let left = length - pos;

    if left > 0 {
        if left >= 11 {
            c = c.wrapping_add(u32::from(data[pos + 10]) << 24);
        }
        if left >= 10 {
            c = c.wrapping_add(u32::from(data[pos + 9]) << 16);
        }
        if left >= 9 {
            c = c.wrapping_add(u32::from(data[pos + 8]) << 8);
        }
        if left >= 8 {
            b = b.wrapping_add(u32::from(data[pos + 7]) << 24);
        }
        if left >= 7 {
            b = b.wrapping_add(u32::from(data[pos + 6]) << 16);
        }
        if left >= 6 {
            b = b.wrapping_add(u32::from(data[pos + 5]) << 8);
        }
        if left >= 5 {
            b = b.wrapping_add(u32::from(data[pos + 4]));
        }
        if left >= 4 {
            a = a.wrapping_add(u32::from(data[pos + 3]) << 24);
        }
        if left >= 3 {
            a = a.wrapping_add(u32::from(data[pos + 2]) << 16);
        }
        if left >= 2 {
            a = a.wrapping_add(u32::from(data[pos + 1]) << 8);
        }
        if left >= 1 {
            a = a.wrapping_add(u32::from(data[pos]));
        }
    }

    (_, _, c) = shifter(a, b, c);
    c
}

/// Shifting implementation for ubicrc
const fn shifter(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = (a.wrapping_sub(b).wrapping_sub(c)) ^ (c >> 0xD);
    b = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x8);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xD);
    a = (a.wrapping_sub(c).wrapping_sub(b)) ^ (c >> 0xC);
    let d = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x10);
    c = (c.wrapping_sub(a).wrapping_sub(d)) ^ (d >> 0x5);
    a = (a.wrapping_sub(c).wrapping_sub(d)) ^ (c >> 0x3);
    b = (d.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0xA);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xF);
    (a, b, c)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use dotstar_toolkit_utils::vfs::VirtualPathBuf;

    use super::{string_id, PathId, SplitPath};

    #[test]
    fn test_string_id() {
        assert_eq!(
            string_id("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm"),
            0x45CC_A9CA
        );
    }

    #[test]
    fn test_splitpath_try_from_path() {
        let path = VirtualPathBuf::from("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm");
        let sp = SplitPath::try_from(path.as_path()).unwrap();
        assert_eq!(&PathId::from(&sp), &PathId::from(0x45CC_A9CA));
    }

    #[test]
    fn test_splitpath_starts_with() {
        let split_path = SplitPath::new(
            Cow::Borrowed("cache/itf_cooked/nx/"),
            Cow::Borrowed("atlascontainer.ckd"),
        )
        .unwrap();
        assert!(split_path.starts_with("cache"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlas"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlascontainer.ckd"));
    }
}
