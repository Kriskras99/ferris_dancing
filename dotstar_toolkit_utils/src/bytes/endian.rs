//! Endianness types for reading and writing bytes in the right endianness

use self::sealed::Sealed;

/// Module to make the `Sealed` trait unimplementable
mod sealed {
    /// Trait that can't be implemented by anyone outside this crate
    pub trait Sealed {}
}

/// Pad the byte array to a larger size, while keeping the endianness correct
///
/// # Panics
/// Will panic if `N` is larger than `M`
#[must_use]
#[inline(always)]
pub fn pad<const N: usize, const M: usize>(bytes: [u8; N]) -> [u8; M] {
    assert!(N <= M, "Cannot pad to a smaller size!");
    let mut new = [0; M];
    #[cfg(target_endian = "big")]
    {
        // left pad with zeroes
        let offset = M - N;
        new[offset..].copy_from_slice(&bytes);
    }
    #[cfg(target_endian = "little")]
    {
        // right pad with zeroes
        new[..N].copy_from_slice(&bytes);
    }
    new
}

/// Unpad the byte array, while keeping the endianness correct
/// # Panics
/// Will panic if `N` is smaller than `M`
/// Will panic if the padding is not zero
#[must_use]
#[inline(always)]
pub fn unpad<const N: usize, const M: usize>(bytes: [u8; N]) -> [u8; M] {
    assert!(N >= M, "Cannot unpad to a bigger size!");
    let mut new = [0; M];
    #[cfg(target_endian = "big")]
    {
        // remove the zeroes on the left
        let offset = N - M;
        new.copy_from_slice(&bytes[offset..]);
        assert!(
            bytes.into_iter().take(offset).all(|b| b == 0),
            "Padding is not zero!"
        );
    }
    #[cfg(target_endian = "little")]
    {
        // remove the zeroes on the right
        new.copy_from_slice(&bytes[..N]);
        assert!(
            bytes.into_iter().skip(N).all(|b| b == 0),
            "Padding is not zero!"
        );
    }
    new
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
}

#[cfg(target_endian = "big")]
/// The endianness of the system the program is running on
pub type NativeEndian = BigEndian;
#[cfg(target_endian = "little")]
/// The endianness of the system the program is running on
pub type NativeEndian = LittleEndian;
