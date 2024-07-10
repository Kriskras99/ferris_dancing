use std::fmt::Display;

use dotstar_toolkit_utils::bytes::{primitives::u32be, read::{BinaryDeserialize, ReadAtExt, ReadError}};
use serde::{Serialize, Deserialize};

pub mod errors;


/// A RGBA color encoded in f32 (0.0 is black, 1.0 is white)
pub type Color = (f32, f32, f32, f32);

/// Represents the id of a localised string
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct LocaleId(u32);

impl BinaryDeserialize<'_> for LocaleId {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<Self, ReadError> {
        Ok(Self(reader.read_at::<u32be>(position)?))
    }
}

impl Default for LocaleId {
    fn default() -> Self {
        Self(u32::MAX)
    }
}

impl Display for LocaleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x}", self.0))
    }
}

impl LocaleId {
    /// The empty/missing LocaleId
    pub const EMPTY: Self = Self(u32::MAX);
    /// The minimum value of a LocaleId
    pub const MIN: Self = Self(0);

    /// Increments the locale id and returns a new higher locale id
    ///
    /// # Panics
    /// Will panic if the increment would cause an overflow
    #[must_use]
    pub fn increment(&self) -> Self {
        Self(self.0.checked_add(1).unwrap())
    }
}

impl From<u32> for LocaleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<LocaleId> for u32 {
    fn from(value: LocaleId) -> Self {
        value.0
    }
}
