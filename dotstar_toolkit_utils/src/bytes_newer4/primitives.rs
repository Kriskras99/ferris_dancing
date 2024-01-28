use std::{marker::PhantomData, num::TryFromIntError};

use super::{
    endian::{BigEndian, Endianness, LittleEndian},
    read::{BinaryDeserialize, ReadError, ZeroCopyReadAt},
    write::{BinarySerialize, WriteError, ZeroCopyWriteAt},
    Len,
};

macro_rules! create_uint {
    ( $name:ident, $lename:ident, $bename:ident, $native:ident, $n_bytes:literal, $max:literal ) => {
        //#[doc = concat!("You can call this as `myfoo(", stringify!($name), ")`.")]
        #[doc = concat!(r" Unsigned integer type of ", stringify!($n_bytes), r" bytes invariant over [`Endianness`].")]
        #[doc = r""]
        #[doc = r" The alignment of this type is 1 byte."]
        #[doc = concat!(r" Two convenience aliases exist [`", stringify!($lename), r"`] and [`", stringify!($bename), r"`].")]
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
                Self::try_from(value)
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

        impl<E: Endianness> TryFrom<$native> for $name<E> {
            type Error = TryFromIntError;

            #[inline(always)]
            fn try_from(value: $native) -> Result<Self, Self::Error> {
                #[allow(unused_comparisons, reason = "Needed for u24, u40, u48, u56")]
                if value > $max {
                    // Force TryFromIntError creation
                    u8::try_from(1024u16)?;
                }
                let wide_bytes = value.to_ne_bytes();
                let mut bytes = [0; $n_bytes];
                #[cfg(target_endian = "big")]
                {
                    bytes.as_mut_slice().copy_from_slice(
                        wide_bytes.as_slice().split_at(std::mem::size_of::<$native>() - $n_bytes).1
                    )
                }
                #[cfg(target_endian = "little")]
                {
                    bytes.as_mut_slice().copy_from_slice(
                        wide_bytes.as_slice().split_at(std::mem::size_of::<$native>() - (std::mem::size_of::<$native>() - $n_bytes)).0
                    )
                }
                Ok(Self {
                    bytes,
                    byteorder: PhantomData,
                })
            }
        }

        impl<'de, E: Endianness> Len<'de> for $name<E> {}

        impl<'de, E: Endianness> BinaryDeserialize<'de> for $name<E> {
            #[inline(always)]
            fn deserialize_at(
                reader: &impl ZeroCopyReadAt<'de>,
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
                writer: &mut (impl ZeroCopyWriteAt + ?Sized),
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

create_uint!(U16, u16le, u16be, u16, 2, 0xFFFF);
create_uint!(U24, u24le, u24be, u32, 3, 0xFFFF_FF);
create_uint!(U32, u32le, u32be, u32, 4, 0xFFFF_FFFF);
create_uint!(U40, u40le, u40be, u64, 5, 0xFFFF_FFFF_FF);
create_uint!(U48, u48le, u48be, u64, 6, 0xFFFF_FFFF_FFFF);
create_uint!(U56, u56le, u56be, u64, 7, 0xFFFF_FFFF_FFFF_FF);
create_uint!(U64, u64le, u64be, u64, 8, 0xFFFF_FFFF_FFFF_FFFF);
