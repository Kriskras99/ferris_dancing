//! # Portraitborders
//! Describes portraitborders, called skins in the UI.
//! However, these are not the same skins as in earlier games
use std::borrow::Cow;

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use ubiart_toolkit::json_types::PortraitBorderDesc;

use super::generate_gacha_id;

/// Is the portraitborder unlocked by default
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LockStatus {
    /// Yes
    UnlockedByDefault,
    /// No, unlocked by the gacha machine
    GachaMachine,
}

impl TryFrom<u8> for LockStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GachaMachine),
            1 => Ok(Self::UnlockedByDefault),
            _ => Err(anyhow!("Unknown lock status type! {value}")),
        }
    }
}

impl From<LockStatus> for u8 {
    fn from(value: LockStatus) -> Self {
        match value {
            LockStatus::UnlockedByDefault => 1,
            LockStatus::GachaMachine => 0,
        }
    }
}

/// Is the portraitborder selectable by the user
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Visibility {
    /// Yes
    Visible,
    /// No (used for guest only portraitborders)
    Hidden,
}

impl TryFrom<u8> for Visibility {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Hidden),
            1 => Ok(Self::Visible),
            _ => Err(anyhow!("Unknown visibility type! {value}")),
        }
    }
}

impl From<Visibility> for u8 {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Visible => 1,
            Visibility::Hidden => 0,
        }
    }
}

/// Describes a portraitborder
#[derive(Debug, Serialize, Deserialize)]
pub struct PortraitBorder<'a> {
    /// Path to the image that should be behind the avatar
    pub background_texture_path: Cow<'a, str>,
    /// Path to the image that should be in front of the avatar
    pub foreground_texture_path: Option<Cow<'a, str>>,
    /// Path to the image that should be behind the avatar (phone)
    pub background_phone_path: Cow<'a, str>,
    /// Path to the image that should be in front of the avatar (phone)
    pub foreground_phone_path: Option<Cow<'a, str>>,
    /// Is the portraitborder unlocked by default
    pub lock_status: LockStatus,
    /// Is the portraitborder selectable by the user
    pub visibility: Visibility,
}

impl<'a> PortraitBorder<'a> {
    /// Convert to the mod representation
    pub fn from_portrait_border_desc(
        desc: &PortraitBorderDesc<'a>,
        name: &str,
    ) -> Result<Self, Error> {
        assert!(
            !desc.background_phone_path.is_empty(),
            "Background phone image does not exist!"
        );
        assert!(
            !desc.background_texture_path.is_empty(),
            "Background texture does not exist!"
        );
        Ok(Self {
            background_texture_path: Cow::Owned(format!("{name}/background_texture.png")),
            foreground_texture_path: if desc.foreground_texture_path.is_empty() {
                None
            } else {
                Some(Cow::Owned(format!("{name}/foreground_texture.png")))
            },
            background_phone_path: Cow::Owned(format!("{name}/background_phone.png")),
            foreground_phone_path: if desc.foreground_phone_path.is_empty() {
                None
            } else {
                Some(Cow::Owned(format!("{name}/foreground_phone.png")))
            },
            lock_status: LockStatus::try_from(desc.original_lock_status)?,
            visibility: Visibility::try_from(desc.visibility)?,
        })
    }

    /// Convert to the UbiArt representation
    pub fn to_portrait_border_desc(&self, name: &'a str) -> PortraitBorderDesc<'a> {
        let id = generate_gacha_id();
        PortraitBorderDesc {
            class: Some(PortraitBorderDesc::CLASS),
            portrait_border_id: id,
            background_texture_path: Cow::Owned(format!(
                "world/features/collectibles/portraitborders/{id:04}_{name}/pb_back.png"
            )),
            foreground_texture_path: if self.foreground_texture_path.is_some() {
                Cow::Owned(format!(
                    "world/features/collectibles/portraitborders/{id:04}_{name}/pb_front.png"
                ))
            } else {
                Cow::Borrowed("")
            },
            background_phone_path: Cow::Owned(format!(
                "world/features/collectibles/portraitborders/{id:04}_{name}/pb_back_phone.png"
            )),
            foreground_phone_path: if self.foreground_phone_path.is_some() {
                Cow::Owned(format!(
                    "world/features/collectibles/portraitborders/{id:04}_{name}/pb_front_phone.png"
                ))
            } else {
                Cow::Borrowed("")
            },
            original_lock_status: self.lock_status.into(),
            visibility: self.visibility.into(),
        }
    }
}
