#![allow(
    non_camel_case_types,
    reason = "The type aliases need to look like primary types"
)]
use std::marker::PhantomData;

use super::{
    endian::{self, BigEndian, Endianness, LittleEndian},
    len::Len,
    read::{BinaryDeserialize, ReadAtExt, ReadError},
    write::BinarySerialize,
};

impl BinaryDeserialize<'_> for u8 {
    type Ctx = ();
    type Output = Self;

    #[inline(always)]
    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(reader.read_slice_at(position, 1)?[0])
    }
}

impl BinarySerialize for u8 {
    type Ctx = ();
    type Input = Self;

    #[inline(always)]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl super::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), super::write::WriteError> {
        writer.write_slice_at(position, &[input])
    }
}

impl Len<'_> for u8 {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U16<Endian: Endianness>(PhantomData<Endian>);
pub type u16be = U16<BigEndian>;
pub type u16le = U16<LittleEndian>;
impl<Endian: Endianness> BinaryDeserialize<'_> for U16<Endian> {
    type Ctx = ();
    type Output = u16;

    #[inline(always)]
    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 2]>(position)?;
        Endian::to_native(&mut bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}

impl<Endian: Endianness> BinarySerialize for U16<Endian> {
    type Ctx = ();
    type Input = u16;

    #[inline(always)]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl super::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), super::write::WriteError> {
        let mut bytes = input.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

impl<Endian: Endianness> Len<'_> for U16<Endian> {}

pub struct U24<Endian: Endianness>(PhantomData<Endian>);
pub type u24be = U24<BigEndian>;
pub type u24le = U24<LittleEndian>;
impl<Endian: Endianness> BinaryDeserialize<'_> for U24<Endian> {
    type Ctx = ();
    type Output = u32;

    #[inline(always)]
    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 3]>(position)?;
        Endian::to_native(&mut bytes);
        let bytes = endian::pad(bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}

impl<Endian: Endianness> BinarySerialize for U24<Endian> {
    type Ctx = ();
    type Input = u32;

    #[inline(always)]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl super::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), super::write::WriteError> {
        let mut bytes = input.to_ne_bytes();
        Endian::from_native(&mut bytes);
        let bytes: [u8; 3] = endian::unpad(bytes);
        writer.write_slice_at(position, &bytes)
    }
}

impl<Endian: Endianness> Len<'_> for U24<Endian> {}

pub struct U32<Endian: Endianness>(PhantomData<Endian>);
pub type u32be = U32<BigEndian>;
pub type u32le = U32<LittleEndian>;
impl<Endian: Endianness> BinaryDeserialize<'_> for U32<Endian> {
    type Ctx = ();
    type Output = u32;

    #[inline(always)]
    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self::Output, ReadError> {
        let mut bytes = reader.read_at::<[u8; 4]>(position)?;
        Endian::to_native(&mut bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}

impl<Endian: Endianness> BinarySerialize for U32<Endian> {
    type Ctx = ();
    type Input = u32;

    #[inline(always)]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl super::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), super::write::WriteError> {
        let mut bytes = input.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

impl<Endian: Endianness> Len<'_> for U32<Endian> {}

pub struct U64<Endian: Endianness>(PhantomData<Endian>);
pub type u64be = U64<BigEndian>;
pub type u64le = U64<LittleEndian>;
impl<Endian: Endianness> BinaryDeserialize<'_> for U64<Endian> {
    type Ctx = ();
    type Output = u64;

    #[inline(always)]
    fn deserialize_at_with_ctx(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self::Output, ReadError> {
        let mut bytes: [u8; 8] = reader.read_at::<[u8; 8]>(position)?;
        Endian::to_native(&mut bytes);
        Ok(Self::Output::from_ne_bytes(bytes))
    }
}

impl<Endian: Endianness> BinarySerialize for U64<Endian> {
    type Ctx = ();
    type Input = u64;

    #[inline(always)]
    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl super::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), super::write::WriteError> {
        let mut bytes = input.to_ne_bytes();
        Endian::from_native(&mut bytes);
        writer.write_slice_at(position, &bytes)
    }
}

impl<Endian: Endianness> Len<'_> for U64<Endian> {}
