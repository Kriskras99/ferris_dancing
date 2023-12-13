//! # Aliases
//! Types for dealing with aliases
use std::{borrow::Cow, collections::HashMap};

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};
use ubiart_toolkit::{
    json_types::{
        AliasesObjectives, UnlockObjectiveOnlineInfo, UnlockableAliasDescriptor,
        UnlockableAliasDescriptor1719,
    },
    utils::LocaleId,
};

use super::{
    generate_gacha_id,
    objectives::{Objective, Objectives},
};
use crate::types::localisation::LocaleIdMap;

/// Describes an alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aliases<'a> {
    /// Color when locked (RGBA as hex string)
    pub locked_color: Cow<'a, str>,
    /// Map rarity to colors (RGBA as hex string)
    pub rarity_color: HashMap<Rarity, Cow<'a, str>>,
    /// All the aliases
    pub aliases: Vec<Alias<'a>>,
}

/// How rare is the alias
#[repr(u8)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Rarity {
    /// Common
    Common = 0,
    /// Uncommon
    Uncommon = 1,
    /// Rare
    Rare = 2,
    /// Epic
    Epic = 3,
    /// Legendary
    Legendary = 4,
    /// Exotic
    Exotic = 5,
}

impl From<Rarity> for u8 {
    #[allow(clippy::as_conversions)]
    fn from(value: Rarity) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Rarity {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Common),
            1 => Ok(Self::Uncommon),
            2 => Ok(Self::Rare),
            3 => Ok(Self::Epic),
            4 => Ok(Self::Legendary),
            5 => Ok(Self::Exotic),
            _ => Err(anyhow!("Unknown DifficultyColor {value}")),
        }
    }
}

/// Describes an alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alias<'a> {
    /// Placeholder for the name (mostly empty)
    pub name_placeholder: Cow<'a, str>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unlock_objective: Option<Cow<'a, str>>,
}

impl<'c> Alias<'c> {
    /// Convert from UbiArt representation
    pub fn from_unlockable_alias_descriptor<'a: 'c, 'b: 'c>(
        descriptor: UnlockableAliasDescriptor<'a>,
        aliasesobjectives: &HashMap<u16, Cow<'b, str>>,
        locale_id_map: &LocaleIdMap,
    ) -> Result<Self, Error> {
        let unlock_objective = aliasesobjectives.get(&descriptor.id).cloned();

        Ok(Self {
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
            rarity: descriptor.difficulty_color.try_into()?,
            unlock_objective,
        })
    }

    /// Convert from old UbiArt representation
    pub fn from_unlockable_alias_descriptor_1719<'a: 'c>(
        descriptor: UnlockableAliasDescriptor1719<'a>,
        locale_id_map: &LocaleIdMap,
        objectives: &mut Objectives<'a>,
    ) -> Result<Self, Error> {
        let unlock_objective = Some(Cow::Owned(objectives.add_objective(
            Objective::from_old_descriptor(
                &descriptor.unlock_objective,
                descriptor.restricted_to_unlimited_songs,
                locale_id_map,
            ),
        )?));
        let name = locale_id_map
            .get(descriptor.string_loc_id)
            .unwrap_or_default();
        Ok(Self {
            name_placeholder: descriptor.string_placeholder,
            name_female: name,
            name,
            description: LocaleId::EMPTY,
            unlocked_by_default: false,
            rarity: descriptor.difficulty_color.try_into()?,
            unlock_objective,
        })
    }

    /// Convert to the UbiArt representation
    pub fn to_unlockable_alias_descriptor(
        self,
        aliasesobjectives: &mut AliasesObjectives<'c>,
    ) -> UnlockableAliasDescriptor<'c> {
        let id = generate_gacha_id();
        if let Some(unlock_objective) = self.unlock_objective {
            aliasesobjectives.insert(id, unlock_objective);
        } else {
            aliasesobjectives.remove(&id);
        }
        UnlockableAliasDescriptor {
            class: Some(UnlockableAliasDescriptor::CLASS),
            id,
            string_loc_id: self.name,
            string_loc_id_female: self.name_female,
            string_online_localized: Cow::Borrowed(""),
            string_online_localized_female: Cow::Borrowed(""),
            string_placeholder: self.name_placeholder,
            unlocked_by_default: self.unlocked_by_default,
            description_loc_id: self.description,
            description_localized: Cow::Borrowed(""),
            unlock_objective: Some(UnlockObjectiveOnlineInfo::default()),
            difficulty_color: self.rarity.into(),
            visibility: 0,
        }
    }
}
