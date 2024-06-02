use std::marker::PhantomData;

use dotstar_toolkit_utils::{
    bytes::{
        endian::{BigEndian, Endianness, LittleEndian},
        primitives::{u16be, u16le, u32be, u32le, U16, U32},
        read::{BinaryDeserialize, BinaryDeserializeExt as _, ReadAtExt, ReadError},
    },
    testing::test_eq,
};

use super::types::Wav;
use crate::cooked::wav::types::{Codec, WavPlatform};

impl BinaryDeserialize<'_> for Wav {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let magic = reader.read_at::<u32be>(position)?;
        test_eq(magic, Self::MAGIC)?;
        let unk1 = reader.read_at::<u32be>(position)?;
        test_eq(unk1, 0)?;
        let platform = reader.read_at::<WavPlatform>(position)?;
        let codec = reader.read_at::<Codec>(position)?;

        match platform {
            WavPlatform::Wii | WavPlatform::WiiU | WavPlatform::PS3 | WavPlatform::X360 => reader.read_at_with_ctx::<WavInner<BigEndian>>(position, (platform, codec)),
            _ => reader.read_at_with_ctx::<WavInner<LittleEndian>>(position, (platform, codec))
        }
    }
}

struct WavInner<Endian: Endianness>(PhantomData<Endian>);

impl<E: Endianness> BinaryDeserialize<'_> for WavInner<E> {
    type Ctx = (WavPlatform, Codec);
    type Output = Wav;

    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let (platform, codec) = ctx;
        let header_size = reader.read_at::<U32<E>>(position)?;
        let start_offset = reader.read_at::<U32<E>>(position)?;
        let number_of_chunks = reader.read_at::<U32<E>>(position)?;
        let unk2 = reader.read_at::<U32<E>>(position)?;
        let fmt_magic = reader.read_at::<u32be>(position)?;
        test_eq(fmt_magic, Wav::FMT_MAGIC)?;
        todo!()
    }
}
