//! # Gacha machine
//! Types for dealing with the gacha machine
use serde::{Deserialize, Serialize};
use ubiart_toolkit::cooked::isg::{
    CollectibleGachaItem, CollectibleGachaItemAlias, CollectibleGachaItemAvatar,
    CollectibleGachaItemPortraitBorder,
};

/// An item in the gacha machine
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum GachaItem {
    /// Alias
    Alias(u32),
    /// Portraitborder/skin
    PortraitBorder(u32),
    /// Avatar
    Avatar(u32),
}

impl From<GachaItem> for CollectibleGachaItem<'_> {
    fn from(value: GachaItem) -> Self {
        match value {
            GachaItem::Alias(alias_id) => Self::Alias(CollectibleGachaItemAlias {
                class: None,
                alias_id,
            }),
            GachaItem::PortraitBorder(portrait_id) => {
                Self::PortraitBorder(CollectibleGachaItemPortraitBorder {
                    class: None,
                    portrait_id,
                })
            }
            GachaItem::Avatar(avatar_id) => Self::Avatar(CollectibleGachaItemAvatar {
                class: None,
                avatar_id,
            }),
        }
    }
}

/// Configuration of the gacha machine
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct GachaConfig {
    /// Price for one roll
    pub price: u32,
    /// Force a high rarity every X rolls
    pub force_high_rarity_reward_count: u32,
    /// Push the gacha screen between A and B maps played
    pub numbers_of_maps_before_push_gacha_screen: (u32, u32),
}
