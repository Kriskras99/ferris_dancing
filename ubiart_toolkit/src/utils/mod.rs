pub mod bytes;

use std::{borrow::Cow, ffi::OsStr, fmt::Display, ops::Deref, path::Path};

use anyhow::{anyhow, Error};
use byteorder::LittleEndian;
use dotstar_toolkit_utils::bytes::read_u32_at;
use nohash_hasher::IsEnabled;
use serde::{Deserialize, Serialize};

use crate::ipk;

/// Represents the id of a localised string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct LocaleId(u32);

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

#[derive(Debug, Clone, Default)]
pub struct SplitPath<'a> {
    pub path: Cow<'a, str>,
    pub filename: Cow<'a, str>,
}

impl SplitPath<'_> {
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
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let pos = value
            .rfind('/')
            .ok_or_else(|| anyhow!("Path does not contain separator ('/')"))?;
        let (path, filename) = value.split_at(pos + 1);
        assert!(
            !filename.contains('/'),
            "Filename does not contain a '/': {filename}"
        );
        Ok(SplitPath {
            path: Cow::Borrowed(path),
            filename: Cow::Borrowed(filename),
        })
    }
}

/// The UbiArt CRC of a path converted to all caps
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathId(u32);
impl From<&str> for PathId {
    fn from(value: &str) -> Self {
        Self(string_id(value))
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

pub fn path_id<P: AsRef<Path>>(path: P) -> PathId {
    PathId::from(os_string_id(path.as_ref().as_os_str()))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GamePlatform {
    pub game: Game,
    pub platform: Platform,
    pub id: u32,
}

impl Display for GamePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{} for {}", self.game, self.platform)
    }
}

impl From<GamePlatform> for u32 {
    fn from(value: GamePlatform) -> Self {
        value.id
    }
}

impl TryFrom<u32> for GamePlatform {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1C24_B91A => Ok(Self {
                game: Game::JustDance2014,
                platform: Platform::Wii,
                id: value,
            }),
            0xC563_9F58 => Ok(Self {
                game: Game::JustDance2015,
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
            _ => Err(anyhow!("Unknown game platform: {value:x}")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
            #[allow(clippy::as_conversions)]
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    X360,
    Ps3,
    Ps4,
    Wii,
    WiiU,
    Nx,
}

impl TryFrom<Platform> for ipk::Platform {
    type Error = anyhow::Error;

    fn try_from(value: Platform) -> Result<Self, Self::Error> {
        match value {
            Platform::X360 => Ok(Self::X360),
            Platform::Ps4 => Ok(Self::Ps4),
            Platform::Wii => Ok(Self::Wii),
            Platform::WiiU => Ok(Self::WiiU),
            Platform::Nx => Ok(Self::Nx),
            Platform::Ps3 => Err(anyhow!("No IPK platform number for {value}")),
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

/// Calculates the Ubisoft string id for a given string.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
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

#[must_use]
/// Implementation of the UbiArt CRC function
pub fn ubi_crc(data: &[u8]) -> u32 {
    let length = data.len();
    let mut a: u32 = 0x9E37_79B9;
    let mut b: u32 = 0x9E37_79B9;
    let mut c: u32 = 0;

    let mut pos = 0;
    while pos + 12 <= length {
        a = a.wrapping_add(
            read_u32_at::<LittleEndian>(data, &mut pos).unwrap_or_else(|_| unreachable!()),
        );
        b = b.wrapping_add(
            read_u32_at::<LittleEndian>(data, &mut pos).unwrap_or_else(|_| unreachable!()),
        );
        c = c.wrapping_add(
            read_u32_at::<LittleEndian>(data, &mut pos).unwrap_or_else(|_| unreachable!()),
        );
        (a, b, c) = shifter(a, b, c);
    }

    c = wrapping_add(c, length);
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

#[cfg(target_pointer_width = "64")]
#[allow(clippy::as_conversions)]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
/// Convenience function for wrapping add
const fn wrapping_add(lhs: u32, rhs: usize) -> u32 {
    let mod_rhs = (rhs % (u32::MAX as usize)) as u32;
    lhs.wrapping_add(mod_rhs)
}

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "16",
    target_pointer_width = "8"
))]
#[allow(clippy::as_conversions)]
#[allow(clippy::cast_possible_truncation)]
#[must_use]
/// Convenience function for wrapping add
const fn wrapping_add(lhs: u32, rhs: usize) -> u32 {
    lhs.wrapping_add(rhs as u32)
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
    use super::string_id;

    #[test]
    fn test_string_id() {
        assert_eq!(
            string_id("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm"),
            0x45CC_A9CA
        );
    }
}
