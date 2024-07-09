//! Endianness types for reading and writing bytes in the right endianness

/// Pad the byte array to a larger size, while keeping the endianness correct
///
/// # Panics
/// Will panic if `N` is larger than `M`
#[must_use]
#[inline]
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
#[inline]
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

/// The byte order of a slice of bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Endian {
    Little,
    Big,
}

impl Endian {
    /// Change the endianness of the bytes to match the host
    ///
    /// Assumes the byte slice is an n-byte integer
    #[inline]
    pub fn to_native(&self, bytes: &mut [u8]) {
        #[cfg(target_endian = "little")]
        if matches!(self, Self::Big) {
            bytes.reverse();
        }
        #[cfg(target_endian = "big")]
        if matches!(self, Self::Little) {
            bytes.reverse();
        }
    }

    /// Change the endianness of the bytes to match the target endian
    ///
    /// Assumes the byte slice is an n-byte integer
    #[inline]
    pub fn from_native(&self, bytes: &mut [u8]) {
        self.to_native(bytes);
    }
}

pub const LE: Endian = Endian::Little;
pub const BE: Endian = Endian::Big;
