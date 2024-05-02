//! Integer primitives with a specific endianness

use std::{marker::PhantomData, num::TryFromIntError};

use super::{
    endian::{BigEndian, Endianness, LittleEndian},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
    Len,
};

/// Creates a uint of n bytes
macro_rules! create_uint {
    ( $name:ident, $lename:ident, $bename:ident, $native:ident, $n_bytes:literal ) => {
        #[doc = concat!(r" Unsigned integer type of ", stringify!($n_bytes), r" bytes invariant over [`Endianness`].")]
        #[doc = r""]
        #[doc = r" The alignment of this type is 1 byte."]
        #[doc = concat!(r" Two convenience aliases exist [`", stringify!($lename), r"`] and [`", stringify!($bename), r"`].")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name<E: Endianness> {
            bytes: [u8; $n_bytes],
            byteorder: PhantomData<E>,
        }

        #[allow(non_camel_case_types, reason = "Makes it more like a primitive type")]
        #[doc = concat!(r"Alias for [`", stringify!($name), r"<BigEndian>`].")]
        pub type $bename = $name<BigEndian>;
        #[allow(non_camel_case_types, reason = "Makes it more like a primitive type")]
        #[doc = concat!(r"Alias for [`", stringify!($name), r"<LittleEndian>`].")]
        pub type $lename = $name<LittleEndian>;

        impl<E: Endianness> $name<E> {
            #[must_use]
            /// Checked integer addition. Computes `self + rhs`, returning None if overflow occurred.
            pub fn checked_add(self, rhs: Self) -> Option<Self> {
                $native::from(self).checked_add($native::from(rhs)).and_then(|n| Self::try_from(n).ok())
            }
        }

        impl<E: Endianness> ::std::ops::BitAnd for $name<E> {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                let mut output = self.bytes;
                for i in 0..$n_bytes {
                    output[i] &= rhs.bytes[i];
                }
                Self {
                    bytes: output,
                    byteorder: self.byteorder,
                }
            }
        }

        impl<E: Endianness> ::std::ops::Not for $name<E> {
            type Output = Self;

            fn not(self) -> Self::Output {
                let mut output = self.bytes;
                for i in 0..$n_bytes {
                    output[i] = !output[i];
                }
                Self {
                    bytes: output,
                    byteorder: self.byteorder,
                }
            }
        }

        impl<E: Endianness> TryFrom<$name<E>> for usize {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: $name<E>) -> Result<Self, Self::Error> {
                let value = $native::from(value);
                let value = usize::try_from(value)?;
                Ok(value)
            }
        }

        impl<E: Endianness> TryFrom<usize> for $name<E> {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                let value = $native::try_from(value)?;
                let value = Self::try_from(value)?;
                Ok(value)
            }
        }

        impl<E: Endianness> From<$name<E>> for $native {
            #[inline(always)]
            fn from(value: $name<E>) -> Self {
                let mut new_bytes = [0; std::mem::size_of::<$native>()];
                #[cfg(target_endian = "big")]
                {
                    new_bytes.as_mut_slice().split_at_mut(std::mem::size_of::<$native>() - $n_bytes).1.copy_from_slice(value.bytes.as_slice());
                }
                #[cfg(target_endian = "little")]
                {
                    new_bytes.as_mut_slice().split_at_mut(std::mem::size_of::<$native>() - (std::mem::size_of::<$native>() - $n_bytes)).0.copy_from_slice(value.bytes.as_slice());
                }
                $native::from_ne_bytes(new_bytes)
            }
        }

        impl<'de, E: Endianness> Len<'de> for $name<E> {}

        impl<'de, E: Endianness> BinaryDeserialize<'de> for $name<E> {
            #[inline(always)]
            fn deserialize_at(
                reader: &'de (impl ReadAtExt + ?Sized),
                position: &mut u64,
            ) -> Result<Self, ReadError> {
                let mut bytes = reader.read_fixed_slice_at(position)?;
                E::to_native(bytes.as_mut_slice());
                Ok($name {
                    bytes,
                    byteorder: PhantomData,
                })
            }
        }

        impl<E: Endianness> BinarySerialize for $name<E> {
            #[inline(always)]
            fn serialize_at(
                &self,
                writer: &mut (impl WriteAt + ?Sized),
                position: &mut u64,
            ) -> Result<(), WriteError> {
                let mut bytes = self.bytes.clone();
                E::to_native(&mut bytes);
                writer.write_at(position, &bytes)?;
                Ok(())
            }
        }
    };
}

create_uint!(U16, u16le, u16be, u16, 2);
create_uint!(U24, u24le, u24be, u32, 3);
create_uint!(U32, u32le, u32be, u32, 4);
create_uint!(U40, u40le, u40be, u64, 5);
create_uint!(U48, u48le, u48be, u64, 6);
create_uint!(U56, u56le, u56be, u64, 7);
create_uint!(U64, u64le, u64be, u64, 8);

/// Implements From for types of the same width
macro_rules! impl_pow2_uint {
    ( $name:ident, $native:ident) => {
        impl<E: Endianness> From<$native> for $name<E> {
            #[inline(always)]
            fn from(value: $native) -> Self {
                Self {
                    bytes: value.to_ne_bytes(),
                    byteorder: PhantomData,
                }
            }
        }

        impl<E: Endianness> $name<E> {
            /// Create a new value
            #[must_use]
            pub const fn new(value: $native) -> Self {
                Self {
                    bytes: value.to_ne_bytes(),
                    byteorder: PhantomData,
                }
            }
        }
    };
}

impl_pow2_uint!(U16, u16);
impl_pow2_uint!(U32, u32);
impl_pow2_uint!(U64, u64);

/// Implement try_from for types that are smaller
macro_rules! impl_non_pow2_uint {
    ( $name:ident, $native:ident, $n_bytes:literal, $max:literal) => {
        impl<E: Endianness> TryFrom<$native> for $name<E> {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: $native) -> Result<Self, Self::Error> {
                if value > $max {
                    // Force TryFromIntError creation
                    u8::try_from(1024u16)?;
                }
                let wide_bytes = value.to_ne_bytes();
                let mut bytes = [0; $n_bytes];
                #[cfg(target_endian = "big")]
                #[allow(clippy::arithmetic_side_effects, reason = "Should be `for i in 0..$n_bytes` but that's not yet supported in const")]
                {
                    let mut i = 0;
                    while i < $n_bytes {
                        bytes[i] = wide_bytes[i + std::mem::size_of::<$native>() - $n_bytes];
                        i += 1;
                    }
                }
                #[cfg(target_endian = "little")]
                #[allow(clippy::arithmetic_side_effects, reason = "Should be `for i in 0..$n_bytes` but that's not yet supported in const")]
                {
                    let mut i = 0;
                    while i < $n_bytes {
                        bytes[i] = wide_bytes[i];
                        i += 1;
                    }
                }
                Ok(Self {
                    bytes,
                    byteorder: PhantomData,
                })
            }
        }

        impl<E: Endianness> $name<E> {
            #[must_use]
            /// Create a new value
            pub const fn new(value: $native) -> Self {
                if value > $max {
                    panic!(concat!("value is larger than", stringify!("$max")))
                }
                let wide_bytes = value.to_ne_bytes();
                let mut bytes = [0; $n_bytes];
                #[cfg(target_endian = "big")]
                #[allow(clippy::arithmetic_side_effects, reason = "Should be `for i in 0..$n_bytes` but that's not yet supported in const")]
                {
                    let mut i = 0;
                    while i < $n_bytes {
                        bytes[i] = wide_bytes[i + std::mem::size_of::<$native>() - $n_bytes];
                        i += 1;
                    }
                }
                #[cfg(target_endian = "little")]
                #[allow(clippy::arithmetic_side_effects, reason = "Should be `for i in 0..$n_bytes` but that's not yet supported in const")]
                {
                    let mut i = 0;
                    while i < $n_bytes {
                        bytes[i] = wide_bytes[i];
                        i += 1;
                    }
                }
                Self {
                    bytes,
                    byteorder: PhantomData,
                }
            }
        }
    };
}

impl_non_pow2_uint!(U24, u32, 3, 0x00FF_FFFF);
impl_non_pow2_uint!(U40, u64, 5, 0x00FF_FFFF_FFFF);
impl_non_pow2_uint!(U48, u64, 6, 0xFFFF_FFFF_FFFF);
impl_non_pow2_uint!(U56, u64, 7, 0x00FF_FFFF_FFFF_FFFF);

/// Implements From for wider types
macro_rules! impl_widening_pow2_uint {
    ( $name:ident, $native:ident, $n_bytes:literal ) => {
        impl<E: Endianness> From<$name<E>> for $native {
            #[inline(always)]
            fn from(value: $name<E>) -> Self {
                let mut new_bytes = [0; std::mem::size_of::<$native>()];
                #[cfg(target_endian = "big")]
                {
                    new_bytes
                        .as_mut_slice()
                        .split_at_mut(std::mem::size_of::<$native>() - $n_bytes)
                        .1
                        .copy_from_slice(value.bytes.as_slice());
                }
                #[cfg(target_endian = "little")]
                {
                    new_bytes
                        .as_mut_slice()
                        .split_at_mut(
                            std::mem::size_of::<$native>()
                                - (std::mem::size_of::<$native>() - $n_bytes),
                        )
                        .0
                        .copy_from_slice(value.bytes.as_slice());
                }
                $native::from_ne_bytes(new_bytes)
            }
        }
    };
}

impl_widening_pow2_uint!(U24, u64, 3);
impl_widening_pow2_uint!(U32, u64, 4);
