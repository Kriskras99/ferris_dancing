//! # Portraitborders
//! Describes portraitborders, called skins in the UI.
//! However, these are not the same skins as in earlier games

use anyhow::{anyhow, bail, Error};
use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::json_types::isg::PortraitBorderDesc;

use super::generate_gacha_id;

/// Is the portraitborder unlocked by default
#[derive(Debug, Serialize, Deserialize, Clone, Copy, IntoOwned)]
pub enum LockStatus {
    /// Yes
    UnlockedByDefault,
    /// No, unlocked by the gacha machine
    GachaMachine,
}

impl TryFrom<u32> for LockStatus {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GachaMachine),
            1 => Ok(Self::UnlockedByDefault),
            _ => Err(anyhow!("Unknown lock status type! {value}")),
        }
    }
}

impl From<LockStatus> for u32 {
    fn from(value: LockStatus) -> Self {
        match value {
            LockStatus::UnlockedByDefault => 1,
            LockStatus::GachaMachine => 0,
        }
    }
}

/// Is the portraitborder selectable by the user
#[derive(Debug, Serialize, Deserialize, Clone, Copy, IntoOwned)]
pub enum Visibility {
    /// Yes
    Visible,
    /// No (used for guest only portraitborders)
    Hidden,
}

impl TryFrom<u32> for Visibility {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Hidden),
            1 => Ok(Self::Visible),
            _ => Err(anyhow!("Unknown visibility type! {value}")),
        }
    }
}

impl From<Visibility> for u32 {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Visible => 1,
            Visibility::Hidden => 0,
        }
    }
}

/// Describes a portraitborder
#[derive(Debug, Serialize, Deserialize, IntoOwned)]
pub struct PortraitBorder<'a> {
    /// Path to the image that should be behind the avatar
    #[serde(borrow)]
    pub background_texture_path: HipStr<'a>,
    /// Path to the image that should be in front of the avatar
    #[serde(borrow)]
    pub foreground_texture_path: Option<HipStr<'a>>,
    /// Path to the image that should be behind the avatar (phone)
    #[serde(borrow)]
    pub background_phone_path: HipStr<'a>,
    /// Path to the image that should be in front of the avatar (phone)
    #[serde(borrow)]
    pub foreground_phone_path: Option<HipStr<'a>>,
    /// Is the portraitborder unlocked by default
    pub lock_status: LockStatus,
    /// Is the portraitborder selectable by the user
    pub visibility: Visibility,
}

impl<'a> PortraitBorder<'a> {
    /// Convert to the mod representation
    ///
    /// # Errors
    /// Will error if `desc` has invalid values
    pub fn from_portrait_border_desc(
        desc: &PortraitBorderDesc<'a>,
        name: &str,
    ) -> Result<Self, Error> {
        if desc.background_phone_path.is_empty() {
            bail!("Background phone image dooes not exist!");
        }
        if desc.background_texture_path.is_empty() {
            bail!("Background texture does not exist!");
        };
        Ok(Self {
            background_texture_path: HipStr::from(format!("{name}/background_texture.png")),
            foreground_texture_path: if desc.foreground_texture_path.is_empty() {
                None
            } else {
                Some(HipStr::from(format!("{name}/foreground_texture.png")))
            },
            background_phone_path: HipStr::from(format!("{name}/background_phone.png")),
            foreground_phone_path: if desc.foreground_phone_path.is_empty() {
                None
            } else {
                Some(HipStr::from(format!("{name}/foreground_phone.png")))
            },
            lock_status: LockStatus::try_from(desc.original_lock_status)?,
            visibility: Visibility::try_from(desc.visibility)?,
        })
    }

    /// Convert to the UbiArt representation
    #[must_use]
    pub fn to_portrait_border_desc(&self, name: &'a str) -> PortraitBorderDesc<'a> {
        let id = generate_gacha_id();
        PortraitBorderDesc {
            class: Some(PortraitBorderDesc::CLASS),
            portrait_border_id: id,
            background_texture_path: HipStr::from(format!(
                "world/features/collectibles/portraitborders/{id:04}_{name}/pb_back.png"
            )),
            foreground_texture_path: if self.foreground_texture_path.is_some() {
                HipStr::from(format!(
                    "world/features/collectibles/portraitborders/{id:04}_{name}/pb_front.png"
                ))
            } else {
                HipStr::borrowed("")
            },
            background_phone_path: HipStr::from(format!(
                "world/features/collectibles/portraitborders/{id:04}_{name}/pb_back_phone.png"
            )),
            foreground_phone_path: if self.foreground_phone_path.is_some() {
                HipStr::from(format!(
                    "world/features/collectibles/portraitborders/{id:04}_{name}/pb_front_phone.png"
                ))
            } else {
                HipStr::borrowed("")
            },
            original_lock_status: self.lock_status.into(),
            visibility: self.visibility.into(),
        }
    }
}
