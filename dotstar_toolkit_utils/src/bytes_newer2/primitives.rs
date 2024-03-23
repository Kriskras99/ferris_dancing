use std::marker::PhantomData;

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
    fn to_native(bytes: &mut [u8]) {}
}

pub enum BigEndian {}

impl Endianness for BigEndian {
    #[cfg(target_endian = "little")]
    fn to_native(bytes: &mut [u8]) {
        bytes.reverse()
    }

    #[cfg(target_endian = "big")]
    fn to_native(bytes: &mut [u8]) {}
}

pub struct U32<E: Endianness> {
    bytes: [u8; 4],
    byteorder: PhantomData<E>,
}
