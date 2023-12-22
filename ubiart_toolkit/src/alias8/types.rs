//! Contains the types that describe the usefull information in this filetype

use std::borrow::Cow;

use byteorder::ByteOrder;
use dotstar_toolkit_utils::{
    bytes_new::{BinaryDeserialize, NewReadError, ZeroCopyReadAt},
    testing::{test, test_any},
};

use crate::utils::SplitPath;

/// Describes a single alias
#[derive(Debug, Clone)]
pub struct Alias<'a> {
    /// The first alias name
    pub first_alias: Cow<'a, str>,
    /// The second alias name
    pub second_alias: Cow<'a, str>,
    /// The (uncooked) path for the alias
    pub path: SplitPath<'a>,
    /// Unknown value
    pub unk3: u16,
}

impl<'a> BinaryDeserialize<'a> for Alias<'a> {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'a> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            // Read the strings
            let first_alias = reader.read_len_string_at::<B, u32>(position)?;
            let second_alias = reader.read_len_string_at::<B, u32>(position)?;
            let path = reader.read_type_at::<B, _>(position)?;

            // Read the unknown values and check them
            let unk2 = reader.read_u16_at::<B>(position)?;
            let unk3 = reader.read_u16_at::<B>(position)?;
            test(&unk2, &Self::UNK2)?;
            test_any(&unk3, Self::UNK3)?;
            Self {
                first_alias,
                second_alias,
                path,
                unk3,
            }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl Alias<'_> {
    /// UNK2 is always u16::MAX
    const UNK2: u16 = 0xFFFF;
    /// Possible values for UNK3
    const UNK3: &'static [u16] = &[
        0x8001, 0x8002, 0x8008, 0x8100, 0x83D6, 0x8400, 0x8800, 0x9000, 0xA000, 0xC000, 0xE001,
        0xE002, 0xE008, 0xE100, 0xE400, 0xE800, 0xEFDF, 0xF000, 0xF001, 0xF002, 0xF008, 0xF100,
        0xF400, 0xF800, 0xFC08, 0xFD19, 0xFFDF, 0xFFFF,
    ];
}

/// Describes the entire file
#[derive(Debug, Clone)]
pub struct Alias8<'a> {
    /// The aliases in this file
    pub aliases: Vec<Alias<'a>>,
}

impl<'a> BinaryDeserialize<'a> for Alias8<'a> {
    fn deserialize_at<B>(
        reader: &(impl ZeroCopyReadAt<'a> + ?Sized),
        position: &mut u64,
    ) -> Result<Self, NewReadError>
    where
        B: ByteOrder,
    {
        let old_position = *position;
        let result: Result<_, _> = try {
            // Read the unknown value at the beginning and check it's correct
            let unk1 = reader.read_u32_at::<B>(position)?;
            test(&unk1, &0x2)?;

            // Read the aliases
            let aliases = reader.read_len_type_at::<B, u32, _>(position)?;
            Alias8 { aliases }
        };
        if result.is_err() {
            *position = old_position;
        }
        result
    }
}

impl Alias8<'_> {
    /// Find the path for a given alias
    #[must_use]
    pub fn get_path_for_alias(&self, alias: &str) -> Option<String> {
        for a in &self.aliases {
            if a.first_alias == alias || a.second_alias == alias {
                return Some(a.path.to_string());
            }
        }
        None
    }
}
