use std::{convert::Infallible, marker::PhantomData, num::TryFromIntError};

use super::Len;
use crate::bytes::{
    endian::{BigEndian, Endianness, LittleEndian},
    read::BinaryDeserialize,
    write::BinarySerialize,
};

#[repr(transparent)]
pub struct U16<E: Endianness> {
    inner: u16,
    _phantom: PhantomData<E>,
}

#[expect(non_camel_case_types, reason = "Needs to look like a primary type")]
pub type u16be = U16<BigEndian>;
#[expect(non_camel_case_types, reason = "Needs to look like a primary type")]
pub type u16le = U16<LittleEndian>;

impl<E: Endianness> BinaryDeserialize<'_> for U16<E> {
    fn deserialize_at_with_ctx(
        reader: &(impl crate::bytes::read::ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<Self, crate::bytes::read::ReadError> {
        Ok(Self {
            inner: reader.read_at_with_ctx(position, &E::sized())?,
            _phantom: PhantomData,
        })
    }
}

impl<E: Endianness> BinarySerialize for U16<E> {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl crate::bytes::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), crate::bytes::write::WriteError> {
        writer.write_at_with_ctx(position, &self.inner, &E::sized())
    }
}

impl<E: Endianness> TryFrom<usize> for U16<E> {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value.try_into()?,
            _phantom: PhantomData,
        })
    }
}

impl<E: Endianness> TryInto<usize> for U16<E> {
    type Error = Infallible;

    fn try_into(self) -> Result<usize, Self::Error> {
        usize::try_from(self.inner)
    }
}

impl<E: Endianness> Len<'_> for U16<E> {}

#[repr(transparent)]
pub struct U32<E: Endianness> {
    inner: u32,
    _phantom: PhantomData<E>,
}

#[expect(non_camel_case_types, reason = "Needs to look like a primary type")]
pub type u32be = U32<BigEndian>;
#[expect(non_camel_case_types, reason = "Needs to look like a primary type")]
pub type u32le = U32<LittleEndian>;

impl<E: Endianness> BinaryDeserialize<'_> for U32<E> {
    fn deserialize_at_with_ctx(
        reader: &(impl crate::bytes::read::ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<Self, crate::bytes::read::ReadError> {
        Ok(Self {
            inner: reader.read_at_with_ctx(position, &E::sized())?,
            _phantom: PhantomData,
        })
    }
}

impl<E: Endianness> BinarySerialize for U32<E> {
    fn serialize_at_with_ctx(
        &self,
        writer: &mut (impl crate::bytes::write::WriteAt + ?Sized),
        position: &mut u64,
        _ctx: &(),
    ) -> Result<(), crate::bytes::write::WriteError> {
        writer.write_at_with_ctx(position, &self.inner, &E::sized())
    }
}

impl<E: Endianness> TryFrom<usize> for U32<E> {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: value.try_into()?,
            _phantom: PhantomData,
        })
    }
}

impl<E: Endianness> TryInto<usize> for U32<E> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        usize::try_from(self.inner)
    }
}

impl<E: Endianness> Len<'_> for U32<E> {}
