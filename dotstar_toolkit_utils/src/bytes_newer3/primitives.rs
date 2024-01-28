use std::{marker::PhantomData, num::TryFromIntError, sync::Arc};

use super::{
    read::{BinaryDeserialize, ReadError, ZeroCopyReadAt},
    Len,
};

pub trait Endianness {
    fn to_native(bytes: &mut [u8]);
}

pub enum LittleEndian {}

impl Endianness for LittleEndian {
    #[cfg(target_endian = "big")]
    fn to_native(bytes: &mut [u8]) {
        bytes.reverse()
    }

    #[cfg(target_endian = "little")]
    fn to_native(_bytes: &mut [u8]) {}
}

pub enum BigEndian {}

impl Endianness for BigEndian {
    #[cfg(target_endian = "little")]
    fn to_native(bytes: &mut [u8]) {
        bytes.reverse()
    }

    #[cfg(target_endian = "big")]
    fn to_native(_bytes: &mut [u8]) {}
}

pub struct U32<E: Endianness> {
    bytes: [u8; 4],
    byteorder: PhantomData<E>,
}

impl<E: Endianness> TryFrom<usize> for U32<E> {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = u32::try_from(value)?;
        Ok(Self {
            bytes: value.to_ne_bytes(),
            byteorder: PhantomData,
        })
    }
}

impl<E: Endianness> TryFrom<U32<E>> for usize {
    type Error = TryFromIntError;

    fn try_from(value: U32<E>) -> Result<Self, Self::Error> {
        let value = u32::from_ne_bytes(value.bytes);
        usize::try_from(value)
    }
}

impl<'de, E: Endianness> BinaryDeserialize<'de> for U32<E> {
    type Endian = E;

    fn deserialize_at(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let mut bytes = reader.read_fixed_slice_at(position)?;
        E::to_native(bytes.as_mut_slice());
        Ok(U32 {
            bytes,
            byteorder: PhantomData,
        })
    }
}

impl<E: Endianness> From<U32<E>> for u32 {
    fn from(value: U32<E>) -> Self {
        u32::from_ne_bytes(value.bytes)
    }
}

impl<'de, E: Endianness> Len<'de> for U32<E> {}

pub struct U16<E: Endianness> {
    bytes: [u8; 2],
    byteorder: PhantomData<E>,
}

impl<'de, E: Endianness> BinaryDeserialize<'de> for U16<E> {
    type Endian = E;

    fn deserialize_at(
        reader: &Arc<impl ZeroCopyReadAt<'de> + 'de>,
        position: &mut u64,
    ) -> Result<Self, ReadError> {
        let mut bytes = reader.read_fixed_slice_at(position)?;
        E::to_native(bytes.as_mut_slice());
        Ok(U16 {
            bytes,
            byteorder: PhantomData,
        })
    }
}

impl<E: Endianness> From<U16<E>> for u16 {
    fn from(value: U16<E>) -> Self {
        u16::from_ne_bytes(value.bytes)
    }
}
