use self::sealed::Sealed;

mod sealed {
    pub trait Sealed {}
}

/// The endianness of a type, for types that are able to be represented in both ways.
///
/// This trait is sealed, it's only implementers are [`LittleEndian`] and [`BigEndian`].
/// There are also two type aliases, [`NativeEndian`] and [`NetworkEndian`].
pub trait Endianness: Sealed + Clone + Copy + std::fmt::Debug + PartialEq {
    fn to_native(bytes: &mut [u8]);

    #[inline(always)]
    fn from_native(bytes: &mut [u8]) {
        Self::to_native(bytes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LittleEndian {}

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
pub enum BigEndian {}
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
pub type NativeEndian = BigEndian;
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;
