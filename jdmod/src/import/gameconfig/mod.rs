//! # Gameconfig
//! Main code for importing anything in the enginedata/gameconfig folder
use anyhow::{anyhow, Error};
use ubiart_toolkit::{cooked, utils::Game};

use crate::{types::ImportState, utils::cook_path};

mod aliases;
mod avatars;
mod gachacontent;
mod maps_goals;
mod maps_objectives;
mod objectives;
mod offline_recommendation;
mod playlists;
mod portraitborders;
mod scheduled_quests;
mod search_labels;
pub mod songdb;

/// Import anything supported in the enginedata/gameconfig folder
pub fn import(is: &ImportState<'_>) -> Result<(), Error> {
    // Get the gameconfig path
    let gameconfig_path = cook_path(
        &is.aliases
            .get_path_for_alias("gameconfig")
            .ok_or_else(|| anyhow!("common.alias8 does not contain gameconfig path!"))?,
        is.platform,
    )?;

    match is.game {
        Game::JustDance2017 => import_v17(is, &gameconfig_path)?,
        Game::JustDance2018 => import_v18(is, &gameconfig_path)?,
        Game::JustDance2019 => import_v19(is, &gameconfig_path)?,
        Game::JustDance2020 => import_v20(is, &gameconfig_path)?,
        Game::JustDanceChina => import_v20c(is, &gameconfig_path)?,
        Game::JustDance2021 => import_v21(is, &gameconfig_path)?,
        Game::JustDance2022 => import_v22(is, &gameconfig_path)?,
        _ => unimplemented!(),
    }

    // TODO: Check if there's anything else to parse

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2022)
fn import_v22(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v22(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse objectives
    objectives::import_v20v22(is, &gameconfig.config_files_path.objectives)?;

    // Parse scheduled quests
    scheduled_quests::import_v20v22(
        is,
        gameconfig.scheduled_quest_setup,
        gameconfig.config_files_path.scheduledquests.as_ref(),
    )?;

    // Parse the search labels
    search_labels::import_v19v22(is, gameconfig.search_labels)?;

    // Parse the maps objectives
    maps_objectives::import_v20v22(is, gameconfig.mapsobjectives)?;

    // Parse the maps goals
    maps_goals::import_v20v22(is, gameconfig.maps_goals)?;

    // Parse the offline recommendations
    offline_recommendation::import_v20v22(is, gameconfig.offline_recommendation)?;

    // Parse the playlists
    playlists::import_v19v22(is, &gameconfig.config_files_path.playlist)?;

    // Parse the aliases
    aliases::import_v20v22(is, &gameconfig.alias_db_path, &gameconfig.aliasesobjectives)?;

    // Parse the avatars
    avatars::import_v18v22(
        is,
        &gameconfig.avatardb_scene,
        Some(&gameconfig.avatarsobjectives),
    )?;

    // Parse the portraitborders
    portraitborders::import_v20v22(is, &gameconfig.config_files_path.portraitborders)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2021)
fn import_v21(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v21(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse objectives
    objectives::import_v20v22(is, &gameconfig.config_files_path.objectives)?;

    // Parse scheduled quests
    scheduled_quests::import_v20v22(
        is,
        gameconfig.scheduled_quest_setup,
        gameconfig.config_files_path.scheduledquests.as_ref(),
    )?;

    // Parse the search labels
    search_labels::import_v19v22(is, gameconfig.search_labels)?;

    // Parse the maps objectives
    maps_objectives::import_v20v22(is, gameconfig.mapsobjectives)?;

    // Parse the maps goals
    maps_goals::import_v20v22(is, gameconfig.maps_goals)?;

    // Parse the offline recommendations
    offline_recommendation::import_v20v22(is, gameconfig.offline_recommendation)?;

    // Parse the playlists
    playlists::import_v19v22(is, &gameconfig.config_files_path.playlist)?;

    // Parse the aliases
    aliases::import_v20v22(is, &gameconfig.alias_db_path, &gameconfig.aliasesobjectives)?;

    // Parse the avatars
    avatars::import_v18v22(
        is,
        &gameconfig.avatardb_scene,
        Some(&gameconfig.avatarsobjectives),
    )?;

    // Parse the portraitborders
    portraitborders::import_v20v22(is, &gameconfig.config_files_path.portraitborders)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2020)
fn import_v20(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v20(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse objectives
    objectives::import_v20v22(is, &gameconfig.config_files_path.objectives)?;

    // Parse scheduled quests
    scheduled_quests::import_v20v22(
        is,
        gameconfig.scheduled_quest_setup,
        gameconfig.config_files_path.scheduledquests.as_ref(),
    )?;

    // Parse the search labels
    search_labels::import_v19v22(is, gameconfig.search_labels)?;

    // Parse the maps objectives
    maps_objectives::import_v20v22(is, gameconfig.mapsobjectives)?;

    // Parse the maps goals
    maps_goals::import_v20v22(is, gameconfig.maps_goals)?;

    // Parse the offline recommendations
    offline_recommendation::import_v20v22(is, gameconfig.offline_recommendation)?;

    // Parse the playlists
    playlists::import_v19v22(is, &gameconfig.config_files_path.playlist)?;

    // Parse the aliases
    aliases::import_v20v22(is, &gameconfig.alias_db_path, &gameconfig.aliasesobjectives)?;

    // Parse the avatars
    avatars::import_v18v22(
        is,
        &gameconfig.avatardb_scene,
        Some(&gameconfig.avatarsobjectives),
    )?;

    // Parse the portraitborders
    portraitborders::import_v20v22(is, &gameconfig.config_files_path.portraitborders)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2020 China)
fn import_v20c(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v20c(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse objectives
    objectives::import_v20v22(is, &gameconfig.config_files_path.objectives)?;

    // Parse scheduled quests
    scheduled_quests::import_v20v22(
        is,
        gameconfig.scheduled_quest_setup,
        gameconfig.config_files_path.scheduledquests.as_ref(),
    )?;

    // Parse the search labels
    search_labels::import_v19v22(is, gameconfig.search_labels)?;

    // Parse the maps objectives
    maps_objectives::import_v20v22(is, gameconfig.mapsobjectives)?;

    // Parse the maps goals
    maps_goals::import_v20v22(is, gameconfig.maps_goals)?;

    // Parse the offline recommendations
    offline_recommendation::import_v20v22(is, gameconfig.offline_recommendation)?;

    // Parse the playlists
    playlists::import_v19v22(is, &gameconfig.config_files_path.playlist)?;

    // Parse the aliases
    aliases::import_v20v22(is, &gameconfig.alias_db_path, &gameconfig.aliasesobjectives)?;

    // Parse the avatars
    avatars::import_v18v22(
        is,
        &gameconfig.avatardb_scene,
        Some(&gameconfig.avatarsobjectives),
    )?;

    // Parse the portraitborders
    portraitborders::import_v20v22(is, &gameconfig.config_files_path.portraitborders)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2019)
fn import_v19(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v19(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse scheduled quests
    scheduled_quests::import_v18v19(is, gameconfig.scheduled_quests)?;

    // Parse the search labels
    search_labels::import_v19v22(is, gameconfig.search_labels)?;

    // Parse the playlists
    playlists::import_v19v22(is, &gameconfig.config_files_path.playlist)?;

    // Parse the aliases
    aliases::import_v19(is, &gameconfig.alias_db_path)?;

    // Parse the avatars
    avatars::import_v18v22(is, &gameconfig.avatardb_scene, None)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2018)
fn import_v18(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v18(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse scheduled quests
    scheduled_quests::import_v18v19(is, gameconfig.scheduled_quests)?;

    // Parse the avatars
    avatars::import_v18v22(is, &gameconfig.avatardb_scene, None)?;

    // Parse the gachacontent
    gachacontent::import_v18v22(is, &gameconfig.gachaconfig)?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}

/// Import anything supported in the enginedata/gameconfig folder (Just Dance 2017)
fn import_v17(is: &ImportState<'_>, gameconfig_path: &str) -> Result<(), Error> {
    let gameconfig_file = is.vfs.open(gameconfig_path.as_ref())?;
    let parsed_json = cooked::json::parse_v17(&gameconfig_file, is.lax)?;
    let gameconfig = parsed_json.game_manager_config()?;

    // Parse the songdb
    songdb::import(is, &gameconfig.songdb_scene)?;

    Ok(())
}
