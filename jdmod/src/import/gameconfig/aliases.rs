//! # Aliases
//! Import all the aliases
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use hipstr::HipStr;
use ubiart_toolkit::{
    cooked,
    cooked::{
        isg::AliasesObjectives,
        json::{DifficultyColors, LocalAliasesV19, LocalAliasesV2022},
    },
};

use crate::{
    import::gameconfig::objectives::{load_objectives, save_objectives},
    types::{
        gameconfig::aliases::{Alias, Aliases},
        ImportState,
    },
    utils::cook_path,
};

/// Import all the aliases (Just Dance 2020-2022)
pub fn import_v20v22(
    is: &ImportState<'_>,
    alias_db_path: &str,
    aliasesobjectives: &AliasesObjectives,
) -> Result<(), Error> {
    println!("Importing aliases...");

    let local_aliases = is.vfs.open(cook_path(alias_db_path, is.ugi)?.as_ref())?;
    let local_aliases = cooked::json::parse::<LocalAliasesV2022>(&local_aliases, is.lax)?;

    let mut aliases = load_aliases(
        is,
        &local_aliases.locked_color,
        &local_aliases.difficulty_colors,
    )?;

    aliases.aliases.sort_by_key(|a| a.name_placeholder.clone());

    for alias in local_aliases.aliases {
        if let Err(index) = aliases
            .aliases
            .binary_search_by_key(&alias.string_placeholder, |a| a.name_placeholder.clone())
        {
            aliases.aliases.insert(
                index,
                Alias::from_unlockable_alias_descriptor(
                    alias,
                    aliasesobjectives,
                    &is.locale_id_map,
                ),
            );
        }
    }

    save_aliases(is, &aliases)?;

    Ok(())
}

/// Import all the aliases (Just Dance 2019)
pub fn import_v19(is: &ImportState<'_>, alias_db_path: &str) -> Result<(), Error> {
    println!("Importing aliases...");

    let local_aliases = is.vfs.open(cook_path(alias_db_path, is.ugi)?.as_ref())?;
    let local_aliases = cooked::json::parse::<LocalAliasesV19>(&local_aliases, is.lax)?;

    let mut aliases = load_aliases(
        is,
        &local_aliases.locked_color,
        &local_aliases.difficulty_colors,
    )?;

    aliases.aliases.sort_by_key(|a| a.name_placeholder.clone());

    let mut objectives = load_objectives(is)?;

    for (_, alias) in local_aliases.aliases {
        if let Err(index) = aliases
            .aliases
            .binary_search_by_key(&alias.string_loc_id, |a| a.name)
        {
            aliases.aliases.insert(
                index,
                Alias::from_unlockable_alias_descriptor_19(
                    alias,
                    &is.locale_id_map,
                    &mut objectives,
                ),
            );
        }
    }

    save_objectives(is, &objectives)?;
    save_aliases(is, &aliases)?;

    Ok(())
}

/// Load existing aliases in the mod
fn load_aliases<'a>(
    is: &ImportState<'_>,
    locked_color: &HipStr<'a>,
    difficulty_colors: &DifficultyColors<'a>,
) -> Result<Aliases<'a>, Error> {
    let aliases_config_path = is.dirs.config().join("aliases.json");

    if let Ok(file) = std::fs::read(aliases_config_path) {
        Ok(serde_json::from_slice::<Aliases>(&file)?.into_owned())
    } else {
        let mut rarity_color = HashMap::new();
        for (diff, color) in difficulty_colors {
            rarity_color.insert((*diff).into(), color.clone());
        }

        Ok(Aliases {
            locked_color: locked_color.clone(),
            rarity_color,
            aliases: Vec::new(),
        })
    }
}

/// Save all aliases to the mod folder
///
/// # Errors
/// Will error if the IO fails
fn save_aliases(is: &ImportState<'_>, aliases: &Aliases) -> std::io::Result<()> {
    let aliases_config_path = is.dirs.config().join("aliases.json");

    let file = File::create(aliases_config_path)?;
    serde_json::to_writer_pretty(file, &aliases)?;

    Ok(())
}
