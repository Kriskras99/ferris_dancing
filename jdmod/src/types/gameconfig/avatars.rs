//! # Avatars
//! Types for dealing with avatars

use anyhow::{anyhow, Error};
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use test_eq::{test_and, test_any, test_eq, test_ne, test_or};

/// For serde to set a value to default to `false`
const fn be_false() -> bool {
    false
}

/// Description of an avatar
#[derive(Debug, Clone, Serialize, Deserialize, IntoOwned)]
pub struct Avatar<'a> {
    /// The ID of this avatar, if `None` will be generated
    pub id: Option<u32>,
    /// Which map this avatar is based on
    #[serde(borrow)]
    pub relative_song_name: HipStr<'a>,
    /// The sound bites it uses
    #[serde(borrow)]
    pub sound_family: HipStr<'a>,
    /// Unknown
    pub status: u32,
    /// How to unlock
    #[serde(borrow)]
    pub unlock_type: UnlockType<'a>,
    /// Which map coach is this avatar based on
    #[serde(borrow)]
    pub used_as_coach_map_name: HipStr<'a>,
    /// Which specific coach in the map
    pub used_as_coach_coach_id: u32,
    /// Should this avatar have the special foil effect
    pub special_effect: bool,
    /// Name of the normal variant of this avatar
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub main_avatar: Option<HipStr<'a>>,
    /// Path to the texture
    #[serde(borrow)]
    pub image_path: HipStr<'a>,
    /// Are the sound effect and image phone path a guess?
    // if it's missing it's not guessed, don't serialize if false
    #[serde(default = "be_false", skip_serializing_if = "std::ops::Not::not")]
    pub guessed: bool,
}

/// How to unlock a avatar
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, IntoOwned)]
pub enum UnlockType<'a> {
    /// Information is missing
    Unknown,
    /// only in 2017
    Unknown1,
    /// only in 2017
    Unknown3,
    /// only in 2020 so maybe anniversary?
    Unknown6,
    /// only in 2017
    Unknown9,
    /// only in 2016
    Unknown10,
    /// only in 2017-2018 so maybe Dance Quest?
    Unknown11,
    /// only on WiiU
    Unknown16,
    /// Gacha machine
    GiftMachine,
    /// Have a save file from a previous Just Dance
    PlayPreviousJD,
    /// Always unlocked
    Unlocked,
    /// Complete a quest
    #[serde(borrow)]
    Quest(HipStr<'a>),
    /// Have unlimited
    Unlimited,
}

impl From<&UnlockType<'_>> for u32 {
    fn from(value: &UnlockType) -> Self {
        match value {
            UnlockType::Unknown => 0,
            UnlockType::Unknown1 => 1,
            UnlockType::Unknown3 => 3,
            UnlockType::Unknown6 => 6,
            UnlockType::Unknown9 => 9,
            UnlockType::Unknown10 => 10,
            UnlockType::Unknown11 => 11,
            UnlockType::Unknown16 => 16,
            UnlockType::GiftMachine => 18,
            UnlockType::PlayPreviousJD => 19,
            UnlockType::Unlocked => 20,
            UnlockType::Quest(_) => 21,
            UnlockType::Unlimited => 22,
        }
    }
}

impl<'a> UnlockType<'a> {
    /// Convert from the UbiArt representation
    ///
    /// # Errors
    /// Will error if the quest type is unknown or a quest name is required for a quest type but missing
    pub fn from_unlock_type(n: u32, quest: Option<&HipStr<'a>>) -> Result<Self, Error> {
        test_or!(
            test_and!(test_eq!(quest.is_some(), true), test_any!(n, [0, 21])),
            test_and!(test_eq!(quest.is_none(), true), test_ne!(n, 21)),
        )?;
        match n {
            0 => match quest {
                Some(quest) => Ok(Self::Quest(quest.clone())),
                None => Ok(Self::Unknown),
            },
            1 => Ok(Self::Unknown1),
            3 => Ok(Self::Unknown3),
            6 => Ok(Self::Unknown6),
            9 => Ok(Self::Unknown9),
            10 => Ok(Self::Unknown10),
            11 => Ok(Self::Unknown11),
            16 => Ok(Self::Unknown16),
            18 => Ok(Self::GiftMachine),
            19 => Ok(Self::PlayPreviousJD),
            20 => Ok(Self::Unlocked),
            21 => {
                Ok(Self::Quest(quest.cloned().ok_or_else(|| {
                    anyhow!("No quest name despite quest type!")
                })?))
            }
            22 => Ok(Self::Unlimited),
            _ => Err(anyhow!("Unknown unlock type {n}!")),
        }
    }

    /// Normalize the unlock type to [`UnlockType::Quest`] or [`UnlockType::Unlocked`]
    #[must_use]
    pub fn normalize(self) -> Self {
        match self {
            Self::GiftMachine => Self::GiftMachine,
            Self::PlayPreviousJD
            | Self::Unlimited
            | Self::Unlocked
            | Self::Unknown
            | Self::Unknown1
            | Self::Unknown3
            | Self::Unknown6
            | Self::Unknown9
            | Self::Unknown10
            | Self::Unknown11
            | Self::Unknown16 => Self::Unlocked,
            Self::Quest(s) => Self::Quest(s),
        }
    }
}

/*
unlock types:
0: Not used/Or missing metadata
18: Gift Machine
19: Play previous Just Dance game
20: Unlocked at the start
21:
    RelativeSongName not empty -> Dance to unlock
    mainAvatarId not u16::MAX -> Megastar
22: Just Dance Unlimited
*/
