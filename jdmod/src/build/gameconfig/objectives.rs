//! # Objectives Building
//! Build the objectives
use std::collections::HashMap;

use anyhow::Error;
use dotstar_toolkit_utils::vfs::VirtualFileSystem;
use hipstr::HipStr;
use ubiart_toolkit::{cooked, utils::UniqueGameId};

use crate::{
    build::{BuildFiles, BuildState},
    types::gameconfig::objectives::Objective,
    utils::cook_path,
};

/// Build the objectives
pub fn build(bs: &BuildState, bf: &mut BuildFiles) -> Result<(), Error> {
    let name_map_file = bs
        .native_vfs
        .open(&bs.rel_tree.config().join("objectives.json"))?;
    let name_map: HashMap<HipStr<'_>, Objective> = serde_json::from_slice(&name_map_file)?;
    let objective_descs = name_map
        .into_iter()
        .map(|(name, descriptor)| (name, descriptor.into()))
        .collect();

    let objective_database = cooked::isg::ObjectivesDatabase {
        class: Some(cooked::isg::ObjectivesDatabase::CLASS),
        objective_descs,
    };

    let objective_database_vec = cooked::json::create_vec(&objective_database)?;
    bf.generated_files.add_file(
        cook_path("enginedata/gameconfig/objectives.isg", UniqueGameId::NX2022)?.into(),
        objective_database_vec,
    )?;

    Ok(())
}
