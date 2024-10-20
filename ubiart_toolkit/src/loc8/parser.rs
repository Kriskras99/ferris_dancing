//! Contains the parser implementation

use std::collections::{hash_map::Entry, HashMap};

use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_any,
};

use super::types::Loc8;
use crate::{loc8::types::Language, utils::LocaleId};

impl<'de> BinaryDeserialize<'de> for Loc8<'de> {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let unk1 = reader.read_at::<u32be>(position)?;
        let language = reader.read_at::<Language>(position)?;

        // When unk1 == 2 there's a second version of strings
        // However these alternative strings seem to be riddled with typos, so just use the first one
        test_any!(unk1, [1, 2])?;

        let mut strings = HashMap::new();

        for i in 0..unk1 {
            let string_count = reader.read_at::<u32be>(position)?;

            for _ in 0..string_count {
                let id = reader.read_at::<LocaleId>(position)?;
                let string = reader.read_len_string_lossy_at::<u32be>(position)?;

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

        if let Ok(footer) = reader.read_at::<[u8; 100]>(position) {
            if test_any!(footer, Loc8::FOOTERS).is_err() {
                println!("Warning! Unexpected footer in loc8 file: {footer:x?}",);
            }
        } else {
            println!("Footer is too small!");
        }

        Ok(Loc8 { language, strings })
    }
}

impl BinaryDeserialize<'_> for Language {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        let value: u32 = reader.read_at::<u32be>(position)?;
        Self::try_from(value).map_err(|e| ReadError::custom(e.to_string()))
    }
}
