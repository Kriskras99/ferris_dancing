//! # Aliases
//! Types for dealing with aliases
use std::collections::HashMap;

use hipstr::HipStr;
use ownable::IntoOwned;
use serde::{Deserialize, Serialize};
use ubiart_toolkit::{
    cooked::{
        isg::AliasesObjectives,
        json::{UnlockableAliasDescriptor19, UnlockableAliasDescriptor2022},
    },
    utils::LocaleId,
};

use super::{
    generate_gacha_id,
    objectives::{Objective, Objectives},
};
use crate::types::localisation::LocaleIdMap;

/// Describes an alias
#[derive(Debug, Clone, Serialize, Deserialize, IntoOwned)]
pub struct Aliases<'a> {
    /// Color when locked (RGBA as hex string)
    #[serde(borrow)]
    pub locked_color: HipStr<'a>,
    /// Map rarity to colors (RGBA as hex string)
    #[serde(borrow)]
    pub rarity_color: HashMap<Rarity, HipStr<'a>>,
    /// All the aliases
    #[serde(borrow)]
    pub aliases: Vec<Alias<'a>>,
}

/// How rare is the alias
///
/// Wrapper type around [`ubiart_toolkit::json_types::isg::Rarity`] that serializes in a more readable way
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, IntoOwned)]
pub enum Rarity {
    /// Common
    Common,
    /// Uncommon
    Uncommon,
    /// Rare
    Rare,
    /// Epic
    Epic,
    /// Legendary
    Legendary,
    /// Exotic
    Exotic,
}

impl From<Rarity> for ubiart_toolkit::cooked::json::Rarity {
    fn from(value: Rarity) -> Self {
        match value {
            Rarity::Common => Self::Common,
            Rarity::Uncommon => Self::Uncommon,
            Rarity::Rare => Self::Rare,
            Rarity::Epic => Self::Epic,
            Rarity::Legendary => Self::Legendary,
            Rarity::Exotic => Self::Exotic,
        }
    }
}

impl From<ubiart_toolkit::cooked::json::Rarity> for Rarity {
    fn from(value: ubiart_toolkit::cooked::json::Rarity) -> Self {
        match value {
            ubiart_toolkit::cooked::json::Rarity::Common => Self::Common,
            ubiart_toolkit::cooked::json::Rarity::Uncommon => Self::Uncommon,
            ubiart_toolkit::cooked::json::Rarity::Rare => Self::Rare,
            ubiart_toolkit::cooked::json::Rarity::Epic => Self::Epic,
            ubiart_toolkit::cooked::json::Rarity::Legendary => Self::Legendary,
            ubiart_toolkit::cooked::json::Rarity::Exotic => Self::Exotic,
        }
    }
}

/// Describes an alias
#[derive(Debug, Clone, Serialize, Deserialize, IntoOwned)]
pub struct Alias<'a> {
    /// Placeholder for the name (mostly empty)
    #[serde(borrow)]
    pub name_placeholder: HipStr<'a>,
    /// Name (ungendered or male)
    pub name: LocaleId,
    /// Name (female)
    pub name_female: LocaleId,
    /// Description
    pub description: LocaleId,
    /// Is it unlocked by default
    pub unlocked_by_default: bool,
    /// How rare is it
    pub rarity: Rarity,
    /// What needs to be done to unlock it (objective name)
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub unlock_objective: Option<HipStr<'a>>,
}

impl<'c> Alias<'c> {
    /// Convert from UbiArt representation
    #[must_use]
    pub fn from_unlockable_alias_descriptor<'a: 'c, 'b: 'c>(
        descriptor: UnlockableAliasDescriptor2022<'a>,
        aliasesobjectives: &HashMap<u32, HipStr<'b>>,
        locale_id_map: &LocaleIdMap,
    ) -> Self {
        let unlock_objective = aliasesobjectives.get(&descriptor.id).cloned();

        Self {
            name_placeholder: descriptor.string_placeholder,
            name: locale_id_map
                .get(descriptor.string_loc_id)
                .unwrap_or_default(),
            name_female: locale_id_map
                .get(descriptor.string_loc_id_female)
                .unwrap_or_default(),
            description: locale_id_map
                .get(descriptor.description_loc_id)
                .unwrap_or_default(),
            unlocked_by_default: descriptor.unlocked_by_default,
            rarity: descriptor.difficulty_color.into(),
            unlock_objective,
        }
    }

    /// Convert from old UbiArt representation
    pub fn from_unlockable_alias_descriptor_19<'a: 'c>(
        descriptor: UnlockableAliasDescriptor19<'a>,
        locale_id_map: &LocaleIdMap,
        objectives: &mut Objectives<'a>,
    ) -> Self {
        let unlock_objective = Some(HipStr::from(objectives.add_objective(
            Objective::from_old_descriptor(
                &descriptor.unlock_objective,
                descriptor.restricted_to_unlimited_songs,
                locale_id_map,
            ),
        )));
        let name = locale_id_map
            .get(descriptor.string_loc_id)
            .unwrap_or_default();
        Self {
            name_placeholder: descriptor.string_placeholder,
            name_female: name,
            name,
            description: LocaleId::EMPTY,
            unlocked_by_default: false,
            rarity: descriptor.difficulty_color.into(),
            unlock_objective,
        }
    }

    /// Convert to the UbiArt representation
    pub fn into_unlockable_alias_descriptor(
        self,
        aliasesobjectives: &mut AliasesObjectives<'c>,
    ) -> UnlockableAliasDescriptor2022<'c> {
        let id = generate_gacha_id();
        if let Some(unlock_objective) = self.unlock_objective {
            aliasesobjectives.insert(id, unlock_objective);
        } else {
            aliasesobjectives.remove(&id);
        }
        UnlockableAliasDescriptor2022 {
            id,
            string_loc_id: self.name,
            string_loc_id_female: self.name_female,
            string_placeholder: self.name_placeholder,
            unlocked_by_default: self.unlocked_by_default,
            description_loc_id: self.description,
            difficulty_color: self.rarity.into(),
            ..Default::default()
        }
    }
}
