use dotstar_toolkit_utils::{
    bytes::{
        endian::{BigEndian, LittleEndian}, primitives::{u16be, u16le, u32be, u32le, U32}, read::{BinaryDeserialize, ReadAtExt, ReadError}
    },
    testing::test_eq,
};

use crate::cooked::wav::types::{Codec, WavPlatform};

use super::types::Wav;

impl<'de> BinaryDeserialize<'de> for Wav {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &'de (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq(&magic, &Self::MAGIC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        let platform = reader.read_at::<WavPlatform>(position)?;
        let codec = reader.read_at::<Codec>(position)?;

        todo!()
    }
}
