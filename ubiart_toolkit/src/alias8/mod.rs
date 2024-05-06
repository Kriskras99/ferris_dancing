//! `.alias8` files contain aliases for paths
//!
//! # File structure
//! ## Header
//! | Pos | Size | Type    | Id            | Description                   |
//! |-----|------|---------|---------------|-------------------------------|
//! | 0x0 | 4    | `u32be` | `unk1`        | Always 0x2                    |
//! | 0x4 | 4    | `u32be` | `num_aliases` | Number of aliases in the file |
//! | 0x8 | ...  | `Alias` | `aliases`     | Repeated `num_aliases` times  |
//!
//! ## Alias
//! | Pos | Size         | Type        | Id           | Description                                        |
//! |-----|--------------|-------------|--------------|----------------------------------------------------|
//! | 0x0 | 4            | `u32be`     | `len_alias1` | The length of the first alias                      |
//! | 0x4 | `len_alias1` | `String`    | `alias1`     | The first alias                                    |
//! | ... | 4            | `u32be`     | `len_alias2` | The lenth of the second alias                      |
//! | ... | `len_alias2` | `String`    | `alias2`     | The second alias                                   |
//! | ... | ...          | `SplitPath` | `path`       | The path the aliases point to                      |
//! | ... | 2            | `u16be`     | `unk2`       | Always 0xFFFF                                      |
//! | ... | 2            | `u16be`     | `unk3`       | Probably flags, possible values in [`Unk3`] |
#![deny(clippy::missing_docs_in_private_items)]

use std::{borrow::Cow, collections::HashMap};

use bitflags::bitflags;
use dotstar_toolkit_utils::{
    bytes::{
        endian::BigEndian,
        len::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
        write::{BinarySerialize, WriteAt, WriteError},
    },
    testing::{test_any, test_eq, test_not},
};

use crate::utils::SplitPath;

/// Describes a single alias
#[derive(Debug, Clone)]
pub struct Alias<'a> {
    /// The alias name
    pub alias: Cow<'a, str>,
    /// The (uncooked) path for the alias
    pub path: SplitPath<'a>,
    /// Unknown value
    pub unk3: u16,
}

bitflags! {
    pub struct Unk3: u16 {
        const Win32        = 0b0000_0000_0000_0001;
        const X360         = 0b0000_0000_0000_0010;
        const UnkPlatform1 = 0b0000_0000_0000_0100;
        const PS4          = 0b0000_0000_0000_1000;
        const UnkPlatform2 = 0b0000_0000_0001_0000;
        const UnkPlatform3 = 0b0000_0000_0010_0000;
        const UnkPlatform4 = 0b0000_0000_0100_0000;
        const UnkPlatform5 = 0b0000_0000_1000_0000;
        const WiiU         = 0b0000_0001_0000_0000;
        const UnkPlatform6 = 0b0000_0010_0000_0000;
        const XOne         = 0b0000_0100_0000_0000;
        const Switch       = 0b0000_1000_0000_0000;
        const Unk1         = 0b0001_0000_0000_0000;
        const Unk2         = 0b0010_0000_0000_0000;
        const Unk3         = 0b0100_0000_0000_0000;
        const Unk4         = 0b1000_0000_0000_0000;
        const Platforms    = 0b0000_1111_1111_1111;
    }
}

impl Alias<'_> {
    /// UNK2 is always u16::MAX
    const UNK2: u16 = 0xFFFF;
    /// Possible values for UNK3
    const UNK3: &'static [u16] = &[
        0x8001, 0x8002, 0x8008, 0x8100, 0x83D6, 0x8400, 0x8800, 0x9000, 0xA000, 0xC000, 0xE001,
        0xE002, 0xE008, 0xE100, 0xE400, 0xE800, 0xEFDF, 0xF000, 0xF001, 0xF002, 0xF008, 0xF100,
        0xF400, 0xF800, 0xF801, 0xFC08, 0xFD19, 0xFFDF, 0xFFFF,
    ];
}

impl<'de> BinaryDeserialize<'de> for Alias<'de> {
    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            // Read the strings
            let alias = reader.read_len_string_at::<u32be>(position)?;
            let second_alias = reader.read_len_string_at::<u32be>(position)?;
            test_eq(&alias, &second_alias).context("1st and 2nd alias are not the same!")?;
            let path = reader.read_at(position)?;

            // Read the unknown values and check them
            let unk2 = reader.read_at_with_ctx(position, &BigEndian)?;
            let unk3 = reader.read_at_with_ctx(position, &BigEndian)?;
            test_eq(&unk2, &Self::UNK2)?;
            test_any(&unk3, Self::UNK3)?;

            Self { alias, path, unk3 }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl BinarySerialize for Alias<'_> {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), WriteError> {
        writer.write_len_string_at::<u32be>(position, &self.alias)?;
        writer.write_len_string_at::<u32be>(position, &self.alias)?;
        writer.write_at(position, &self.path)?;
        writer.write_at_with_ctx(position, &Self::UNK2, &BigEndian)?;
        writer.write_at_with_ctx(position, &self.unk3, &BigEndian)?;
        Ok(())
    }
}

/// Describes the entire file
#[derive(Debug, Clone)]
pub struct Alias8<'a> {
    /// The aliases in this file
    aliases: HashMap<Cow<'a, str>, Alias<'a>>,
}

impl<'a> Alias8<'a> {
    /// UNK1 is always 0x2
    const UNK1: u32 = 0x2;

    /// Find the path for a given alias
    #[must_use]
    pub fn get_path_for_alias(&self, alias: &str) -> Option<String> {
        self.aliases.get(alias).map(|a| a.path.to_string())
    }

    pub fn aliases(&self) -> impl Iterator<Item = &Alias<'a>> {
        self.aliases.values()
    }
}

impl<'de> BinaryDeserialize<'de> for Alias8<'de> {
    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<Self, ReadError> {
        let old_position = *position;
        let result: Result<_, _> = try {
            // Read the unknown value at the beginning and check it's correct
            let unk1 = reader.read_at_with_ctx(position, &BigEndian)?;
            test_eq(&unk1, &Self::UNK1)?;

            // Read the aliases
            let lazy_aliases = reader.read_len_type_at::<u32be, Alias>(position)?;
            let mut aliases = HashMap::with_capacity(lazy_aliases.size_hint().0);
            for alias in lazy_aliases {
                let alias = alias?;
                let exists = aliases.insert(alias.alias.clone(), alias).is_some();
                test_not(exists)?;
            }
            Alias8 { aliases }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl BinarySerialize for Alias8<'_> {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), WriteError> {
        writer.write_at_with_ctx(position, &Self::UNK1, &BigEndian)?;
        writer.write_len_type_at::<u32be>(position, &mut self.aliases.values())?;
        Ok(())
    }
}
