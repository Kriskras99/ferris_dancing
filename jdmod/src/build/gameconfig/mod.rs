//! # Gameconfig Building
//! Build all gameconfig related components
use std::collections::HashMap;

use anyhow::{anyhow, Error};
use ubiart_toolkit::{alias8, cooked, json_types};

use crate::utils::cook_path;

use super::{BuildFiles, BuildState};

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

/// Build all gameconfig related components
pub fn build(bs: &BuildState<'_>, bf: &mut BuildFiles) -> Result<(), Error> {
    println!("Building gameconfig...");
    // Load the alias8 file
    let aliases_file = bs
        .patched_base_vfs
        .open("enginedata/common.alias8".as_ref())?;
    let aliases = alias8::parse(&aliases_file)?;

    // Load the gameconfig
    let gameconfig_path = cook_path(
        &aliases
            .get_path_for_alias("gameconfig")
            .ok_or_else(|| anyhow!("gameconfig path not found in common.alias8!"))?,
        bs.platform,
    )?;
    let gameconfig_file = bs.patched_base_vfs.open(gameconfig_path.as_ref())?;
    let gameconfig_template = cooked::json::parse_v22(&gameconfig_file, false)?;
    let mut gameconfig = Box::new(gameconfig_template.game_manager_config()?.clone());

    scheduled_quests::build(bs, bf, &mut gameconfig)?;
    objectives::build(bs, bf)?;
    search_labels::build(bs, &mut gameconfig)?;
    maps_objectives::build(bs, &mut gameconfig)?;
    maps_goals::build(bs, &mut gameconfig)?;
    offline_recommendation::build(bs, &mut gameconfig)?;
    playlists::build(bs, bf, &gameconfig)?;

    let mut gacha_items = Vec::new();
    aliases::build(bs, bf, &mut gameconfig, &mut gacha_items)?;
    avatars::build(bs, bf, &mut gameconfig, &mut gacha_items)?;
    portraitborders::build(bs, bf, &gameconfig, &mut gacha_items)?;

    gachacontent::build(bs, bf, &mut gameconfig, gacha_items)?;

    gameconfig.redeem_maps = HashMap::new();

    let gameconfig_vec =
        cooked::json::create_vec(&json_types::v22::Template22::GameManagerConfig(gameconfig))?;
    bf.generated_files.add_file(gameconfig_path, gameconfig_vec);

    Ok(())
}
