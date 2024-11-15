//! Path related stuff, like [`SplitPath`] and [`PathId`]
use std::{fmt::Display, ops::Deref};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    vfs::{VirtualPath, VirtualPathBuf},
};
use hipstr::HipStr;
use nohash_hasher::IsEnabled;
use serde::Serialize;
use test_eq::{test_and, test_eq, test_or, TestFailure};
use ubiart_toolkit_shared_types::errors::ParserError;

use crate::utils::{string_id, string_id_2};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct SplitPath<'a> {
    path: HipStr<'a>,
    filename: HipStr<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedPadding {
    Value(u32),
    None,
}

impl Default for ExpectedPadding {
    fn default() -> Self {
        Self::Value(0)
    }
}

impl<'a> SplitPath<'a> {
    const EMPTY_PATH_ID: PathId = PathId(0xFFFF_FFFF);
    const PADDING: u32 = 0x0;

    pub fn new(mut path: HipStr<'a>, filename: HipStr<'a>) -> Result<Self, TestFailure> {
        if !path.is_empty() && !path.ends_with('/') {
            let mut string = path.into_owned();
            string.push('/');
            path = string;
        }
        test_or!(
            test_eq!(path.ends_with('/'), true),
            test_eq!(path.is_empty(), true)
        )?;
        test_eq!(
            !path.contains('.'),
            true,
            "Path: {path}, filename: {filename}"
        )?;
        test_and!(
            test_eq!(!filename.ends_with('/'), true),
            test_eq!(!filename.starts_with('/'), true)
        )?;
        Ok(Self { path, filename })
    }

    /// The total length of the path and filename combined
    #[must_use]
    pub const fn len(&self) -> usize {
        self.path.len() + self.filename.len()
    }

    /// Returns `true` if the path and filename are empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
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
    type Ctx = ExpectedPadding;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        expected_padding: Self::Ctx,
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            let filename = reader.read_len_string_at::<u32be>(position)?;
            let path = reader.read_len_string_at::<u32be>(position)?;
            let path_id = reader.read_at::<PathId>(position)?;
            let split_path = SplitPath::new(path, filename)?;
            test_eq!(path_id, split_path.id())?;
            if let ExpectedPadding::Value(expected_padding) = expected_padding {
                let padding = reader.read_at::<u32be>(position)?;
                test_eq!(padding, expected_padding, "Position: {position}")?;
            }
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
            HipStr::borrowed(path),
            HipStr::borrowed(filename),
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
        pb.push(value.path.as_str());
        pb.push(value.filename.as_str());
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

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(Self(reader.read_at::<u32be>(position)?))
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
