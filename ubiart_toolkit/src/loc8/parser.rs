//! Contains the parser implementation

use std::collections::{hash_map::Entry, HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadError, ZeroCopyReadAtExt},
    },
    testing::test_any,
};

use super::types::Loc8;
use crate::{loc8::types::Language, utils::LocaleId};

impl<'de> BinaryDeserialize<'de> for Loc8<'de> {
    fn deserialize_at(
        reader: &'de (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?.into();
        let language = reader.read_at::<Language>(position)?;

        // When unk1 == 2 there's a second version of strings
        // However these alternative strings seem to be riddled with typos, so just use the first one
        test_any(&unk1, &[1u32, 2])?;

        let mut strings = HashMap::new();

        for i in 0..unk1 {
            let string_count: u32 = reader.read_at::<u32be>(position)?.into();

            for _ in 0..string_count {
                let id = reader.read_at::<LocaleId>(position)?;
                let string = reader.read_len_string_at::<u32be>(position)?;

                if i == 0 {
                    strings.insert(id, string);
                } else if i == 1 {
                    // Only insert new strings the second time around
                    if let Entry::Vacant(e) = strings.entry(id) {
                        e.insert(string);
                    }
                }
            }

            let _unk2 = reader.read_at::<u32be>(position)?;
        }

        let footer: [u8; 100] = reader.read_fixed_slice_at(position)?;
        if test_any(&footer, Loc8::FOOTERS).is_err() {
            println!("Warning! Unexpected footer in loc8 file: {footer:x?}",);
        }

        Ok(Loc8 { language, strings })
    }
}

impl BinaryDeserialize<'_> for Language {
    fn deserialize_at(
        reader: &'_ (impl ZeroCopyReadAtExt + ?Sized),
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let value: u32 = reader.read_at::<u32be>(position)?.into();
        match value {
            0x0 => Ok(Self::English),
            0x1 => Ok(Self::French),
            0x2 => Ok(Self::Japanese),
            0x3 => Ok(Self::German),
            0x4 => Ok(Self::Spanish),
            0x5 => Ok(Self::Italian),
            0x6 => Ok(Self::Korean),
            0x7 => Ok(Self::TradChinese),
            0x8 => Ok(Self::Portuguese),
            0x9 => Ok(Self::SimplChinese),
            0xB => Ok(Self::Russian),
            0xC => Ok(Self::Dutch),
            0xD => Ok(Self::Danish),
            0xE => Ok(Self::Norwegian),
            0xF => Ok(Self::Swedish),
            0x10 => Ok(Self::Finnish),
            0x16 => Ok(Self::GavChinese),
            0x17 => Ok(Self::DevReference),
            _ => Err(ReadError::custom(format!("Unknown language: {value:x}"))),
        }
    }
}
