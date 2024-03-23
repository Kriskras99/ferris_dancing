use std::marker::PhantomData;

use super::endian::Endianness;



pub struct U16<E: Endianness> {
    bytes: u16,
    byteorder: PhantomData<E>
}

pub struct U24<E: Endianness> {
    bytes: u32,
    byteorder: PhantomData<E>
}
