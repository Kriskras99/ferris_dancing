//! Endianness types for reading and writing bytes in the right endianness

use self::sealed::Sealed;

/// Module to make the `Sealed` trait unimplementable
mod sealed {
    /// Trait that can't be implemented by anyone outside this crate
    pub trait Sealed {}
}

/// The endianness of a type, for types that are able to be represented in both ways.
///
/// This trait is sealed, it's only implementers are [`LittleEndian`] and [`BigEndian`].
/// There are also two type aliases, [`NativeEndian`] and [`NetworkEndian`].
pub trait Endianness: Sealed + Clone + Copy + std::fmt::Debug + PartialEq {
    /// Convert `bytes` to the native endianness
    fn to_native(bytes: &mut [u8]);

    #[inline(always)]
    /// Convert `bytes` from the native endianness
    fn from_native(bytes: &mut [u8]) {
        Self::to_native(bytes);
    }

    fn sized() -> impl Endianness;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The least significant byte is at the smallest address
pub struct LittleEndian;

impl Sealed for LittleEndian {}
impl Endianness for LittleEndian {
    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_native(bytes: &mut [u8]) {
        bytes.reverse();
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_native(_bytes: &mut [u8]) {}

    #[expect(refining_impl_trait, reason = "It's better")]
    fn sized() -> Self {
        Self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The most siginficant byte is at the smallest address
pub struct BigEndian;
/// The endianness used for network communication
pub type NetworkEndian = BigEndian;

impl Sealed for BigEndian {}
impl Endianness for BigEndian {
    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn to_native(bytes: &mut [u8]) {
        bytes.reverse();
    }

    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn to_native(_bytes: &mut [u8]) {}

    #[expect(refining_impl_trait, reason = "It's better")]
    fn sized() -> Self {
        Self
    }
}

#[cfg(target_endian = "big")]
/// The endianness of the system the program is running on
pub type NativeEndian = BigEndian;
#[cfg(target_endian = "little")]
/// The endianness of the system the program is running on
pub type NativeEndian = LittleEndian;
