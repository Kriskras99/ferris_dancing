use dotstar_toolkit_utils::{
    bytes::{
        primitives::u32be,
        read::{BinaryDeserialize, ReadAtExt, ReadError},
    },
    test_eq,
};
use hipstr::HipStr;

use crate::utils::UniqueGameId;

pub struct DlcDescriptor<'a> {
    pub name: HipStr<'a>,
}

impl<'de> BinaryDeserialize<'de> for DlcDescriptor<'de> {
    type Ctx = UniqueGameId;
    type Output = Self;

    fn deserialize_at_with(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq!(magic, u32::from_be_bytes(*b"JDLC"))?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq!(unk1, 0x1)?;
        let unk2 = reader.read_at::<u32be>(position)?;
        test_eq!(unk2, 0x07DF)?;
        let unk3 = reader.read_at::<u32be>(position)?;
        test_eq!(unk3, 0x1)?;
        let name = reader.read_len_string_at::<u32be>(position)?.into();
        Ok(Self { name })
    }
}
