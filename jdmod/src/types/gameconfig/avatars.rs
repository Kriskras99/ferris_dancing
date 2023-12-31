//! # Avatars
//! Types for dealing with avatars
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use dotstar_toolkit_utils::testing::{test, test_any, test_not, TestResult};
use serde::{Deserialize, Serialize};

/// Description of an avatar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar<'a> {
    /// Which map this avatar is based on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_song_name: Option<Cow<'a, str>>,
    /// The sound bites it uses
    pub sound_family: Cow<'a, str>,
    /// Unknown
    pub status: u8,
    /// How to unlock
    pub unlock_type: UnlockType<'a>,
    /// Which map coach is this avatar based on
    pub used_as_coach_map_name: Cow<'a, str>,
    /// Which specific coach in the map
    pub used_as_coach_coach_id: u8,
    /// Should this avatar have the special foil effect
    pub special_effect: bool,
    /// Name of the normal variant of this avatar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main_avatar: Option<Cow<'a, str>>,
    /// Path to the texture
    pub image_path: Cow<'a, str>,
    /// Path to the phone image
    pub image_phone_path: Cow<'a, str>,
}

/// How to unlock a avatar
#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnlockType<'a> {
    /// Information is missing
    Unknown,
    /// Gacha machine
    GiftMachine,
    /// Have a save file from a previous Just Dance
    PlayPreviousJD,
    /// Always unlocked
    Unlocked,
    /// Complete a quest
    Quest(Cow<'a, str>),
    /// Have unlimited
    Unlimited,
}

impl From<&UnlockType<'_>> for u8 {
    fn from(value: &UnlockType) -> Self {
        match value {
            UnlockType::Unknown => 0,
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
    pub fn from_unlock_type(n: u8, quest: Option<&Cow<'a, str>>) -> Result<Self, Error> {
        TestResult::or(
            TestResult::and(test(&quest.is_some(), &true), test_any(&n, &[0, 21])),
            TestResult::and(test(&quest.is_none(), &true), test_not(&n, &21)),
        )?;
        match n {
            0 => match quest {
                Some(quest) => Ok(Self::Quest(quest.clone())),
                None => Ok(Self::Unknown),
            },
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
    pub fn normalize(self) -> Self {
        match self {
            Self::GiftMachine => Self::GiftMachine,
            Self::PlayPreviousJD | Self::Unlimited | Self::Unlocked | Self::Unknown => {
                Self::Unlocked
            }
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
